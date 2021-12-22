use glam::{ivec3, vec3};
use std::{collections::HashSet, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input_test")?;
    let scanner_strs: Vec<&str> = input.split("\n\n--- scanner ").collect();

    let mut scanners = Vec::new();

    for scanstr in scanner_strs {
        let mut lines: Vec<&str> = scanstr.lines().collect();
        lines.remove(0);
        let mut positions = Vec::new();
        for line in lines {
            let parts: Vec<&str> = line.split(",").collect();
            positions.push(ivec3(
                parts[0].parse::<i32>().unwrap(),
                parts[1].parse::<i32>().unwrap(),
                parts[2].parse::<i32>().unwrap(),
            ));
        }
        let mut distances_vec = Vec::new();
        for i1 in 0..positions.len() {
            let mut distances = HashSet::new();
            for i2 in 0..positions.len() {
                if i1 != i2 {
                    let diff = positions[i1] - positions[i2];
                    let dist = vec3(diff.x as f32, diff.y as f32, diff.z as f32).length();
                    distances.insert((dist * 1000000000.0) as i64);
                }
            }
            distances_vec.push(distances);
        }

        scanners.push((positions, distances_vec));
    }

    let mut beacon_count = 0;

    for i1 in 0..scanners.len() {
        beacon_count += scanners[i1].0.len();
        for i2 in i1..scanners.len() {
            let scanner1 = &scanners[i1];
            let scanner2 = &scanners[i2];
            let mut dupes = Vec::new();
            if i1 != i2 {
                for diff1 in scanner1.1.iter().enumerate() {
                    for diff2 in scanner2.1.iter().enumerate() {
                        let diff = diff1.1.intersection(diff2.1);
                        let count = diff.count();
                        if count > 10 {
                            dupes.push((diff1.0, diff2.0));
                        }
                    }
                }
            }
            if dupes.len() > 10 {
                println!(
                    "scanner {} - {}: {} matches, {:?}",
                    i1,
                    i2,
                    dupes.len(),
                    dupes
                );
                beacon_count -= dupes.len();
            }
        }
    }

    // 334 is too low?
    println!("beacons: {}", beacon_count);

    Ok(())
}
