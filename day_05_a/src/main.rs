use std::{collections::HashMap, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let mut map = HashMap::new();
    for line in fs::read_to_string("input")?.lines() {
        let line: Vec<(i32, i32)> = line
            .split(" -> ")
            .map(|s| {
                let parts: Vec<i32> = s.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
                (parts[0], parts[1])
            })
            .collect();

        if line[0].0 == line[1].0 {
            let x = line[0].0;
            for y in std::cmp::min(line[0].1, line[1].1)..=std::cmp::max(line[0].1, line[1].1) {
                if map.contains_key(&(x, y)) {
                    let old_val = map[&(x, y)];
                    map.insert((x, y), old_val + 1);
                } else {
                    map.insert((x, y), 1);
                }
            }
        }

        if line[0].1 == line[1].1 {
            let y = line[0].1;
            for x in std::cmp::min(line[0].0, line[1].0)..=std::cmp::max(line[0].0, line[1].0) {
                if map.contains_key(&(x, y)) {
                    let old_val = map[&(x, y)];
                    map.insert((x, y), old_val + 1);
                } else {
                    map.insert((x, y), 1);
                }
            }
        }
    }

    let mut risk_areas = 0;

    for v in map.values() {
        if *v > 1 {
            risk_areas += 1;
        }
    }

    println!("{:?}", risk_areas);

    Ok(())
}
