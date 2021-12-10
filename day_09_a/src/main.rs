use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let mut map: Vec<Vec<u8>> = Vec::new();

    for line in fs::read_to_string("input")?.lines() {
        let mut map_line = Vec::new();

        for c in line.chars() {
            map_line.push(c.to_string().parse::<u8>().unwrap());
        }

        map.push(map_line);
    }

    let y_max = map.len();
    let x_max = map[0].len();

    let mut risk_level: i32 = 0;

    for y in 0..y_max {
        for x in 0..x_max {
            let mut is_low_point = true;
            if y != 0 && map[y - 1][x] <= map[y][x] {
                is_low_point = false;
            }
            if y + 1 < y_max && map[y + 1][x] <= map[y][x] {
                is_low_point = false;
            }
            if x != 0 && map[y][x - 1] <= map[y][x] {
                is_low_point = false;
            }
            if x + 1 < x_max && map[y][x + 1] <= map[y][x] {
                is_low_point = false;
            }

            if is_low_point {
                // println!("{} {} is low", x, y);
                risk_level += 1 + map[y][x] as i32;
            }
        }
    }

    println!("{}", risk_level);

    Ok(())
}
