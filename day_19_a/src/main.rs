use glam::{ivec3, vec3};
use std::{collections::HashSet, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;
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

    let mut globbed = scanners[0].clone();
    scanners.remove(0);

    while scanners.len() != 0 {
        'main: for i2 in 0..scanners.len() {
            let scanner2 = &scanners[i2];
            let mut dupes = Vec::new();
            for diff1 in globbed.1.iter().enumerate() {
                for diff2 in scanner2.1.iter().enumerate() {
                    let diff = diff1.1.intersection(diff2.1);
                    let count = diff.count();
                    if count > 10 {
                        dupes.push((diff1.0, diff2.0));
                    }
                }
            }
            if dupes.len() > 10 {
                for axis_permut in &[
                    (0, 1, 2),
                    (1, 2, 0),
                    (2, 0, 1),
                    (0, 2, 1),
                    (2, 1, 0),
                    (1, 0, 2),
                ] {
                    for sign_permut in &[
                        (1, 1, 1),
                        (1, 1, -1),
                        (1, -1, 1),
                        (1, -1, -1),
                        (-1, 1, 1),
                        (-1, 1, -1),
                        (-1, -1, 1),
                        (-1, -1, -1),
                    ] {
                        let mut permutated_scanner2 = scanner2.clone();
                        for coord in &mut permutated_scanner2.0 {
                            let copy = coord.clone();
                            coord[0] = sign_permut.0 * copy[axis_permut.0];
                            coord[1] = sign_permut.1 * copy[axis_permut.1];
                            coord[2] = sign_permut.2 * copy[axis_permut.2];
                        }
                        let translation = globbed.0[dupes[0].0] - permutated_scanner2.0[dupes[0].1];
                        for coord in &mut permutated_scanner2.0 {
                            *coord += translation;
                        }

                        let mut all_ok = true;
                        for dupe in &dupes {
                            if globbed.0[dupe.0] != permutated_scanner2.0[dupe.1] {
                                all_ok = false;
                            }
                        }
                        if all_ok {
                            println!("GOT {}", i2);
                            for coord in permutated_scanner2.0.iter().enumerate() {
                                let mut is_dupe = false;
                                for dupe in &dupes {
                                    if coord.0 == dupe.1 {
                                        is_dupe = true;
                                    }
                                }
                                if !is_dupe {
                                    globbed.0.push(*coord.1);
                                }
                            }
                            globbed.1.clear();
                            for i1 in 0..globbed.0.len() {
                                let mut distances = HashSet::new();
                                for i2 in 0..globbed.0.len() {
                                    if i1 != i2 {
                                        let diff = globbed.0[i1] - globbed.0[i2];
                                        let dist =
                                            vec3(diff.x as f32, diff.y as f32, diff.z as f32)
                                                .length();
                                        distances.insert((dist * 1000000000.0) as i64);
                                    }
                                }
                                globbed.1.push(distances);
                            }
                            scanners.remove(i2);
                            break 'main;
                        }
                    }
                }
            }
        }
    }

    // 334 is too low
    // 388 is too high
    println!("beacons: {}", globbed.0.len());

    Ok(())
}
