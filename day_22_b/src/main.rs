use std::{collections::BTreeSet, error::Error, fs};

fn split(
    mut cubes: BTreeSet<((i64, i64), (i64, i64), (i64, i64))>,
    splits: (Vec<i64>, Vec<i64>, Vec<i64>),
) -> BTreeSet<((i64, i64), (i64, i64), (i64, i64))> {
    {
        println!("split x {}", splits.0.len());

        let mut iters = 0;
        let mut adds = 0;
        let mut removes = 0;

        let mut did_split = true;
        while did_split {
            did_split = false;

            let mut to_remove = BTreeSet::new();
            let mut to_add = BTreeSet::new();

            'cube_match_x: for cube in &cubes {
                for split in &splits.0 {
                    if cube.0 .0 < *split && cube.0 .1 >= *split {
                        to_add.insert(((cube.0 .0, *split - 1), cube.1, cube.2));
                        to_add.insert(((*split, cube.0 .1), cube.1, cube.2));
                        to_remove.insert(*cube);
                        did_split = true;
                        continue 'cube_match_x;
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
        println!("iters {}, adds: {}, removes: {}", iters, adds, removes);
    }

    {
        println!("split y {}", splits.1.len());

        let mut iters = 0;
        let mut adds = 0;
        let mut removes = 0;

        let mut did_split = true;
        while did_split {
            did_split = false;

            let mut to_remove = BTreeSet::new();
            let mut to_add = BTreeSet::new();

            'cube_match_y: for cube in &cubes {
                for split in &splits.1 {
                    if cube.1 .0 < *split && cube.1 .1 >= *split {
                        to_add.insert((cube.0, (cube.1 .0, *split - 1), cube.2));
                        to_add.insert((cube.0, (*split, cube.1 .1), cube.2));
                        to_remove.insert(*cube);
                        did_split = true;
                        continue 'cube_match_y;
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
        println!("iters {}, adds: {}, removes: {}", iters, adds, removes);
    }

    {
        println!("split z {}", splits.2.len());

        let mut iters = 0;
        let mut adds = 0;
        let mut removes = 0;

        let mut did_split = true;
        while did_split {
            did_split = false;

            let mut to_remove = BTreeSet::new();
            let mut to_add = BTreeSet::new();

            'cube_match_z: for cube in &cubes {
                for split in &splits.2 {
                    if cube.2 .0 < *split && cube.2 .1 >= *split {
                        to_add.insert((cube.0, cube.1, (cube.2 .0, *split - 1)));
                        to_add.insert((cube.0, cube.1, (*split, cube.2 .1)));
                        to_remove.insert(*cube);
                        did_split = true;
                        continue 'cube_match_z;
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
        println!("iters {}, adds: {}, removes: {}", iters, adds, removes);
    }

    println!("split done, joining");

    {
        // Join

        let mut did_join = true;

        while did_join {
            did_join = false;

            let mut to_remove = BTreeSet::new();
            let mut to_add = BTreeSet::new();

            'outer: for c1 in &cubes {
                for c2 in &cubes {
                    if c1 != c2 {
                        if c1.0 .1 + 1 == c2.0 .0 && c1.1 == c2.1 && c1.2 == c2.2 {
                            to_remove.insert(*c1);
                            to_remove.insert(*c2);
                            to_add.insert(((c1.0 .0, c2.0 .1), c1.1, c1.2));
                            did_join = true;
                            break 'outer;
                        }
                        if c1.0 .0 + 1 == c2.0 .1 && c1.1 == c2.1 && c1.2 == c2.2 {
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
                        if c1.1 .0 + 1 == c2.1 .1 && c1.0 == c2.0 && c1.2 == c2.2 {
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
                        if c1.2 .0 + 1 == c2.2 .1 && c1.1 == c2.1 && c1.0 == c2.0 {
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
    }

    println!("join done");

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

fn main() -> Result<(), Box<dyn Error>> {
    let mut cubes: BTreeSet<((i64, i64), (i64, i64), (i64, i64))> = BTreeSet::new();

    for line in fs::read_to_string("input_test")?.lines() {
        println!("line: {}", line);

        let parts: Vec<&str> = line.split(" ").collect();

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

        let mut fresh: BTreeSet<((i64, i64), (i64, i64), (i64, i64))> = BTreeSet::new();
        fresh.insert((
            (ranges[0].0, ranges[0].1),
            (ranges[1].0, ranges[1].1),
            (ranges[2].0, ranges[2].1),
        ));

        println!("hmm 0 ?");

        cubes = split(cubes, split_points(fresh.clone()));

        println!("hmm 0.5 ?");

        fresh = split(fresh, split_points(cubes.clone()));

        if parts[0] == "on" {
            cubes.extend(fresh);
        } else {
            for f in fresh {
                // println!("removing: {:?}", f);
                cubes.remove(&f);
            }
        }

        // println!("{:?}", cubes);

        let mut volume: i64 = 0;

        for v in &cubes {
            volume += (v.0 .1 - v.0 .0 + 1) * (v.1 .1 - v.1 .0 + 1) * (v.2 .1 - v.2 .0 + 1);
        }

        println!("cube count: {}, volume: {:?}\n", cubes.len(), volume);
    }

    Ok(())
}
