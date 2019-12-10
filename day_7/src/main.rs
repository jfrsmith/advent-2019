use intcode::*;
use permutohedron::Heap;

struct Thrusters {
    phase_seq: Vec<i64>,
    program: Vec<i64>,
    feedback_loop: bool,
}

impl Thrusters {
    fn new(program: Vec<i64>, phase_seq: Vec<i64>, feedback_loop: bool) -> Thrusters {
        Thrusters {
            phase_seq,
            program,
            feedback_loop,
        }
    }

    fn run(&self) -> i64 {
        let mut amps = self
            .phase_seq
            .iter()
            .map(|_| IntcodeComputer::new(&self.program))
            .collect::<Vec<IntcodeComputer>>();
        let mut last_output = self
            .phase_seq
            .iter()
            .enumerate()
            .fold(0, |input, (i, setting)| {
                amps[i].run(vec![*setting, input]).unwrap()
            });

        if !self.feedback_loop {
            return last_output;
        }

        loop {
            if (0..amps.len() - 1).all(|i| amps[i].get_last_result().is_none()) {
                return last_output;
            }

            last_output = self
                .phase_seq
                .iter()
                .enumerate()
                .fold(last_output, |input, (i, _)| {
                    amps[i].run(vec![input]).unwrap_or(input)
                });
        }
    }
}

fn get_sequence_permutations(min: i64, max: i64) -> Vec<Vec<i64>> {
    let mut data = (min..=max).collect::<Vec<i64>>();
    let heap = Heap::new(&mut data);

    let mut permutations = Vec::new();
    for data in heap {
        permutations.push(data.clone());
    }

    permutations
}

fn main() {
    let prog = parse_program(include_str!("../input/day_7.txt"));
    let max_val = get_sequence_permutations(0, 4)
        .iter()
        .map(|s| Thrusters::new(prog.clone(), s.clone(), false).run())
        .max()
        .unwrap();
    println!("Part 1 => {}", max_val);

    let max_feedback_val = get_sequence_permutations(5, 9)
        .iter()
        .map(|s| Thrusters::new(prog.clone(), s.clone(), true).run())
        .max()
        .unwrap();
    println!("Part 2 => {}", max_feedback_val);
}

#[test]
fn thruster_signal() {
    let mut prog = parse_program("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
    assert_eq!(
        Thrusters::new(prog, vec!(4, 3, 2, 1, 0), false).run(),
        43210
    );

    prog =
        parse_program("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0");
    assert_eq!(
        Thrusters::new(prog, vec!(0, 1, 2, 3, 4), false).run(),
        54321
    );

    prog = parse_program("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0");
    assert_eq!(
        Thrusters::new(prog, vec!(1, 0, 4, 3, 2), false).run(),
        65210
    );
}

#[test]
fn part_1_complete() {
    let prog = parse_program(include_str!("../input/day_7.txt"));
    let max_val = get_sequence_permutations(0, 4)
        .iter()
        .map(|s| Thrusters::new(prog.clone(), s.clone(), false).run())
        .max()
        .unwrap();
    assert_eq!(max_val, 11828);
}

#[test]
fn thruster_signal_feedback_loop() {
    let mut prog = parse_program(
        "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",
    );
    assert_eq!(
        Thrusters::new(prog, vec!(9, 8, 7, 6, 5), true).run(),
        139629729
    );

    prog = parse_program("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10");
    assert_eq!(Thrusters::new(prog, vec!(9, 7, 8, 5, 6), true).run(), 18216);
}

#[test]
fn part_2_complete() {
    let prog = parse_program(include_str!("../input/day_7.txt"));

    let max_feedback_val = get_sequence_permutations(5, 9)
        .iter()
        .map(|s| Thrusters::new(prog.clone(), s.clone(), true).run())
        .max()
        .unwrap();
    println!("Part 2 => {}", max_feedback_val);
    assert_eq!(max_feedback_val, 1714298);
}
