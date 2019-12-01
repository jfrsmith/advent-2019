fn calc_fuel(in_val: i32) -> i32 {
    (in_val as f32 / 3.0).floor() as i32 - 2
}

fn calc_fuel_2(in_val: i32) -> i32 {
    let mut fuel_total = 0;
    let mut next_fuel_val = in_val;
    
    loop {
        next_fuel_val = calc_fuel(next_fuel_val);

        if next_fuel_val < 0 {
            break;
        }

        fuel_total = fuel_total + next_fuel_val;
    }

    fuel_total
}

fn main() {
    let mass_vals = include_str!("../input/day_1.txt");
    println!("Part 1: {}", mass_vals.lines().map(|l| calc_fuel(l.parse::<i32>().unwrap())).sum::<i32>());
    println!("Part 2: {}", mass_vals.lines().map(|l| calc_fuel_2(l.parse::<i32>().unwrap())).sum::<i32>());
}

#[test]
fn part_1_tests() {
    assert_eq!(calc_fuel(12), 2);
    assert_eq!(calc_fuel(14), 2);
    assert_eq!(calc_fuel(1969), 654);
    assert_eq!(calc_fuel(100756), 33583);
}

#[test]
fn part_2_test() {
    assert_eq!(calc_fuel_2(14), 2);
    assert_eq!(calc_fuel_2(1969), 966);
    assert_eq!(calc_fuel_2(100756), 50346);
}