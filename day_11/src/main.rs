use intcode::*;
use std::collections::HashMap;

type Panel = (i32, i32);
type PaintedGrid = HashMap<Panel, u8>;

enum Facing {
    North,
    East,
    South,
    West,
}

enum Rotate {
    Left,
    Right,
}

impl Facing {
    fn rotate(&self, rotate: Rotate) -> Facing {
        match *self {
            Facing::North => match rotate {
                Rotate::Left => Facing::West,
                Rotate::Right => Facing::East,
            },
            Facing::East => match rotate {
                Rotate::Left => Facing::North,
                Rotate::Right => Facing::South,
            },
            Facing::South => match rotate {
                Rotate::Left => Facing::East,
                Rotate::Right => Facing::West,
            },
            Facing::West => match rotate {
                Rotate::Left => Facing::South,
                Rotate::Right => Facing::North,
            },
        }
    }
}

struct PaintBot {
    brain: IntcodeComputer,
    facing_dir: Facing,
    location: Panel,
    grid: PaintedGrid,
}

impl PaintBot {
    fn new(program: &'static str) -> PaintBot {
        PaintBot {
            brain: IntcodeComputer::new(&parse_program(program)),
            facing_dir: Facing::North,
            location: (0, 0),
            grid: PaintedGrid::new(),
        }
    }

    fn paint_panel(&mut self, panel: Panel, color: u8) {
        let panel_color = self.grid.entry(panel).or_insert(0);
        *panel_color = color;
    }

    fn rotate_and_move(&mut self, rot: Rotate) {
        self.facing_dir = self.facing_dir.rotate(rot);
        self.location = match self.facing_dir {
            Facing::North => (self.location.0, self.location.1 - 1),
            Facing::East => (self.location.0 + 1, self.location.1),
            Facing::South => (self.location.0, self.location.1 + 1),
            Facing::West => (self.location.0 - 1, self.location.1),
        };
    }

    fn tick(&mut self) -> bool {
        let panel_color = self.grid.get(&self.location).unwrap_or(&0);

        let new_color = match self.brain.run(vec![*panel_color as i64]) {
            Some(x) => x as u8,
            None => return false,
        };

        self.paint_panel(self.location, new_color);

        match self.brain.run(vec![]) {
            Some(x) => self.rotate_and_move(if x == 0 { Rotate::Left } else { Rotate::Right }),
            None => return false,
        };

        true
    }

    fn paint(&mut self, starting_panel_color: u8) -> PaintedGrid {
        self.paint_panel(self.location, starting_panel_color);
        loop {
            if !self.tick() {
                break;
            }
        }

        self.grid.clone()
    }
}

fn main() {
    println!(
        "Part 1 => {}",
        PaintBot::new(include_str!("../input/day_11.txt"))
            .paint(0)
            .keys()
            .len()
    );

    let painted_panels = PaintBot::new(include_str!("../input/day_11.txt")).paint(1);
    let min_x = painted_panels.keys().map(|p| p.0).min();
    let max_x = painted_panels.keys().map(|p| p.0).max();

    let min_y = painted_panels.keys().map(|p| p.1).min();
    let max_y = painted_panels.keys().map(|p| p.1).max();

    println!("Part 2 => \n");
    for y in min_y.unwrap()..=max_y.unwrap() {
        for x in min_x.unwrap()..max_x.unwrap() {
            print!(
                "{}",
                if *painted_panels.get(&(x, y)).unwrap_or(&0) == 0 {
                    '▓'
                } else {
                    '░'
                }
            );
        }
        println!("");
    }
}

#[test]
fn part_1_complete() {
    assert_eq!(
        2255,
        PaintBot::new(include_str!("../input/day_11.txt"))
            .paint(0)
            .keys()
            .len()
    );
}
