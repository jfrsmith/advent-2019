#[derive(Debug)]
enum Mode {
    Position,
    Immediate,
    Relative,
}

#[derive(Debug)]
enum Type {
    Parameter,
    Address,
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
    Halt,
}

pub enum Output {
    OutputVal(i64),
    WaitingForInput,
    Exit
}

pub struct IntcodeComputer {
    memory: Vec<i64>,
    pointer: usize,
    relative_base: i64,
    input_stack: Vec<i64>,
    wait_for_input_addr: Option<usize>,
    has_exited: bool
}

impl IntcodeComputer {
    pub fn new(program: &Vec<i64>) -> IntcodeComputer {
        IntcodeComputer {
            memory: program.clone(),
            pointer: 0,
            relative_base: 0,
            input_stack: vec!(),
            wait_for_input_addr: None,
            has_exited: false
        }
    }

    pub fn finished(&self) -> bool {
        self.has_exited
    }

    pub fn provide_input(&mut self, input: i64) {
        self.write(self.wait_for_input_addr.expect("Tried to write input when it was not expected!"), input);
    }

    fn check_for_resize(&mut self, addr: usize) {
        if addr >= self.memory.len() {
            self.memory.resize(addr + 1, 0);
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
            Mode::Relative => (self.relative_base + self.read(self.pointer)) as usize,
        };

        let read_param = match param_type {
            Type::Parameter => self.read(address),
            Type::Address => address as i64,
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

    fn next_instruction(&mut self) -> Instruction {
        let instruction = self.split_instruction();

        //check for halt
        if instruction.len() > 1 && instruction[0] == 9 && instruction[1] == 9 {
            return Instruction::Halt;
        }

        let opcode = instruction[0];

        let mut param_stack: Vec<Mode> = if instruction.len() > 2 {
            instruction[2..]
                .iter()
                .rev()
                .map(|p| match p {
                    0 => Mode::Position,
                    1 => Mode::Immediate,
                    2 => Mode::Relative,
                    _ => panic!("Unknown parameter mode."),
                })
                .collect()
        } else {
            vec![]
        };

        match opcode {
            1 | 2 | 7 | 8 => {
                let p1 = self.read_next_parameter(
                    Type::Parameter,
                    param_stack.pop().unwrap_or(Mode::Position),
                );
                let p2 = self.read_next_parameter(
                    Type::Parameter,
                    param_stack.pop().unwrap_or(Mode::Position),
                );
                let addr = self
                    .read_next_parameter(Type::Address, param_stack.pop().unwrap_or(Mode::Position))
                    as usize;

                if opcode == 1 {
                    Instruction::Add(p1, p2, addr)
                } else if opcode == 2 {
                    Instruction::Multiply(p1, p2, addr)
                } else if opcode == 7 {
                    Instruction::LessThan(p1, p2, addr)
                } else {
                    Instruction::Equals(p1, p2, addr)
                }
            }
            3 => Instruction::Input(
                self.read_next_parameter(Type::Address, param_stack.pop().unwrap_or(Mode::Position))
                    as usize,
            ),
            4 => {
                Instruction::Output(self.read_next_parameter(
                    Type::Parameter,
                    param_stack.pop().unwrap_or(Mode::Position),
                ))
            }
            5 | 6 => {
                let p1 = self.read_next_parameter(
                    Type::Parameter,
                    param_stack.pop().unwrap_or(Mode::Position),
                );
                let p2 = self.read_next_parameter(
                    Type::Parameter,
                    param_stack.pop().unwrap_or(Mode::Position),
                );
                if opcode == 5 {
                    Instruction::JumpIfTrue(p1, p2)
                } else {
                    Instruction::JumpIfFalse(p1, p2)
                }
            }
            9 => {
                Instruction::RelativeBaseOffset(self.read_next_parameter(
                    Type::Parameter,
                    param_stack.pop().unwrap_or(Mode::Position),
                ))
            }
            _ => panic!("Unknown instruction opcode"),
        }
    }

    fn internal_run(&mut self) -> Output {
        loop {
            match self.next_instruction() {
                Instruction::Add(p1, p2, addr) => self.write(addr, p1 + p2),
                Instruction::Multiply(p1, p2, addr) => self.write(addr, p1 * p2),
                Instruction::Input(addr) => {
                    if let Some(next_input) = self.input_stack.pop() {
                        self.write(addr, next_input);
                    } else {
                        self.wait_for_input_addr = Some(addr);
                        return Output::WaitingForInput;
                    }
                },
                Instruction::Output(p) => return Output::OutputVal(p),
                Instruction::JumpIfTrue(p1, p2) => {
                    self.pointer = if p1 != 0 { p2 as usize } else { self.pointer }
                },
                Instruction::JumpIfFalse(p1, p2) => {
                    self.pointer = if p1 == 0 { p2 as usize } else { self.pointer }
                },
                Instruction::LessThan(p1, p2, addr) => {
                    self.write(addr, if p1 < p2 { 1 } else { 0 })
                },
                Instruction::Equals(p1, p2, addr) => self.write(addr, if p1 == p2 { 1 } else { 0 }),
                Instruction::RelativeBaseOffset(p1) => self.relative_base += p1,
                Instruction::Halt => {
                    break;
                }
            };
        }

        self.has_exited = true;
        Output::Exit
    }

    pub fn run(&mut self, input: Vec<i64>) -> Vec<i64> {
        let mut all_outputs = vec![];
        self.input_stack = input.into_iter().rev().collect::<Vec<i64>>();
        loop {
            match self.internal_run() {
                Output::OutputVal(x) => all_outputs.push(x),
                Output::WaitingForInput => panic!("Unexpected request for input"),
                Output::Exit => break
            }
        }

        all_outputs
    }

    pub fn tick(&mut self) -> Output {
        self.internal_run()
    }
}

pub fn parse_program(program: &'static str) -> Vec<i64> {
    program
        .split(',')
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_output() {
        assert_eq!(
            *IntcodeComputer::new(&parse_program("3,0,4,0,99"))
                .run(vec!(1))
                .last()
                .unwrap(),
            1
        );
    }

    #[test]
    fn equal_to() {
        assert_eq!(
            *IntcodeComputer::new(&parse_program("3,9,8,9,10,9,4,9,99,-1,8"))
                .run(vec!(8))
                .last()
                .unwrap(),
            1
        );
        assert_eq!(
            *IntcodeComputer::new(&parse_program("3,9,8,9,10,9,4,9,99,-1,8"))
                .run(vec!(10))
                .last()
                .unwrap(),
            0
        );
        assert_eq!(
            *IntcodeComputer::new(&parse_program("3,3,1108,-1,8,3,4,3,99"))
                .run(vec!(8))
                .last()
                .unwrap(),
            1
        );
        assert_eq!(
            *IntcodeComputer::new(&parse_program("3,3,1108,-1,8,3,4,3,99"))
                .run(vec!(10))
                .last()
                .unwrap(),
            0
        );
    }

    #[test]
    fn less_than() {
        assert_eq!(
            *IntcodeComputer::new(&parse_program("3,9,7,9,10,9,4,9,99,-1,8"))
                .run(vec!(7))
                .last()
                .unwrap(),
            1
        );
        assert_eq!(
            *IntcodeComputer::new(&parse_program("3,9,8,9,10,9,4,9,99,-1,8"))
                .run(vec!(9))
                .last()
                .unwrap(),
            0
        );
        assert_eq!(
            *IntcodeComputer::new(&parse_program("3,3,1107,-1,8,3,4,3,99"))
                .run(vec!(7))
                .last()
                .unwrap(),
            1
        );
        assert_eq!(
            *IntcodeComputer::new(&parse_program("3,3,1107,-1,8,3,4,3,99"))
                .run(vec!(9))
                .last()
                .unwrap(),
            0
        );
    }

    #[test]
    fn compare_to_zero() {
        assert_eq!(
            *IntcodeComputer::new(&parse_program("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9"))
                .run(vec!(0))
                .last()
                .unwrap(),
            0
        );
        assert_eq!(
            *IntcodeComputer::new(&parse_program("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9"))
                .run(vec!(-1))
                .last()
                .unwrap(),
            1
        );
        assert_eq!(
            *IntcodeComputer::new(&parse_program("3,3,1105,-1,9,1101,0,0,12,4,12,99,1"))
                .run(vec!(0))
                .last()
                .unwrap(),
            0
        );
        assert_eq!(
            *IntcodeComputer::new(&parse_program("3,3,1105,-1,9,1101,0,0,12,4,12,99,1"))
                .run(vec!(-1))
                .last()
                .unwrap(),
            1
        );
    }
    #[test]
    fn compare_to_val() {
        let program = parse_program("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99");
        assert_eq!(*IntcodeComputer::new(&program).run(vec!(0)).last().unwrap(), 999);
        assert_eq!(*IntcodeComputer::new(&program).run(vec!(8)).last().unwrap(), 1000);
        assert_eq!(*IntcodeComputer::new(&program).run(vec!(20)).last().unwrap(), 1001);
    }

    #[test]
    fn quine_test() {
        let prog = parse_program("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
        assert_eq!(IntcodeComputer::new(&prog).run(vec!()), prog);
    }
    #[test]
    fn sixteen_digit_output() {
        assert_eq!(
            IntcodeComputer::new(&parse_program("1102,34915192,34915192,7,4,7,99,0"))
                .run(vec![])
                .last()
                .unwrap()
                .to_string()
                .len(),
            16
        );
    }
    #[test]
    fn large_number() {
        let prog = parse_program("104,1125899906842624,99");
        assert_eq!(
            *IntcodeComputer::new(&prog).run(vec![]).last().unwrap(),
            1125899906842624
        );
    }
}
