use std::{error::Error, fs};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Empty,
    Down,
    Right,
}

fn print_map(map: &Vec<Vec<State>>) {
    for line in map {
        for s in line {
            match s {
                State::Empty => {
                    print!(".")
                }
                State::Down => {
                    print!("v")
                }
                State::Right => {
                    print!(">")
                }
            }
        }
        println!();
    }
    println!();
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut map = Vec::new();

    for line in fs::read_to_string("input")?.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(match c {
                '>' => State::Right,
                'v' => State::Down,
                '.' => State::Empty,
                _ => panic!(),
            });
        }
        map.push(row);
    }

    let mut step = 0;

    let mut did_move = true;
    while did_move {
        did_move = false;

        print_map(&map);

        let old_map = map.clone();

        for y in 0..old_map.len() {
            for x in 0..old_map[y].len() {
                map[y][x] = State::Empty;
            }
        }

        for y in 0..old_map.len() {
            for x in 0..old_map[y].len() {
                match old_map[y][x] {
                    State::Empty => {}
                    State::Down => {
                        map[y][x] = State::Down;
                    }
                    State::Right => {
                        if old_map[y][(x + 1) % old_map[y].len()] == State::Empty {
                            map[y][(x + 1) % old_map[y].len()] = State::Right;
                            did_move = true;
                        } else {
                            map[y][x] = State::Right;
                        }
                    }
                }
            }
        }

        let old_map = map.clone();

        for y in 0..old_map.len() {
            for x in 0..old_map[y].len() {
                map[y][x] = State::Empty;
            }
        }

        for y in 0..old_map.len() {
            for x in 0..old_map[y].len() {
                match old_map[y][x] {
                    State::Empty => {}
                    State::Down => {
                        if old_map[(y + 1) % old_map.len()][x] == State::Empty {
                            map[(y + 1) % old_map.len()][x] = State::Down;
                            did_move = true;
                        } else {
                            map[y][x] = State::Down;
                        }
                    }
                    State::Right => {
                        map[y][x] = State::Right;
                    }
                }
            }
        }

        step += 1;

        if step == 4 {
            // break;
        }
    }

    print_map(&map);

    println!("steps: {}", step);

    Ok(())
}
