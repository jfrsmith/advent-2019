use intcode::*;

fn main() {
    let prog = parse_program(include_str!("../input/day_5.txt"));
    println!(
        "Part 1 => {}",
        IntcodeComputer::new(&prog)
            .run_until_halt(vec!(1))
            .last()
            .unwrap()
    );

    println!(
        "Part 2 => {}",
        IntcodeComputer::new(&prog)
            .run_until_halt(vec!(5))
            .last()
            .unwrap()
    );
}

#[test]
fn part_1_complete() {
    assert_eq!(
        *IntcodeComputer::new(&parse_program(include_str!("../input/day_5.txt")))
            .run_until_halt(vec!(1))
            .last()
            .unwrap(),
        4887191
    );
}

#[test]
fn part_2_complete() {
    assert_eq!(
        *IntcodeComputer::new(&parse_program(include_str!("../input/day_5.txt")))
            .run_until_halt(vec!(5))
            .last()
            .unwrap(),
        3419022
    );
}
