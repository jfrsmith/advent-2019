use rayon::prelude::*;
use std::collections::HashSet;

type Asteroid = (i32, i32);
type AsteroidField = Vec<Asteroid>;

struct Quadrants {
    ne_quadrant: AsteroidField,
    se_quadrant: AsteroidField,
    sw_quadrant: AsteroidField,
    nw_quadrant: AsteroidField,
}

fn parse_asteroids(input: &'static str) -> AsteroidField {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some((x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .collect()
}

fn get_angle(from: &Asteroid, to: &Asteroid) -> i64 {
    (((to.1 - from.1) as f32)
        .atan2((to.0 - from.0) as f32)
        .to_degrees()
        * 100.0) as i64
}

fn get_num_unique_collision_angles(asteroids: &AsteroidField) -> Vec<usize> {
    asteroids
        .par_iter()
        .map(|a| {
            let mut angle_set = HashSet::new();
            for other_a in asteroids {
                if a != other_a {
                    angle_set.insert(get_angle(a, other_a));
                }
            }
            angle_set.len()
        })
        .collect()
}

fn get_best_location_and_count(asteroids: &AsteroidField) -> ((i32, i32), usize) {
    let collision_angles = get_num_unique_collision_angles(asteroids);
    let (idx, max) = collision_angles
        .iter()
        .enumerate()
        .max_by_key(|(_, angle)| *angle)
        .unwrap();

    (asteroids[idx], *max)
}

fn get_dist(from: &Asteroid, to: &Asteroid) -> i32 {
    (to.0 - from.0).abs() + (to.1 - from.1).abs()
}

fn split_into_quadrants(from: &Asteroid, asteroids: &AsteroidField) -> Quadrants {
    //order by distance from point
    let mut ordered_roids = asteroids.clone();
    ordered_roids.sort_by(|a, b| get_dist(from, &a).cmp(&get_dist(from, &b)));

    Quadrants {
        ne_quadrant: ordered_roids
            .clone()
            .into_iter()
            .filter(|(x, y)| x >= &from.0 && y < &from.1)
            .collect(),
        se_quadrant: ordered_roids
            .clone()
            .into_iter()
            .filter(|(x, y)| x >= &from.0 && y >= &from.1)
            .collect(),
        sw_quadrant: ordered_roids
            .clone()
            .into_iter()
            .filter(|(x, y)| x < &from.0 && y >= &from.1)
            .collect(),
        nw_quadrant: ordered_roids
            .clone()
            .into_iter()
            .filter(|(x, y)| x < &from.0 && y < &from.1)
            .collect(),
    }
}

fn sweep_quadrant(from: &Asteroid, quadrant: &AsteroidField) -> Vec<Asteroid> {
    let mut hit_angles = HashSet::new();
    let mut hit_asteroids: Vec<(Asteroid, i64)> = vec![];

    for to in quadrant {
        if to != from {
            let hit_angle = get_angle(from, to);
            if hit_angles.insert(hit_angle) {
                hit_asteroids.push((*to, hit_angle))
            }
        }
    }

    hit_asteroids.sort_by(|a, b| a.1.cmp(&b.1));
    hit_asteroids.iter().map(|roid| roid.0).collect()
}

fn sweep_laser_hits(from: &Asteroid, asteroids: &AsteroidField) -> Vec<Asteroid> {
    let quadrants = split_into_quadrants(from, asteroids);

    let ne_hits = sweep_quadrant(from, &quadrants.ne_quadrant);
    let se_hits = sweep_quadrant(from, &quadrants.se_quadrant);
    let sw_hits = sweep_quadrant(from, &quadrants.sw_quadrant);
    let nw_hits = sweep_quadrant(from, &quadrants.nw_quadrant);

    [
        ne_hits.as_slice(),
        se_hits.as_slice(),
        sw_hits.as_slice(),
        nw_hits.as_slice(),
    ]
    .concat()
}

fn find_nth_destroyed_asteroid(
    laser_roid: &Asteroid,
    in_field: &AsteroidField,
    nth: usize,
) -> Asteroid {
    let mut destruction_field = in_field.clone();
    let mut destroyed_roids = vec![];

    while destroyed_roids.len() < nth {
        if in_field.is_empty() {
            unreachable!();
        }

        let mut hit_roids = sweep_laser_hits(laser_roid, &destruction_field);

        //remove any hit asteroids from the field
        destruction_field = destruction_field
            .into_iter()
            .filter(|roid| hit_roids.contains(roid))
            .collect();

        destroyed_roids.append(&mut hit_roids);
    }

    destroyed_roids[nth - 1]
}

fn main() {
    let asteroids = parse_asteroids(include_str!("../input/day_10.txt"));
    let best = get_best_location_and_count(&asteroids);
    println!("Part 1 => {} at {:?}", best.1, best.0);

    let found_vaporised_roid = find_nth_destroyed_asteroid(&best.0, &asteroids, 200);
    println!("Vaporised roid is {:?}", found_vaporised_roid);
    println!(
        "Part 2 => {}",
        (found_vaporised_roid.0 * 100) + found_vaporised_roid.1
    );
}

#[test]
fn simple() {
    let asteroids = ".#..#
.....
#####
....#
...##";

    assert_eq!(
        ((3, 4), 8),
        get_best_location_and_count(&parse_asteroids(asteroids))
    );
}

#[test]
fn case_1() {
    assert_eq!(
        ((5, 8), 33),
        get_best_location_and_count(&parse_asteroids(include_str!("../input/test_1.txt")))
    );
}

#[test]
fn case_2() {
    assert_eq!(
        ((1, 2), 35),
        get_best_location_and_count(&parse_asteroids(include_str!("../input/test_2.txt")))
    );
}

#[test]
fn case_3() {
    assert_eq!(
        ((6, 3), 41),
        get_best_location_and_count(&parse_asteroids(include_str!("../input/test_3.txt")))
    );
}

#[test]
fn case_4() {
    assert_eq!(
        ((11, 13), 210),
        get_best_location_and_count(&parse_asteroids(include_str!("../input/test_4.txt")))
    );
}

#[test]
fn part_1_complete() {
    assert_eq!(
        ((26, 28), 267),
        get_best_location_and_count(&parse_asteroids(include_str!("../input/day_10.txt")))
    );
}

#[test]
fn laser_sweep() {
    let asteroid_field = parse_asteroids(include_str!("../input/laser_test.txt"));
    let laser_location = (8, 3);

    let mut hit = sweep_laser_hits(&laser_location, &asteroid_field);
    hit.truncate(9);
    let expected = vec![
        (8, 1),
        (9, 0),
        (9, 1),
        (10, 0),
        (9, 2),
        (11, 1),
        (12, 1),
        (11, 2),
        (15, 1),
    ];
    assert_eq!(hit, expected);
}

#[test]
fn part_2_laser_sweep() {
    let asteroid_field = parse_asteroids(include_str!("../input/test_4.txt"));
    let laser_location = get_best_location_and_count(&asteroid_field).0;

    assert_eq!(
        (11, 12),
        find_nth_destroyed_asteroid(&laser_location, &asteroid_field, 1)
    );

    assert_eq!(
        (12, 1),
        find_nth_destroyed_asteroid(&laser_location, &asteroid_field, 2)
    );
    assert_eq!(
        (12, 2),
        find_nth_destroyed_asteroid(&laser_location, &asteroid_field, 3)
    );
    assert_eq!(
        (12, 8),
        find_nth_destroyed_asteroid(&laser_location, &asteroid_field, 10)
    );
}

#[test]
fn part_2_complete() {
    assert_eq!(
        (13, 9),
        find_nth_destroyed_asteroid(
            &(26, 28),
            &parse_asteroids(include_str!("../input/day_10.txt")),
            200
        )
    );
}
