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

        let mut start = (0, 0);
        let mut end = (0, 0);
        let mut step = (0, 0);

        if line[0].0 == line[1].0 {
            start.0 = line[0].0;
            end.0 = line[0].0;
            start.1 = std::cmp::min(line[0].1, line[1].1);
            end.1 = std::cmp::max(line[0].1, line[1].1);
            step.1 = 1;
        } else if line[0].1 == line[1].1 {
            start.0 = std::cmp::min(line[0].0, line[1].0);
            end.0 = std::cmp::max(line[0].0, line[1].0);
            start.1 = line[0].1;
            end.1 = line[0].1;
            step.0 = 1;
        } else {
            if line[0].0 < line[1].0 {
                start = line[0];
                end = line[1];
            } else {
                start = line[1];
                end = line[0];
            }
            if start.1 < end.1 {
                step = (1, 1);
            } else {
                step = (1, -1);
            }
        }
        let mut p = start;
        end.0 += step.0;
        end.1 += step.1;

        while p != end {
            if map.contains_key(&p) {
                let old_val = map[&p];
                map.insert(p, old_val + 1);
            } else {
                map.insert(p, 1);
            }

            p.0 += step.0;
            p.1 += step.1;
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
