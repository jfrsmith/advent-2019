#[derive(Debug, Clone)]
enum Direction {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

type Path = Vec<Direction>;

fn parse_path(path_str: &'static str) -> Path {
    path_str
        .split(',')
        .map(|s: &str| {
            let (dir, val) = s.split_at(1);
            match dir {
                "R" => Direction::Right(val.parse::<i32>().unwrap()),
                "L" => Direction::Left(val.parse::<i32>().unwrap()),
                "U" => Direction::Up(val.parse::<i32>().unwrap()),
                "D" => Direction::Down(val.parse::<i32>().unwrap()),
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

fn get_intercept_dist(path_a: &Path, path_b: &Path) -> i32 {
    unimplemented!()
}

fn main() {
    let (path_a, path_b) = parse_paths(include_str!("../input/day_3.txt"));
    println!("Part 1: {:?}", get_intercept_dist(&path_a, &path_b));
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
        159
    );
}
