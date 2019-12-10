use std::collections::HashSet;
use rayon::prelude::*;

type AsteroidField = Vec<(i32, i32)>;

fn parse_asteroids(input: &'static str) -> AsteroidField {
    input.lines().enumerate().flat_map(|(y, l)| l.chars().enumerate().filter_map(move |(x, c)| if c == '#' { Some((x as i32,y as i32)) } else { None })).collect()
}

fn get_num_unique_collision_angles(asteroids: &AsteroidField) -> Vec<usize> {
    asteroids.par_iter().map(|a| {
        let mut angle_set = HashSet::new();
        for other_a in asteroids {
            if a != other_a {
                //can't compare f32 so bump it up to i64
                angle_set.insert((((a.1 - other_a.1) as f32).atan2((a.0 - other_a.0) as f32).to_degrees() * 100.0) as i64);
            }
        }
        angle_set.len()
    }).collect()
}

fn get_best_location_and_count(asteroids: &AsteroidField) -> ((i32, i32), usize) {
    let collision_angles = get_num_unique_collision_angles(asteroids);
    let (idx, max) = collision_angles.iter().enumerate().max_by_key(|(_, angle)| *angle).unwrap();

    (asteroids[idx], *max)
}

fn main() {
    let asteroids = parse_asteroids(include_str!("../input/day_10.txt"));
    let best = get_best_location_and_count(&asteroids);
    println!("Part 1 => {} at {:?}", best.1, best.0);
}

#[test]
fn simple() {
    let asteroids = ".#..#
.....
#####
....#
...##";

    assert_eq!(((3, 4), 8), get_best_location_and_count(&parse_asteroids(asteroids)));  
}

#[test]
fn case_1() {
    assert_eq!(((5, 8), 33), get_best_location_and_count(&parse_asteroids(include_str!("../input/test_1.txt"))));
}

#[test]
fn case_2() {
    assert_eq!(((1, 2), 35), get_best_location_and_count(&parse_asteroids(include_str!("../input/test_2.txt"))));
}

#[test]
fn case_3() {
    assert_eq!(((6, 3), 41), get_best_location_and_count(&parse_asteroids(include_str!("../input/test_3.txt"))));
}

#[test]
fn case_4() {
    assert_eq!(((11, 13), 210), get_best_location_and_count(&parse_asteroids(include_str!("../input/test_4.txt"))));
}

#[test]
fn part_1_complete() {
    assert_eq!(((26, 28), 267), get_best_location_and_count(&parse_asteroids(include_str!("../input/day_10.txt"))));
}