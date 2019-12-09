#[derive(Debug)]
enum Mode {
    Position,
    Immediate,
    Relative
}

#[derive(Debug)]
enum Type {
    Parameter,
    Address
}

#[derive(Debug)]
enum Instruction {
    Add(i64, i64, usize),
    Multiply(i64, i64, usize),
    Input(usize),
    Output(i64),
    JumpIfTrue(i64, i64),
    JumpIfFalse(i64, i64),
    LessThan(i64, i64, usize),
    Equals(i64, i64, usize),
    RelativeBaseOffset(i64),
    Halt
}

struct IntcodeComputer {
    memory: Vec<i64>,
    pointer: usize,
    inputs: Vec<i64>,
    input_count: usize,
    last_result: Option<i64>,
    relative_base: i64
}

fn parse_program(program: &'static str) -> Vec<i64> {
    program.split(',').map(|n| n.parse::<i64>().unwrap()).collect::<Vec<i64>>()
}

impl IntcodeComputer {
    fn new(program: &Vec<i64>) -> IntcodeComputer {
        IntcodeComputer {
            memory: program.clone(),
            pointer: 0,
            inputs: vec!(),
            input_count : 0,
            last_result: None,
            relative_base: 0
        }
    }

    fn check_for_resize(&mut self, addr: usize) {
        if addr >= self.memory.len() {
            self.memory.resize(addr+1, 0);
        }
    }

    fn read(&mut self, addr: usize) -> i64 {
        self.check_for_resize(addr);
        self.memory[addr]
    }

    fn write(&mut self, addr: usize, val: i64) {
        self.check_for_resize(addr);
        self.memory[addr] = val;
    }

    fn read_next_parameter(&mut self, param_type: Type, param_mode: Mode) -> i64 {
        let address = match param_mode {
            Mode::Position => self.read(self.pointer) as usize,
            Mode::Immediate => self.pointer,
            Mode::Relative => (self.relative_base + self.read(self.pointer)) as usize
        };

        let read_param = match param_type {
            Type::Parameter => self.read(address),
            Type::Address => address as i64
        };

        self.pointer += 1;

        return read_param;
    }

    fn split_instruction(&mut self) -> Vec<u8> {
        let instr = self.memory[self.pointer];
        self.pointer += 1;

        if instr == 0 {
            panic!("Invalid instruction value");
        }
    
        let mut val = instr;
        std::iter::from_fn(move || {
            if val == 0 {
                None
            } else {
                let next_val = val % 10;
                val /= 10;
                Some(next_val as u8)
            }
        })
        .collect()
    }

    fn next_input(&mut self) -> i64 {
        if self.inputs.len() <= self.input_count {
            panic!("Not enough inputs provided!");
        }

        let input = self.inputs[self.input_count];
        self.input_count += 1;
        input
    }

    fn next_instruction(&mut self) -> Instruction {
        let instruction = self.split_instruction();

        //check for halt
        if instruction.len() > 1 && instruction[0] == 9 && instruction[1] == 9 {
            return Instruction::Halt;
        }

        let opcode = instruction[0];

        let mut param_stack : Vec<Mode> = if instruction.len() > 2 { 
            instruction[2..].iter().rev().map(|p| {
                match p {
                    0 => Mode::Position,
                    1 => Mode::Immediate,
                    2 => Mode::Relative,
                    _ => panic!("Unknown parameter mode.")
                }
            }).collect() 
        } else { vec!() };

        match opcode {
            1 | 2 | 7 | 8 => {
                let p1 = self.read_next_parameter(Type::Parameter, param_stack.pop().unwrap_or(Mode::Position));
                let p2 = self.read_next_parameter(Type::Parameter, param_stack.pop().unwrap_or(Mode::Position));
                let addr = self.read_next_parameter(Type::Address, param_stack.pop().unwrap_or(Mode::Position)) as usize;

                if opcode == 1 {
                    Instruction::Add(p1, p2, addr)
                } else if opcode == 2 {
                    Instruction::Multiply(p1, p2, addr)
                } else if opcode == 7 {
                    Instruction::LessThan(p1, p2, addr)
                } else {
                    Instruction::Equals(p1, p2, addr)
                }
            },
            3 => Instruction::Input(self.read_next_parameter(Type::Address, param_stack.pop().unwrap_or(Mode::Position)) as usize),
            4 => Instruction::Output(self.read_next_parameter(Type::Parameter, param_stack.pop().unwrap_or(Mode::Position))),
            5 | 6 => {
                let p1 = self.read_next_parameter(Type::Parameter, param_stack.pop().unwrap_or(Mode::Position));
                let p2 = self.read_next_parameter(Type::Parameter, param_stack.pop().unwrap_or(Mode::Position));
                if opcode == 5 {
                    Instruction::JumpIfTrue(p1, p2)
                } else {
                    Instruction::JumpIfFalse(p1, p2)
                }
            },
            9 => Instruction::RelativeBaseOffset(self.read_next_parameter(Type::Parameter, param_stack.pop().unwrap_or(Mode::Position))),
            _ => panic!("Unknown instruction opcode")
        }
    }

    fn run(&mut self, input_vals: Vec<i64>) -> Option<i64> {
        self.inputs = input_vals;
        self.input_count = 0;

        loop {
            match self.next_instruction() {
                Instruction::Add(p1, p2, addr) => self.write(addr, p1+p2),
                Instruction::Multiply(p1, p2, addr) => self.write(addr, p1*p2),
                Instruction::Input(addr) => {
                    let input = self.next_input();
                    self.write(addr, input);
                },
                Instruction::Output(p) => {
                    self.last_result = Some(p);
                    break;
                },
                Instruction::JumpIfTrue(p1, p2) => self.pointer = if p1 != 0 { p2 as usize } else { self.pointer },
                Instruction::JumpIfFalse(p1, p2) => self.pointer = if p1 == 0 { p2 as usize } else { self.pointer },
                Instruction::LessThan(p1, p2, addr) => self.write(addr, if p1 < p2 { 1 } else { 0 }),
                Instruction::Equals(p1, p2, addr) => self.write(addr, if p1 == p2 { 1 } else { 0 }),
                Instruction::RelativeBaseOffset(p1) => self.relative_base += p1,
                Instruction::Halt => {
                    self.last_result = None;
                    break;
                }
            };
        }

        self.last_result
    }
}

fn main() {
    let program = parse_program(include_str!("../input/day_9.txt"));
    let mut boost_comp = IntcodeComputer::new(&program);
    let mut output = boost_comp.run(vec!(1));
    let mut last_out_val = 0;

    while output.is_some() {
        last_out_val = output.unwrap();
        output = boost_comp.run(vec!());
    }

    println!("Part 1 => {}", last_out_val);
    println!("Part 2 => {}", IntcodeComputer::new(&program).run(vec!(2)).unwrap());
}

#[test]
fn quine_test() {
    let prog = parse_program("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
    let mut comp = IntcodeComputer::new(&prog);
    let mut output = vec!();

    loop {
        let next_output = comp.run(vec!());
        if next_output.is_none() {
            break;
        }
        output.push(next_output.unwrap());
    }

    assert_eq!(output, prog);
}

#[test]
fn sixteen_digit_output() {
    let prog = parse_program("1102,34915192,34915192,7,4,7,99,0");
    let output = IntcodeComputer::new(&prog).run(vec!()).unwrap();
    assert_eq!(output.to_string().len(), 16);
}

#[test]
fn large_number() {
    let prog = parse_program("104,1125899906842624,99");
    let output = IntcodeComputer::new(&prog).run(vec!()).unwrap();
    assert_eq!(output, 1125899906842624);
}