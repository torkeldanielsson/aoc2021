use std::{collections::HashSet, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let mut cubes = HashSet::new();

    for line in fs::read_to_string("input")?.lines() {
        println!("line: {}", line);

        let parts: Vec<&str> = line.split(" ").collect();

        let raw_ranges: Vec<&str> = parts[1].split(",").collect();
        let mut ranges = Vec::new();
        for rr in raw_ranges {
            let parts: Vec<&str> = rr.split("=").collect();
            let range: Vec<i32> = parts[1]
                .split("..")
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
            ranges.push((range[0], range[1]));
        }

        for range in &mut ranges {
            if range.0 < -50 {
                range.0 = -50;
            }
            if range.1 > 50 {
                range.1 = 50;
            }
        }

        if parts[0] == "on" {
            for x in ranges[0].0..=ranges[0].1 {
                for y in ranges[1].0..=ranges[1].1 {
                    for z in ranges[2].0..=ranges[2].1 {
                        if x >= -50 && x <= 50 && y >= -50 && y <= 50 && z >= -50 && z <= 50 {
                            cubes.insert((x, y, z));
                        }
                    }
                }
            }
        } else {
            for x in ranges[0].0..=ranges[0].1 {
                for y in ranges[1].0..=ranges[1].1 {
                    for z in ranges[2].0..=ranges[2].1 {
                        if x >= -50 && x <= 50 && y >= -50 && y <= 50 && z >= -50 && z <= 50 {
                            cubes.remove(&(x, y, z));
                        }
                    }
                }
            }
        }
        println!("res: {}", cubes.len());
    }

    Ok(())
}
