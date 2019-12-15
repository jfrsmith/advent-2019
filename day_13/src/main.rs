use intcode::*;
use console::Term;

enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball
}

impl Tile {
    fn from_int(i: i64) -> Tile {
        match i {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            _ => panic!("Unsupported Tile type")
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match *self {
            Tile::Empty => ' ',
            Tile::Wall => '█',
            Tile::Block => '▒',
            Tile::Paddle => '▃',
            Tile::Ball => '●'
        };

        write!(f, "{}", c)
    }
}

fn play_game(initial_memory: &Vec<i64>) -> i64 {
    let mut hacked_memory = initial_memory.clone();
    hacked_memory[0] = 2;

    let mut computer = IntcodeComputer::new(&hacked_memory);
    let mut frame_buffer = vec!();
    let mut score = 0;

    let terminal = Term::stdout();
    terminal.clear_screen().unwrap();

    let mut ball_x = 0;
    let mut paddle_x = 0;

    loop {
        let next_frame = computer.tick();
        match next_frame {
            Output::OutputVal(x) => {
                frame_buffer.push(x);
                if frame_buffer.len() == 3 {
                    if frame_buffer[0] == -1 && frame_buffer[1] == 0 {
                        score = frame_buffer[2];
                        terminal.move_cursor_to(0, 0).unwrap();
                        terminal.write_line(&format!("Score: {}", score)).unwrap();
                    } else {
                        let x = frame_buffer[0] as usize;
                        let y = frame_buffer[1] as usize;
                        let tile_type = Tile::from_int(frame_buffer[2]);

                        match tile_type {
                            Tile::Ball => ball_x = x,
                            Tile::Paddle => paddle_x = x,
                            _ => {},
                        }
                        
                        terminal.move_cursor_to(x, y + 1).unwrap();
                        terminal.write_line(&tile_type.to_string()).unwrap();
                    }
                    frame_buffer.clear();
                }
            },
            Output::WaitingForInput => {
                computer.provide_input(if paddle_x < ball_x {1} else if paddle_x > ball_x {-1} else {0});
            },
            Output::Exit => {
                terminal.read_key().unwrap();
                break;
            }
        }
    }

    score
}

fn main() {
    let program = parse_program(include_str!("../input/day_13.txt"));
    let num_blocks = IntcodeComputer::new(&program)
                                        .run(vec!())
                                        .chunks_exact(3)
                                        .filter(|cmd| cmd[2] == 2)
                                        .count();
    println!("Part 1 => {}", num_blocks);

    println!("Part 2 => {}", play_game(&program));
}

#[test]
fn part_1_complete() {
    let num_blocks = IntcodeComputer::new(&parse_program(include_str!("../input/day_13.txt")))
    .run(vec!())
    .chunks_exact(3)
    .filter(|cmd| cmd[2] == 2)
    .count();

    assert_eq!(num_blocks, 213);
}