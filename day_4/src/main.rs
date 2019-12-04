use digits_iterator::*;
use itertools::Itertools;

fn is_not_decreasing(pwd: &Vec<u8>) -> bool {
    pwd.windows(2).all(|w| w[0] <= w[1])
}

fn has_adjacent_digits(pwd: &Vec<u8>, limit_repeating_digits: bool) -> bool {
    let split: Vec<Vec<&u8>> = pwd
        .iter()
        .group_by(|&x| x)
        .into_iter()
        .map(|(_, r)| r.collect())
        .collect();

    if split.len() < pwd.len() {
        //we have repeating elements
        if !limit_repeating_digits {
            return true;
        }

        return split.iter().any(|s| s.len() == 2);
    }
    false
}

fn is_valid_password(pwd: u32, limit_repeating_digits: bool) -> bool {
    let digits = pwd.digits().collect();
    is_not_decreasing(&digits) && has_adjacent_digits(&digits, limit_repeating_digits)
}

fn main() {
    println!(
        "Part 1: {:?}",
        (245318..=765747)
            .filter(|p| is_valid_password(*p, false))
            .count()
    );
    println!(
        "Part 2: {:?}",
        (245318..=765747)
            .filter(|p| is_valid_password(*p, true))
            .count()
    );
}

#[test]
fn part_1() {
    assert_eq!(is_valid_password(111111, false), true);
    assert_eq!(is_valid_password(223450, false), false);
    assert_eq!(is_valid_password(123789, false), false);
}

#[test]
fn part_2() {
    assert_eq!(is_valid_password(112233, true), true);
    assert_eq!(is_valid_password(123444, true), false);
    assert_eq!(is_valid_password(111122, true), true);
}
