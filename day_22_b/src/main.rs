use std::{collections::BTreeSet, error::Error, fs};

fn split(
    mut cubes: BTreeSet<((i64, i64), (i64, i64), (i64, i64))>,
    splits: (Vec<i64>, Vec<i64>, Vec<i64>),
) -> BTreeSet<((i64, i64), (i64, i64), (i64, i64))> {
    let mut iters = 0;
    let mut adds = 0;
    let mut removes = 0;

    let mut split_limits = (
        (i64::MAX, i64::MIN),
        (i64::MAX, i64::MIN),
        (i64::MAX, i64::MIN),
    );
    for x_split in &splits.0 {
        if split_limits.0 .0 > *x_split {
            split_limits.0 .0 = *x_split;
        }
        if split_limits.0 .1 < *x_split {
            split_limits.0 .1 = *x_split;
        }
    }
    for y_split in &splits.1 {
        if split_limits.1 .0 > *y_split {
            split_limits.1 .0 = *y_split;
        }
        if split_limits.1 .1 < *y_split {
            split_limits.1 .1 = *y_split;
        }
    }
    for z_split in &splits.2 {
        if split_limits.2 .0 > *z_split {
            split_limits.2 .0 = *z_split;
        }
        if split_limits.2 .1 < *z_split {
            split_limits.2 .1 = *z_split;
        }
    }

    {
        let mut did_split = true;
        while did_split {
            did_split = false;

            let mut to_remove = BTreeSet::new();
            let mut to_add = BTreeSet::new();

            'cube_match_x: for cube in &cubes {
                for split in &splits.0 {
                    if cube.0 .0 < *split && cube.0 .1 >= *split {
                        let mut hit_y = false;
                        for splity in &splits.1 {
                            if cube.1 .0 < *splity && cube.1 .1 >= *splity {
                                hit_y = true;
                                break;
                            }
                        }
                        if (split_limits.1 .0 <= cube.1 .0 && split_limits.1 .1 >= cube.1 .0)
                            || (split_limits.1 .0 <= cube.1 .1 && split_limits.1 .1 >= cube.1 .1)
                        {
                            hit_y = true;
                        }
                        let mut hit_z = false;
                        for splitz in &splits.2 {
                            if cube.2 .0 < *splitz && cube.2 .1 >= *splitz {
                                hit_z = true;
                                break;
                            }
                        }
                        if (split_limits.2 .0 <= cube.2 .0 && split_limits.2 .1 >= cube.2 .0)
                            || (split_limits.2 .0 <= cube.2 .1 && split_limits.2 .1 >= cube.2 .1)
                        {
                            hit_z = true;
                        }

                        if hit_y && hit_z {
                            to_add.insert(((cube.0 .0, *split - 1), cube.1, cube.2));
                            to_add.insert(((*split, cube.0 .1), cube.1, cube.2));
                            to_remove.insert(*cube);
                            did_split = true;
                            continue 'cube_match_x;
                        }
                    }
                }
            }

            adds += to_add.len();
            removes += to_remove.len();

            cubes.append(&mut to_add);
            for r in to_remove {
                cubes.remove(&r);
            }

            iters += 1;
        }
    }

    {
        let mut did_split = true;
        while did_split {
            did_split = false;

            let mut to_remove = BTreeSet::new();
            let mut to_add = BTreeSet::new();

            'cube_match_y: for cube in &cubes {
                for split in &splits.1 {
                    if cube.1 .0 < *split && cube.1 .1 >= *split {
                        let mut hit_x = false;
                        for splitx in &splits.0 {
                            if cube.0 .0 < *splitx && cube.0 .1 >= *splitx {
                                hit_x = true;
                                break;
                            }
                        }
                        if (split_limits.0 .0 <= cube.0 .0 && split_limits.0 .1 >= cube.0 .0)
                            || (split_limits.0 .0 <= cube.0 .1 && split_limits.0 .1 >= cube.0 .1)
                        {
                            hit_x = true;
                        }
                        let mut hit_z = false;
                        for splitz in &splits.2 {
                            if cube.2 .0 < *splitz && cube.2 .1 >= *splitz {
                                hit_z = true;
                                break;
                            }
                        }
                        if (split_limits.2 .0 <= cube.2 .0 && split_limits.2 .1 >= cube.2 .0)
                            || (split_limits.2 .0 <= cube.2 .1 && split_limits.2 .1 >= cube.2 .1)
                        {
                            hit_z = true;
                        }

                        if hit_x && hit_z {
                            to_add.insert((cube.0, (cube.1 .0, *split - 1), cube.2));
                            to_add.insert((cube.0, (*split, cube.1 .1), cube.2));
                            to_remove.insert(*cube);
                            did_split = true;
                            continue 'cube_match_y;
                        }
                    }
                }
            }

            adds += to_add.len();
            removes += to_remove.len();

            cubes.append(&mut to_add);
            for r in to_remove {
                cubes.remove(&r);
            }

            iters += 1;
        }
    }

    {
        let mut did_split = true;
        while did_split {
            did_split = false;

            let mut to_remove = BTreeSet::new();
            let mut to_add = BTreeSet::new();

            'cube_match_z: for cube in &cubes {
                for split in &splits.2 {
                    if cube.2 .0 < *split && cube.2 .1 >= *split {
                        let mut hit_x = false;
                        for splitx in &splits.0 {
                            if cube.0 .0 < *splitx && cube.0 .1 >= *splitx {
                                hit_x = true;
                                break;
                            }
                        }
                        if (split_limits.0 .0 <= cube.0 .0 && split_limits.0 .1 >= cube.0 .0)
                            || (split_limits.0 .0 <= cube.0 .1 && split_limits.0 .1 >= cube.0 .1)
                        {
                            hit_x = true;
                        }
                        let mut hit_y = false;
                        for splity in &splits.1 {
                            if cube.1 .0 < *splity && cube.1 .1 >= *splity {
                                hit_y = true;
                                break;
                            }
                        }
                        if (split_limits.1 .0 <= cube.1 .0 && split_limits.1 .1 >= cube.1 .0)
                            || (split_limits.1 .0 <= cube.1 .1 && split_limits.1 .1 >= cube.1 .1)
                        {
                            hit_y = true;
                        }
                        if hit_x && hit_y {
                            to_add.insert((cube.0, cube.1, (cube.2 .0, *split - 1)));
                            to_add.insert((cube.0, cube.1, (*split, cube.2 .1)));
                            to_remove.insert(*cube);
                            did_split = true;
                            continue 'cube_match_z;
                        }
                    }
                }
            }

            adds += to_add.len();
            removes += to_remove.len();

            cubes.append(&mut to_add);
            for r in to_remove {
                cubes.remove(&r);
            }

            iters += 1;
        }
    }

    /*
    println!(
        "split: iters {}, adds: {}, removes: {}",
        iters, adds, removes
    );*/

    cubes
}

fn split_points(
    cubes: BTreeSet<((i64, i64), (i64, i64), (i64, i64))>,
) -> (Vec<i64>, Vec<i64>, Vec<i64>) {
    let mut x = Vec::new();
    let mut y = Vec::new();
    let mut z = Vec::new();

    for cube in cubes {
        x.push(cube.0 .0);
        x.push(cube.0 .1 + 1);
        y.push(cube.1 .0);
        y.push(cube.1 .1 + 1);
        z.push(cube.2 .0);
        z.push(cube.2 .1 + 1);
    }

    (x, y, z)
}

fn join(
    mut cubes: BTreeSet<((i64, i64), (i64, i64), (i64, i64))>,
) -> BTreeSet<((i64, i64), (i64, i64), (i64, i64))> {
    let mut did_join = true;

    while did_join {
        did_join = false;

        let mut to_remove = BTreeSet::new();
        let mut to_add = BTreeSet::new();

        let cube_vec: Vec<((i64, i64), (i64, i64), (i64, i64))> =
            cubes.clone().into_iter().collect();

        if cube_vec.len() > 0 {
            'outer: for i1 in 0..cube_vec.len() - 1 {
                for i2 in i1 + 1..cube_vec.len() {
                    let c1 = &cube_vec[i1];
                    let c2 = &cube_vec[i2];
                    if c1.0 .1 + 1 == c2.0 .0 && c1.1 == c2.1 && c1.2 == c2.2 {
                        to_remove.insert(*c1);
                        to_remove.insert(*c2);
                        to_add.insert(((c1.0 .0, c2.0 .1), c1.1, c1.2));
                        did_join = true;
                        break 'outer;
                    }
                    if c1.0 .0 == c2.0 .1 + 1 && c1.1 == c2.1 && c1.2 == c2.2 {
                        to_remove.insert(*c1);
                        to_remove.insert(*c2);
                        to_add.insert(((c2.0 .0, c1.0 .1), c1.1, c1.2));
                        did_join = true;
                        break 'outer;
                    }

                    if c1.1 .1 + 1 == c2.1 .0 && c1.0 == c2.0 && c1.2 == c2.2 {
                        to_remove.insert(*c1);
                        to_remove.insert(*c2);
                        to_add.insert((c1.0, (c1.1 .0, c2.1 .1), c1.2));
                        did_join = true;
                        break 'outer;
                    }
                    if c1.1 .0 == c2.1 .1 + 1 && c1.0 == c2.0 && c1.2 == c2.2 {
                        to_remove.insert(*c1);
                        to_remove.insert(*c2);
                        to_add.insert((c1.0, (c2.1 .0, c1.1 .1), c1.2));
                        did_join = true;
                        break 'outer;
                    }

                    if c1.2 .1 + 1 == c2.2 .0 && c1.1 == c2.1 && c1.0 == c2.0 {
                        to_remove.insert(*c1);
                        to_remove.insert(*c2);
                        to_add.insert((c1.0, c1.1, (c1.2 .0, c2.2 .1)));
                        did_join = true;
                        break 'outer;
                    }
                    if c1.2 .0 == c2.2 .1 + 1 && c1.1 == c2.1 && c1.0 == c2.0 {
                        to_remove.insert(*c1);
                        to_remove.insert(*c2);
                        to_add.insert((c1.0, c1.1, (c2.2 .0, c1.2 .1)));
                        did_join = true;
                        break 'outer;
                    }
                }
            }
        }

        cubes.append(&mut to_add);
        for r in to_remove {
            cubes.remove(&r);
        }
    }

    cubes
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut cubes: BTreeSet<((i64, i64), (i64, i64), (i64, i64))> = BTreeSet::new();

    let input = fs::read_to_string("input_test")?;

    let lines: Vec<&str> = input.lines().collect();

    let line_count = lines.len();

    for line in lines.iter().enumerate() {
        println!("line {}/{}: {}", line.0 + 1, line_count, line.1);

        let parts: Vec<&str> = line.1.split(" ").collect();

        let raw_ranges: Vec<&str> = parts[1].split(",").collect();
        // [(x_min, x_max), (y_min, y_max), (z_min, z_max)], inclusive!
        let mut ranges = Vec::new();
        for rr in raw_ranges {
            let parts: Vec<&str> = rr.split("=").collect();
            let range: Vec<i64> = parts[1]
                .split("..")
                .map(|s| s.parse::<i64>().unwrap())
                .collect();
            ranges.push((range[0], range[1]));
        }

        let fresh_one = (
            (ranges[0].0, ranges[0].1),
            (ranges[1].0, ranges[1].1),
            (ranges[2].0, ranges[2].1),
        );

        let mut fresh: BTreeSet<((i64, i64), (i64, i64), (i64, i64))> = BTreeSet::new();
        fresh.insert(fresh_one);

        cubes = split(cubes, split_points(fresh.clone()));

        fresh = split(fresh, split_points(cubes.clone()));

        cubes = split(cubes, split_points(fresh.clone()));

        if parts[0] == "on" {
            cubes.extend(fresh);
        } else {
            for f in fresh {
                // println!("removing: {:?}", f);
                cubes.remove(&f);
            }
        }

        let mut volume: i64 = 0;

        for v in &cubes {
            volume += (v.0 .1 - v.0 .0 + 1) * (v.1 .1 - v.1 .0 + 1) * (v.2 .1 - v.2 .0 + 1);
        }

        //println!("cubes: {:?}", cubes);

        println!("cube count: {}, volume: {:?}", cubes.len(), volume);

        cubes = join(cubes);

        //println!("cubes after: {:?}", cubes);

        println!();
    }

    Ok(())
}
