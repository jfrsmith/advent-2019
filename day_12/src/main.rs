#[derive(Debug, Copy, Clone)]
struct Moon {
    pos: (i32, i32, i32),
    vel: (i32, i32, i32),
}

fn pos_to_vel_change(a: i32, b: i32) -> i32 {
    match a.cmp(&b) {
        std::cmp::Ordering::Less => 1,
        std::cmp::Ordering::Greater => -1,
        std::cmp::Ordering::Equal => 0
    }
}

fn get_vel_change(a: (i32, i32, i32), b: (i32, i32, i32)) -> (i32, i32, i32) {
    (
        pos_to_vel_change(a.0, b.0), 
        pos_to_vel_change(a.1, b.1),
        pos_to_vel_change(a.2, b.2)
    )
}

impl Moon {
    fn new(input: &str) -> Moon {
        let coords = input.trim_start()
                            .trim_matches(|c| c == '>' || c == '<')
                            .split(',')
                            .map(|s| s.trim_start()[2..].to_owned().parse::<i32>().unwrap())
                            .collect::<Vec<i32>>();

        assert!(coords.len() == 3);
        Moon {
            pos : (coords[0], coords[1], coords[2]),
            vel : (0,0,0)
        }       
    }

    fn tick(&mut self) {
        self.pos.0 += self.vel.0;
        self.pos.1 += self.vel.1;
        self.pos.2 += self.vel.2;
    }

    fn apply_gravity(&mut self, moons: &Vec<Moon>) {
        for m in moons {
            if self.pos != m.pos {
                self.add_vel(get_vel_change(self.pos, m.pos));
            }
        }
    }

    fn add_vel(&mut self, vel: (i32, i32, i32)) {
        self.vel.0 += vel.0;
        self.vel.1 += vel.1;
        self.vel.2 += vel.2
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

fn parse_moons(input: &'static str) -> Vec<Moon> {
    input.lines().map(|l| Moon::new(l)).collect()
}

fn tick_moons(input: &'static str, max_ticks: i32) -> i32 {
    let mut moons = parse_moons(input);
    let mut tick_count = 0;

    while tick_count < max_ticks {
        let cloned_moons = moons.clone();
        for m in &mut moons {
            m.apply_gravity(&cloned_moons);
        }

        for m in &mut moons {
            m.tick();
        }

        tick_count += 1;
    }

    moons.iter().fold(0, |acc, m| acc + m.total())
}

fn find_convergent_states

fn main() {
    println!(
        "Part 1 = {}",
        tick_moons(include_str!("../input/day_12.txt"), 1000)
    );
}

#[test]
fn energy_calc() {
    let input = "<x=-1, y=0, z=2>
    <x=2, y=-10, z=-7>
    <x=4, y=-8, z=8>
    <x=3, y=5, z=-1>";

    assert_eq!(tick_moons(&input, 10), 179);
}

#[test]
fn energy_calc_hundred_steps() {
    let input = "<x=-8, y=-10, z=0>
    <x=5, y=5, z=10>
    <x=2, y=-7, z=3>
    <x=9, y=-8, z=-3>";

    assert_eq!(tick_moons(&input, 100), 1940);
}
