#[derive(Debug)]
enum Mode {
    Position,
    Immediate,
    Relative
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

fn read_parameter_mode(param_num: usize, param_modes: &Vec<u8>) -> Mode {
    if param_num > param_modes.len() || param_modes[param_num] == 0 {
        Mode::Position
    } else if param_modes[param_num] == 1 {
        Mode::Immediate
    } else {
        Mode::Relative
    }
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

    //TODO: fix up addressing memory

    fn get_next_param_val(&mut self, param_mode : Mode) -> i64 {
        let value = match param_mode {
            Mode::Position => self.memory[self.memory[self.pointer] as usize],
            Mode::Immediate => self.memory[self.pointer],
            Mode::Relative => unimplemented!()
        };

        self.pointer += 1;
        
        return value;
    }

    fn get_next_param_addr(&mut self) -> usize {
        let addr = self.memory[self.pointer] as usize;
        self.pointer += 1;
        return addr;
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

        let param_modes : Vec<u8> = if instruction.len() > 2 { instruction[2..].to_vec() } else { vec!() };

        match opcode {
            1 | 2 | 7 | 8 => {
                let p1 = self.get_next_param_val(read_parameter_mode(0, &param_modes));
                let p2 = self.get_next_param_val(read_parameter_mode(1, &param_modes));
                let addr = self.get_next_param_addr();

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
            3 => Instruction::Input(self.get_next_param_addr()),
            4 => Instruction::Output(self.get_next_param_val(read_parameter_mode(0, &param_modes))),
            5 | 6 => {
                let p1 = self.get_next_param_val(read_parameter_mode(0, &param_modes));
                let p2 = self.get_next_param_val(read_parameter_mode(1, &param_modes));
                if opcode == 5 {
                    Instruction::JumpIfTrue(p1, p2)
                } else {
                    Instruction::JumpIfFalse(p1, p2)
                }
            },
            9 => {
                Instruction::RelativeBaseOffset(self.get_next_param_val(read_parameter_mode(0, &param_modes)))
            },
            _ => panic!("Unknown instruction opcode")
        }
    }

    fn run(&mut self, input_vals: Vec<i64>) -> Option<i64> {
        self.inputs = input_vals;
        self.input_count = 0;

        loop {
            match self.next_instruction() {
                Instruction::Add(p1, p2, addr) => self.memory[addr] = p1 + p2,
                Instruction::Multiply(p1, p2, addr) => self.memory[addr] = p1 * p2,
                Instruction::Input(addr) => self.memory[addr] = self.next_input(),
                Instruction::Output(p) => {
                    self.last_result = Some(p);
                    break;
                },
                Instruction::JumpIfTrue(p1, p2) => self.pointer = if p1 != 0 { p2 as usize } else { self.pointer },
                Instruction::JumpIfFalse(p1, p2) => self.pointer = if p1 == 0 { p2 as usize } else { self.pointer },
                Instruction::LessThan(p1, p2, addr) => self.memory[addr] = if p1 < p2 { 1 } else { 0 },
                Instruction::Equals(p1, p2, addr) => self.memory[addr] = if p1 == p2 { 1 } else { 0 },
                Instruction::RelativeBaseOffset(p1) => self.relative_base += p1,
                Instruction::Halt => {
                    self.last_result = None;
                    break;
                }
            };
        }

        self.last_result
    }

    fn get_last_result(&self) -> Option<i64> {
        self.last_result
    }
}

fn main() {
    let mut boost_comp = IntcodeComputer::new(&parse_program(include_str!("../input/day_9.txt")));
    let mut output = boost_comp.run(vec!(1));
    let mut last_out_val = 0;
    while output.is_some() {
        last_out_val = output.unwrap();
        output = boost_comp.run(vec!());
    }

    println!("Part 1 => {}", last_out_val);
}

#[test]
fn part_1_test() {
    let mut prog = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
    println!("{}", IntcodeComputer::new(prog).run(vec!()));
    assert_eq!(false);
}