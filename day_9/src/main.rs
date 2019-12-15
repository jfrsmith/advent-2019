use intcode::*;

fn main() {
    let program = parse_program(include_str!("../input/day_9.txt"));

    println!(
        "Part 1 => {}",
        IntcodeComputer::new(&program)
            .run(vec!(1))
            .last()
            .unwrap()
    );
    println!(
        "Part 2 => {}",
        IntcodeComputer::new(&program)
            .run(vec!(2))
            .last()
            .unwrap()
    );
}

#[test]
fn part_1_complete() {
    let program = parse_program(include_str!("../input/day_9.txt"));
    assert_eq!(
        *IntcodeComputer::new(&program)
            .run(vec!(1))
            .last()
            .unwrap(),
        2518058886
    );
}

#[test]
fn part_2_complete() {
    let program = parse_program(include_str!("../input/day_9.txt"));
    assert_eq!(
        *IntcodeComputer::new(&program)
            .run(vec!(2))
            .last()
            .unwrap(),
        44292
    );
}
