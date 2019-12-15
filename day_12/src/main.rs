type CoOrd = (i32, i32, i32);
const MOON_COUNT: usize = 4;

#[derive(Debug, Eq, PartialEq, Clone)]
struct State {
    positions: [CoOrd; MOON_COUNT],
    velocities: [CoOrd; MOON_COUNT],
}

fn parse_state(input: &'static str) -> State {
    let moon_coords = input
        .lines()
        .map(|l| {
            l.trim_start()
                .trim_matches(|c| c == '>' || c == '<')
                .split(',')
                .map(|s| s.trim_start()[2..].to_owned().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    State {
        positions : [
            (moon_coords[0][0], moon_coords[0][1], moon_coords[0][2]),
            (moon_coords[1][0], moon_coords[1][1], moon_coords[1][2]),
            (moon_coords[2][0], moon_coords[2][1], moon_coords[2][2]),
            (moon_coords[3][0], moon_coords[3][1], moon_coords[3][2])
        ],
        velocities : [(0,0,0),(0,0,0),(0,0,0),(0,0,0)]
    }
}

fn relative_velocity(a: i32, b: i32) -> i32 {
    match a.cmp(&b) {
        std::cmp::Ordering::Less => 1,
        std::cmp::Ordering::Greater => -1,
        std::cmp::Ordering::Equal => 0,
    }
}

fn get_gravity(a: &(i32, i32, i32), b: &(i32, i32, i32)) -> (i32, i32, i32) {
    (
        relative_velocity(a.0, b.0),
        relative_velocity(a.1, b.1),
        relative_velocity(a.2, b.2),
    )
}

fn tick_state_mut(state: &mut State) {
    for i in 0..MOON_COUNT {
        for j in 0..MOON_COUNT {
            if state.positions[i] != state.positions[j] {
                let gravity_adjustment = get_gravity(&state.positions[i], &state.positions[j]);
                state.velocities[i].0 += gravity_adjustment.0;
                state.velocities[i].1 += gravity_adjustment.1;
                state.velocities[i].2 += gravity_adjustment.2;
            }
        }
    }

    for i in 0..MOON_COUNT {
        state.positions[i].0 += state.velocities[i].0;
        state.positions[i].1 += state.velocities[i].1;
        state.positions[i].2 += state.velocities[i].2;
    }
}

fn get_total_energy(state: &State) -> i32 {
    state
        .positions
        .iter()
        .zip(state.velocities.iter())
        .map(|((px, py, pz), (vx, vy, vz))| {
            (px.abs() + py.abs() + pz.abs()) * (vx.abs() + vy.abs() + vz.abs())
        })
        .sum()
}

fn run(input: &'static str, num_ticks: Option<i64>) -> (i32, i64) {
    let initial_state = parse_state(input);
    let mut state = initial_state.clone();
    let max_ticks = num_ticks.unwrap_or(std::i64::MAX);
    let mut tick_cnt = 0;

    while tick_cnt < max_ticks {
        tick_state_mut(&mut state);
        tick_cnt += 1;

        if state == initial_state {
            break;
        }
    }

    (get_total_energy(&state), tick_cnt)
}

fn main() {
    println!(
        "Part 1 => {}",
        run(include_str!("../input/day_12.txt"), Some(1000)).0
    );

    println!(
        "Part 2 => {}",
        run(include_str!("../input/day_12.txt"), None).1
    );
}

#[test]
fn energy_calc() {
    let input = "<x=-1, y=0, z=2>
    <x=2, y=-10, z=-7>
    <x=4, y=-8, z=8>
    <x=3, y=5, z=-1>";

    assert_eq!(run(&input, Some(10)).0, 179);
}

#[test]
fn energy_calc_hundred_steps() {
    let input = "<x=-8, y=-10, z=0>
    <x=5, y=5, z=10>
    <x=2, y=-7, z=3>
    <x=9, y=-8, z=-3>";

    assert_eq!(run(&input, Some(100)).0, 1940);
}

#[test]
fn seen_state() {
    let input = "<x=-8, y=-10, z=0>
    <x=5, y=5, z=10>
    <x=2, y=-7, z=3>
    <x=9, y=-8, z=-3>";

    assert_eq!(run(&input, None).1, 4686774924);
}