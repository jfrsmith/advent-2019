#[derive(Debug)]
enum Mode {
    Position,
    Immediate
}

#[derive(Debug)]
enum Instruction {
    Add(i32, i32, usize),
    Multiply(i32, i32, usize),
    Input(usize),
    Output(i32),
    JumpIfTrue(i32, i32),
    JumpIfFalse(i32, i32),
    LessThan(i32, i32, usize),
    Equals(i32, i32, usize),
    Halt
}

struct IntcodeComputer {
    memory: Vec<i32>,
    pointer: usize,
    output: Option<i32>
}

fn read_input() -> Option<i32> {
    use std::io::Write;

    print!("Enter input value: ");
    let _ = std::io::stdout().flush();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().ok()
}

fn read_parameter_mode(param_num: usize, param_modes: &Vec<u8>) -> Mode {
    if param_num < param_modes.len() && param_modes[param_num] == 1 {
        Mode::Immediate
    } else {
        Mode::Position
    }
}

impl IntcodeComputer {
    fn new(input: &'static str) -> IntcodeComputer {
        IntcodeComputer {
            memory: input.split(',').map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>(),
            pointer: 0,
            output: None
        }
    }

    fn get_next_param_val(&mut self, param_mode : Mode) -> i32 {
        let value = match param_mode {
            Mode::Position => self.memory[self.memory[self.pointer] as usize],
            Mode::Immediate => self.memory[self.pointer]
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

    fn next_instruction(&mut self) -> Instruction {
        let instruction = self.split_instruction();
        
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
            9 => Instruction::Halt,
            _ => panic!("Unknown instruction opcode")
        }
    }

    fn run(&mut self, input: Option<i32>) -> Option<i32> {
        loop {
            match self.next_instruction() {
                Instruction::Add(p1, p2, addr) => self.memory[addr] = p1 + p2,
                Instruction::Multiply(p1, p2, addr) => self.memory[addr] = p1 * p2,
                Instruction::Input(addr) => self.memory[addr] = input.or_else(|| read_input()).unwrap(),
                Instruction::Output(p) => {
                    self.output = Some(p); 
                    println!("Output => {}", p);
                },
                Instruction::JumpIfTrue(p1, p2) => self.pointer = if p1 != 0 { p2 as usize } else { self.pointer },
                Instruction::JumpIfFalse(p1, p2) => self.pointer = if p1 == 0 { p2 as usize } else { self.pointer },
                Instruction::LessThan(p1, p2, addr) => self.memory[addr] = if p1 < p2 { 1 } else { 0 },
                Instruction::Equals(p1, p2, addr) => self.memory[addr] = if p1 == p2 { 1 } else { 0 },
                Instruction::Halt => break
            };
        }

        self.output
    }
}

fn main() {
    IntcodeComputer::new(include_str!("../input/day_5.txt")).run(None);
}

#[test]
fn input_output() {
    assert_eq!(IntcodeComputer::new("3,0,4,0,99").run(Some(1)).unwrap(), 1);
}

#[test]
fn part_1_complete() {
    assert_eq!(IntcodeComputer::new(include_str!("../input/day_5.txt")).run(Some(1)).unwrap(), 4887191);
}

#[test]
fn equal_to() {
    assert_eq!(IntcodeComputer::new("3,9,8,9,10,9,4,9,99,-1,8").run(Some(8)).unwrap(), 1);
    assert_eq!(IntcodeComputer::new("3,9,8,9,10,9,4,9,99,-1,8").run(Some(10)).unwrap(), 0);

    assert_eq!(IntcodeComputer::new("3,3,1108,-1,8,3,4,3,99").run(Some(8)).unwrap(), 1);
    assert_eq!(IntcodeComputer::new("3,3,1108,-1,8,3,4,3,99").run(Some(10)).unwrap(), 0);
}

#[test]
fn less_than() {
    assert_eq!(IntcodeComputer::new("3,9,7,9,10,9,4,9,99,-1,8").run(Some(7)).unwrap(), 1);
    assert_eq!(IntcodeComputer::new("3,9,8,9,10,9,4,9,99,-1,8").run(Some(9)).unwrap(), 0);

    assert_eq!(IntcodeComputer::new("3,3,1107,-1,8,3,4,3,99").run(Some(7)).unwrap(), 1);
    assert_eq!(IntcodeComputer::new("3,3,1107,-1,8,3,4,3,99").run(Some(9)).unwrap(), 0);
}

#[test]
fn compare_to_zero() {
    assert_eq!(IntcodeComputer::new("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9").run(Some(0)).unwrap(), 0);
    assert_eq!(IntcodeComputer::new("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9").run(Some(-1)).unwrap(), 1);

    assert_eq!(IntcodeComputer::new("3,3,1105,-1,9,1101,0,0,12,4,12,99,1").run(Some(0)).unwrap(), 0);
    assert_eq!(IntcodeComputer::new("3,3,1105,-1,9,1101,0,0,12,4,12,99,1").run(Some(-1)).unwrap(), 1);
}

#[test]
fn compare_to_val() {
    let program = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
    
    assert_eq!(IntcodeComputer::new(program).run(Some(0)).unwrap(), 999);
    assert_eq!(IntcodeComputer::new(program).run(Some(8)).unwrap(), 1000);
    assert_eq!(IntcodeComputer::new(program).run(Some(20)).unwrap(), 1001);
}

#[test]
fn part_2_complete() {
    assert_eq!(IntcodeComputer::new(include_str!("../input/day_5.txt")).run(Some(5)).unwrap(), 3419022);
}