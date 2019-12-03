use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Instruction {
    dir : Direction,
    dist : i32
}

type Path = Vec<Instruction>;

fn parse_path(path_str: &'static str) -> Path {
    path_str
        .split(',')
        .map(|s: &str| {
            let (dir, val) = s.split_at(1);
            match dir {
                "R" => Instruction { dir : Direction::Right, dist : val.parse::<i32>().unwrap() },
                "L" => Instruction { dir : Direction::Left, dist : val.parse::<i32>().unwrap() },
                "U" => Instruction { dir : Direction::Up, dist : val.parse::<i32>().unwrap() },
                "D" => Instruction { dir : Direction::Down, dist : val.parse::<i32>().unwrap() },
                _ => panic!("Unexpected path value"),
            }
        })
        .collect::<Path>()
}

fn parse_paths(input: &'static str) -> (Path, Path) {
    let paths: Vec<Path> = input.lines().map(|p| parse_path(p)).collect::<Vec<Path>>();
    assert!(paths.len() == 2);
    (paths[0].clone(), paths[1].clone())
}

fn step(from: (i32, i32), dir: Direction) -> (i32, i32) {
    match dir {
        Direction::Up => (from.0, from.1+1),
        Direction::Down => (from.0, from.1-1),
        Direction::Right => (from.0+1, from.1),
        Direction::Left => (from.0-1, from.1),
    }
}

fn walk(path: &Path) -> HashSet<(i32, i32)> {
    let mut pos = (0,0);
    let mut visited = HashSet::new();

    for instr in path {
        for _ in 0..instr.dist {
            pos = step(pos, instr.dir);
            visited.insert(pos);
        }
    }

    visited
}

fn count_steps_to(point: (i32, i32), on_path: &Path) -> i32 {
    let mut pos = (0,0);
    let mut steps = 0;

    for instr in on_path {
        for _ in 0..instr.dist {
            pos = step(pos, instr.dir);
            steps = steps + 1;
            if pos == point {
                return steps;
            }
        }
    }

    unreachable!()    
}

fn get_intercept_dist(path_a: &Path, path_b: &Path) -> i32 {
    walk(path_a).intersection(&walk(path_b)).filter(|i| **i != (0,0)).map(|i| i.0.abs() + i.1.abs()).min().unwrap()
}

fn get_intercept_min_steps(path_a: &Path, path_b: &Path) -> i32 {
    walk(path_a).intersection(&walk(path_b)).filter(|i| **i != (0,0)).map(|i| count_steps_to(*i, &path_a) + count_steps_to(*i, &path_b)).min().unwrap()
}

fn main() {
    let (path_a, path_b) = parse_paths(include_str!("../input/day_3.txt"));
    println!("Part 1: {:?}", get_intercept_dist(&path_a, &path_b));
    println!("Part 2: {:?}", get_intercept_min_steps(&path_a, &path_b));
}

#[test]
fn part_1() {
    assert_eq!(
        get_intercept_dist(
            &parse_path("R75,D30,R83,U83,L12,D49,R71,U7,L72"),
            &parse_path("U62,R66,U55,R34,D71,R55,D58,R83")
        ),
        159
    );

    assert_eq!(
        get_intercept_dist(
            &parse_path("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"),
            &parse_path("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")
        ),
        135
    );

    assert_eq!(
        get_intercept_dist(&parse_path("R8,U5,L5,D3"), &parse_path("U7,R6,D4,L4")),
        6
    );
}

#[test]
fn part_1_complete() {
    let (path_a, path_b) = parse_paths(include_str!("../input/day_3.txt"));
    assert_eq!(get_intercept_dist(&path_a, &path_b), 870);
}

#[test]
fn part_2() {
    assert_eq!(
        get_intercept_min_steps(
            &parse_path("R75,D30,R83,U83,L12,D49,R71,U7,L72"),
            &parse_path("U62,R66,U55,R34,D71,R55,D58,R83")
        ),
        610
    );
    
    assert_eq!(
        get_intercept_min_steps(
            &parse_path("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"),
            &parse_path("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")
        ),
        410
    );
}

#[test]
fn part_2_complete() {
    let (path_a, path_b) = parse_paths(include_str!("../input/day_3.txt"));
    assert_eq!(get_intercept_min_steps(&path_a, &path_b), 13698);
}