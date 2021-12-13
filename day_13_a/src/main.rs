use std::{collections::HashSet, error::Error, fs};

use glam::{ivec2, IVec2};

fn main() -> Result<(), Box<dyn Error>> {
    let input: Vec<String> = fs::read_to_string("input")?
        .split("\n\n")
        .map(|s| s.to_owned())
        .collect();

    let mut map: HashSet<IVec2> = HashSet::new();
    for line in input[0].lines() {
        let coords: Vec<i32> = line.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
        map.insert(ivec2(coords[0], coords[1]));
    }

    println!("{}", map.len());

    for fold_instruction in input[1].lines() {
        let parts: Vec<&str> = fold_instruction.split("=").collect();
        let axis: char = parts[0].chars().nth(11).unwrap();
        let coord = parts[1].parse::<i32>().unwrap();

        println!("{} {}", axis, coord);

        let old_map = map.clone();
        map.clear();
        for mut p in old_map {
            if axis == 'x' {
                if p.x < coord {
                    p.x = -(p.x - coord) + coord;
                }
            } else {
                if p.y < coord {
                    p.y = -(p.y - coord) + coord;
                }
            }
            map.insert(p);
        }

        println!("{}", map.len());

        return Ok(());
    }

    Ok(())
}
