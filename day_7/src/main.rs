use permutohedron::Heap;

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
    inputs: Vec<i32>,
    input_count: usize,
    last_result: Option<i32>
}

fn read_parameter_mode(param_num: usize, param_modes: &Vec<u8>) -> Mode {
    if param_num < param_modes.len() && param_modes[param_num] == 1 {
        Mode::Immediate
    } else {
        Mode::Position
    }
}

fn parse_program(program: &'static str) -> Vec<i32> {
    program.split(',').map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>()
}

impl IntcodeComputer {
    fn new(program: &Vec<i32>) -> IntcodeComputer {
        IntcodeComputer {
            memory: program.clone(),
            pointer: 0,
            inputs: vec!(),
            input_count : 0,
            last_result: None
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

    fn next_input(&mut self) -> i32 {
        if self.inputs.len() <= self.input_count {
            panic!("Not enough inputs provided!");
        }

        let input = self.inputs[self.input_count];
        self.input_count += 1;
        input
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

    fn run(&mut self, input_vals: Vec<i32>) -> Option<i32> {
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
                Instruction::Halt => {
                    self.last_result = None;
                    break;
                }
            };
        }

        self.last_result
    }

    fn get_last_result(&self) -> Option<i32> {
        self.last_result
    }
}

struct Thrusters {
    phase_seq: Vec<i32>,
    program: Vec<i32>,
    feedback_loop : bool
}

impl Thrusters {
    fn new(program: Vec<i32>, phase_seq: Vec<i32>, feedback_loop : bool) -> Thrusters {
        Thrusters {
            phase_seq,
            program,
            feedback_loop
        }
    }

    fn run(&self) -> i32 {
        let mut amps = self.phase_seq.iter().map(|_| IntcodeComputer::new(&self.program)).collect::<Vec<IntcodeComputer>>();
        let mut last_output = self.phase_seq.iter().enumerate().fold(0, |input, (i, setting)| amps[i].run(vec!(*setting, input)).unwrap());

        if !self.feedback_loop {
            return last_output;
        }

        loop {
            if (0..amps.len()-1).all(|i| amps[i].get_last_result().is_none()) {
                return last_output;
            }

            last_output = self.phase_seq.iter().enumerate().fold(last_output, |input, (i, _)| amps[i].run(vec!(input)).unwrap_or(input));
        }
    }
}

fn get_sequence_permutations(min: i32, max: i32) -> Vec<Vec<i32>> {
    let mut data = (min..=max).collect::<Vec<i32>>();
    let heap = Heap::new(&mut data);

    let mut permutations = Vec::new();
    for data in heap {
        permutations.push(data.clone());
    }

    permutations
}

fn main() {
    let prog = parse_program(include_str!("../input/day_7.txt"));
    let max_val = get_sequence_permutations(0,4).iter().map(|s| Thrusters::new(prog.clone(), s.clone(), false).run()).max().unwrap();
    println!("Part 1 => {}", max_val);

    let max_feedback_val = get_sequence_permutations(5,9).iter().map(|s| Thrusters::new(prog.clone(), s.clone(), true).run()).max().unwrap();
    println!("Part 2 => {}", max_feedback_val);
}

#[test]
fn thruster_signal() {
    let mut prog = parse_program("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
    assert_eq!(Thrusters::new(prog, vec!(4,3,2,1,0), false).run(), 43210);

    prog = parse_program("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0");
    assert_eq!(Thrusters::new(prog, vec!(0,1,2,3,4), false).run(), 54321);

    prog = parse_program("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0");
    assert_eq!(Thrusters::new(prog, vec!(1,0,4,3,2), false).run(), 65210);
}

#[test]
fn part_1_complete() {
    let prog = parse_program(include_str!("../input/day_7.txt"));
    let max_val = get_sequence_permutations(0,4).iter().map(|s| Thrusters::new(prog.clone(), s.clone(), false).run()).max().unwrap();
    assert_eq!(max_val, 11828); 
}

#[test]
fn thruster_signal_feedback_loop() {
    let mut prog = parse_program("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5");
    assert_eq!(Thrusters::new(prog, vec!(9,8,7,6,5), true).run(), 139629729);

    prog = parse_program("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10");
    assert_eq!(Thrusters::new(prog, vec!(9,7,8,5,6), true).run(), 18216);
}