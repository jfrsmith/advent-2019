fn read_values(program_state: &Vec<i32>, op_pos: usize) -> (i32, i32, usize) {
    (
        program_state[program_state[op_pos + 1] as usize], 
        program_state[program_state[op_pos + 2] as usize], 
        program_state[op_pos + 3] as usize
    )
}

fn run_program(mut program_state: Vec<i32>) -> Vec<i32> {

    let mut instr_ptr = 0 as usize;

    loop {
        match program_state[instr_ptr] {
            1 => {
                let (in_1, in_2, out_addr) = read_values(&program_state, instr_ptr);
                program_state[out_addr] = in_1 + in_2;
            },
            2 => {
                let (in_1, in_2, out_addr) = read_values(&program_state, instr_ptr);
                program_state[out_addr] = in_1 * in_2;
            },
            99 => break,
            _ => panic!("Unexpected op_code")
        }

        instr_ptr = instr_ptr + 4;
    }

    program_state
}

fn parse(input_str: &'static str) -> Vec<i32> {
    input_str.split(',').map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>()
}

fn modify_initial_state(initial_state: Vec<i32>, addr_1_val: i32, addr_2_val: i32) -> Vec<i32> {
    let mut mod_state = initial_state.clone();
    mod_state[1] = addr_1_val;
    mod_state[2] = addr_2_val;
    mod_state
}

fn find_target_value(initial_state: Vec<i32>, target_val: i32) -> (i32, i32) {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let output = run_program(modify_initial_state(initial_state.clone(), noun, verb))[0];
            if output == target_val {
                return (noun, verb);
            }
        }
    }

    unreachable!("Failed to find target value!");
}

fn main() {
    let initial_state = parse(include_str!("../input/day_2.txt"));
    
    let program_state = modify_initial_state(initial_state.clone(), 12, 2);
    println!("Part 1 => {}", run_program(program_state)[0]);

    let (noun, verb) = find_target_value(initial_state.clone(), 19690720);
    println!("Part 2 => {}", 100 * noun + verb);
}

#[test]
fn part_1_test() {
    let mut expected = vec!(2,0,0,0,99);
    assert_eq!(run_program(parse("1,0,0,0,99")), expected);

    expected = vec!(2,3,0,6,99);
    assert_eq!(run_program(parse("2,3,0,3,99")), expected);

    expected = vec!(2,4,4,5,99,9801);
    assert_eq!(run_program(parse("2,4,4,5,99,0")), expected);

    expected = vec!(30,1,1,4,2,5,6,0,99);
    assert_eq!(run_program(parse("1,1,1,4,99,5,6,0,99")), expected);
}

#[test]
fn part_1_complete() {
    let initial_state = parse(include_str!("../input/day_2.txt"));
    let program_state = modify_initial_state(initial_state, 12, 2);
    assert_eq!(run_program(program_state)[0], 7210630);
}

#[test]
fn part_2_complete() {
    let initial_state = parse(include_str!("../input/day_2.txt"));
    let (noun, verb) = find_target_value(initial_state.clone(), 19690720);
    assert_eq!(100 * noun + verb, 3892);
}