use glam::{const_ivec2, IVec2};
use std::{collections::HashMap, collections::HashSet, error::Error, fs};

fn print_map(map: &HashMap<IVec2, i32>) {
    let size = 10;

    for y in 0..size {
        for x in 0..size {
            print!("{}", map[&const_ivec2!([x, y])]);
        }
        println!();
    }

    println!();
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut map = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert(
                const_ivec2!([x as i32, y as i32]),
                c.to_string().parse::<i32>().unwrap(),
            );
        }
    }

    print_map(&map);

    let mut flashes = 0;

    for step in 0..10000 {
        for val in map.iter_mut() {
            *val.1 += 1;
        }

        let mut flashed = HashSet::new();
        let mut did_flash = true;
        while did_flash {
            did_flash = false;
            let stupid_map_clone = map.clone();
            for (coord, val) in stupid_map_clone {
                if val >= 10 && !flashed.contains(&coord) {
                    flashed.insert(coord);
                    for neighbour in &[
                        const_ivec2!([-1, 1]),
                        const_ivec2!([0, 1]),
                        const_ivec2!([1, 1]),
                        const_ivec2!([-1, 0]),
                        const_ivec2!([1, 0]),
                        const_ivec2!([-1, -1]),
                        const_ivec2!([0, -1]),
                        const_ivec2!([1, -1]),
                    ] {
                        let neighbour = coord + *neighbour;
                        if map.contains_key(&neighbour) {
                            *map.get_mut(&neighbour).unwrap() += 1;
                            did_flash = true;
                        }
                    }
                }
            }
        }

        for val in map.iter_mut() {
            if *val.1 > 9 {
                *val.1 = 0;
                flashes += 1;
            }
        }

        println!("After step {}: {}", step + 1, flashes);
        print_map(&map);

        let mut all_zeroes = true;

        for val in map.iter_mut() {
            if *val.1 != 0 {
                all_zeroes = false;
            }
        }

        if all_zeroes {
            break;
        }
    }

    Ok(())
}
