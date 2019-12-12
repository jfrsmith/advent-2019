struct Moon {
    pos: (i32, i32, i32),
    vel: (i32, i32, i32),
}

impl Moon {
    fn new(input: &str) -> Moon {}

    fn tick(&mut self) {}

    fn pos(&self) -> (i32, i32, i32) {
        self.pos
    }

    fn vel(&self) -> (i32, i32, i32) {
        self.vel
    }

    fn pot(&self) -> i32 {
        self.pos.0.abs() + self.pos.1.abs() + self.pos.2.abs()
    }

    fn kin(&self) -> i32 {
        self.vel.0.abs() + self.vel.1.abs() + self.vel.2.abs()
    }

    fn total(&self) -> i32 {
        self.pot() * self.kin()
    }
}

fn tick_moons(moons: &Vec<Moon>, ticks: i32) -> i32 {}

fn main() {
    println!(
        "Part 1 = {}",
        tick_moons(&parse_moons(include_str!("../input/day_12.txt")), 1000)
    );
}

#[test]
fn parse_moons() {
    let input = "<x=-1, y=0, z=2>
    <x=2, y=-10, z=-7>
    <x=4, y=-8, z=8>
    <x=3, y=5, z=-1>";

    let moons = parse_moons(&input);

    assert_eq!(moons[0].pos(), (-1, 0, 2));
    assert_eq!(moons[1].pos(), (2, 10, -7));
    assert_eq!(moons[2].pos(), (4, -8, 8));
    assert_eq!(moons[3].pos(), (3, 5, -1));
}

fn energy_calc() {
    let input = "<x=-1, y=0, z=2>
    <x=2, y=-10, z=-7>
    <x=4, y=-8, z=8>
    <x=3, y=5, z=-1>";

    let mut moons = parse_moons(&input);

    assert_eq!(tick_moons(&moons, 10), 179);
}

fn energy_calc_hundred_steps() {
    let input = "<x=-8, y=-10, z=0>
    <x=5, y=5, z=10>
    <x=2, y=-7, z=3>
    <x=9, y=-8, z=-3>";

    let mut moons = parse_moons(&input);

    assert_eq!(tick_moons(&moons, 100), 1940);
}
