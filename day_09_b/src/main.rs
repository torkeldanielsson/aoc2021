use std::{collections::HashMap, error::Error, fs};

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

    let mut basin_sizes = Vec::new();

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
                let mut points_in_basin = HashMap::new();
                points_in_basin.insert((x as i32, y as i32), ());

                let mut did_insert = true;
                while did_insert {
                    did_insert = false;
                    let mut new_candidates = HashMap::new();
                    for p in points_in_basin.keys() {
                        for delta_coord in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
                            let new_candidate = (p.0 + delta_coord.0, p.1 + delta_coord.1);
                            if new_candidate.0 >= 0
                                && new_candidate.0 < x_max as i32
                                && new_candidate.1 >= 0
                                && new_candidate.1 < y_max as i32
                            {
                                new_candidates.insert(new_candidate.clone(), ());
                            }
                        }
                    }

                    for candidate in new_candidates {
                        let candidate = &candidate.0;
                        if !points_in_basin.contains_key(candidate)
                            && map[candidate.1 as usize][candidate.0 as usize] != 9
                        {
                            points_in_basin.insert(*candidate, ());
                            did_insert = true;
                        }
                    }
                }

                basin_sizes.push(points_in_basin.len());

                // println!("{:?} {}", points_in_basin, points_in_basin.len());
            }
        }
    }

    basin_sizes.sort();
    basin_sizes.reverse();

    println!("{}", basin_sizes[0] * basin_sizes[1] * basin_sizes[2]);

    Ok(())
}
