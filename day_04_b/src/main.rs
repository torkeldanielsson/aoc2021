use std::{error::Error, fs};

#[derive(Debug, Clone)]
struct Tile {
    number: i32,
    called: bool,
}

#[derive(Debug, Clone)]
struct Board {
    tiles: Vec<Vec<Tile>>,
    called: bool,
}

fn calculate_score(board: &mut Vec<Vec<Tile>>, number: i32) -> Result<(), Box<dyn Error>> {
    let mut sum = 0;
    for tile_row in &mut board.into_iter() {
        for tile in &mut tile_row.into_iter() {
            if !tile.called {
                sum += tile.number;
            }
        }
    }

    println!("{}", sum * number);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let input: Vec<String> = fs::read_to_string("input")?
        .lines()
        .map(|s| s.to_owned())
        .collect();

    let numbers: Vec<i32> = input[0]
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let mut boards = Vec::new();

    let boards_raw: Vec<String> = fs::read_to_string("input")?
        .split("\n\n")
        .map(|s| s.to_owned())
        .collect();

    //println!("{:?}", boards_raw);

    for board_raw in &boards_raw[1..] {
        let mut board = Vec::new();
        for line in board_raw.lines() {
            let mut tile_row = Vec::new();
            for number in line.split_whitespace() {
                tile_row.push(Tile {
                    number: number.parse::<i32>().unwrap(),
                    called: false,
                });
            }
            board.push(tile_row);
        }
        boards.push(Board {
            tiles: board,
            called: false,
        });
    }

    //println!("{:?}", boards);

    let mut called_boards = 0;
    let boards_count = boards.len();

    for number in numbers {
        for board in boards.iter_mut() {
            for tile_row in board.tiles.iter_mut() {
                for tile in tile_row.iter_mut() {
                    if tile.number == number {
                        tile.called = true;
                    }
                }
            }
        }

        for board in boards.iter_mut() {
            'inner_row: for tile_row in board.tiles.iter_mut() {
                for tile in tile_row.iter_mut() {
                    if !tile.called {
                        continue 'inner_row;
                    }
                }
                if !board.called {
                    called_boards += 1;
                    board.called = true;
                    if called_boards == boards_count {
                        return calculate_score(&mut board.tiles, number);
                    }
                }
            }
            'inner_col: for i in 0..board.tiles[0].len() {
                for tile_row in board.tiles.iter_mut() {
                    if !tile_row[i].called {
                        continue 'inner_col;
                    }
                }
                if !board.called {
                    called_boards += 1;
                    board.called = true;
                    if called_boards == boards_count {
                        return calculate_score(&mut board.tiles, number);
                    }
                }
            }
        }
    }

    Ok(())
}
