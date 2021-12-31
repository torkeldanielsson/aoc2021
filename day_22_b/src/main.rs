use std::{collections::BTreeSet, error::Error, fs};

#[derive(Hash, Eq, PartialEq, PartialOrd, Ord, Clone, Copy)]
struct Dim {
    min: i32,
    max: i32,
}

#[derive(Hash, Eq, PartialEq, PartialOrd, Ord, Clone, Copy)]
struct Cube {
    d: [Dim; 3],
}

fn overlaps(a: &Cube, b: &Cube) -> bool {
    let mut overlap_count = 0;
    for d in 0..3 {
        if (a.d[d].max > b.d[d].min && a.d[d].max < b.d[d].max)
            || (a.d[d].min > b.d[d].min && a.d[d].min < b.d[d].max)
            || (b.d[d].max > a.d[d].min && b.d[d].max < a.d[d].max)
            || (b.d[d].min > a.d[d].min && b.d[d].min < a.d[d].max)
        {
            overlap_count += 1;
        }
    }

    overlap_count == 3
}

fn contains(a: &Cube, b: &Cube) -> bool {
    let mut contain_count = 0;
    for d in 0..3 {
        if a.d[d].max >= b.d[d].max && a.d[d].min <= b.d[d].min {
            contain_count += 1;
        }
    }

    contain_count == 3
}

fn split(mut cubes: BTreeSet<Cube>, splits: [Vec<i32>; 3]) -> BTreeSet<Cube> {
    for i in 0..3 {
        let mut did_split = true;
        while did_split {
            did_split = false;

            let mut to_remove = BTreeSet::new();
            let mut to_add = BTreeSet::new();

            'cube_match: for cube in &cubes {
                for split in &splits[i] {
                    if cube.d[i].min < *split && cube.d[i].max > *split {
                        let mut new_cube_1 = cube.clone();
                        new_cube_1.d[i].max = *split;

                        let mut new_cube_2 = cube.clone();
                        new_cube_2.d[i].min = *split;

                        to_add.insert(new_cube_1);
                        to_add.insert(new_cube_2);
                        to_remove.insert(*cube);
                        did_split = true;
                        continue 'cube_match;
                    }
                }
            }

            cubes.extend(to_add);
            for r in to_remove {
                cubes.remove(&r);
            }
        }
    }

    cubes
}

fn split_points(cubes: BTreeSet<Cube>) -> [Vec<i32>; 3] {
    let mut res = [Vec::new(), Vec::new(), Vec::new()];
    for cube in cubes {
        for i in 0..3 {
            res[i].push(cube.d[i].min);
            res[i].push(cube.d[i].max);
        }
    }

    res
}

fn join(mut cubes: BTreeSet<Cube>) -> BTreeSet<Cube> {
    let mut did_join = true;

    let mut iters = 0;

    while did_join && iters < 3 {
        iters += 1;
        did_join = false;

        let mut to_remove = Vec::new();
        let mut to_add = Vec::new();

        let cube_vec: Vec<Cube> = cubes.clone().into_iter().collect();

        let mut join_touched = BTreeSet::new();

        if cube_vec.len() > 0 {
            for i1 in 0..cube_vec.len() - 1 {
                for i2 in i1 + 1..cube_vec.len() {
                    let c1 = &cube_vec[i1];
                    let c2 = &cube_vec[i2];

                    if !join_touched.contains(c1) && !join_touched.contains(c2) {
                        for i in 0..3 {
                            {
                                let mut can_extend = true;
                                if c1.d[i].max != c2.d[i].min {
                                    can_extend = false;
                                }
                                if can_extend {
                                    for ii in 0..3 {
                                        if i != ii && c1.d[ii] != c2.d[ii] {
                                            can_extend = false;
                                            break;
                                        }
                                    }
                                    if can_extend {
                                        to_remove.push(*c1);
                                        to_remove.push(*c2);

                                        join_touched.insert(c1.clone());
                                        join_touched.insert(c2.clone());

                                        let mut new_cube = c1.clone();
                                        new_cube.d[i].max = c2.d[i].max;

                                        to_add.push(new_cube);
                                        did_join = true;
                                    }
                                }
                            }

                            {
                                let mut can_extend = true;
                                if c2.d[i].max != c1.d[i].min {
                                    can_extend = false;
                                }
                                if can_extend {
                                    for ii in 0..3 {
                                        if i != ii && c1.d[ii] != c2.d[ii] {
                                            can_extend = false;
                                            break;
                                        }
                                    }
                                    if can_extend {
                                        to_remove.push(*c1);
                                        to_remove.push(*c2);

                                        join_touched.insert(c1.clone());
                                        join_touched.insert(c2.clone());

                                        let mut new_cube = c2.clone();
                                        new_cube.d[i].max = c1.d[i].max;

                                        to_add.push(new_cube);
                                        did_join = true;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        cubes.extend(to_add);
        for r in to_remove {
            cubes.remove(&r);
        }
    }

    println!("join iters: {}", iters);

    cubes
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut cubes: BTreeSet<Cube> = BTreeSet::new();

    let input = fs::read_to_string("input")?;

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
            let range: Vec<i32> = parts[1]
                .split("..")
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
            ranges.push((range[0], range[1]));
        }

        let fresh_one = Cube {
            d: [
                Dim {
                    min: ranges[0].0,
                    max: ranges[0].1 + 1,
                },
                Dim {
                    min: ranges[1].0,
                    max: ranges[1].1 + 1,
                },
                Dim {
                    min: ranges[2].0,
                    max: ranges[2].1 + 1,
                },
            ],
        };

        cubes.retain(|c| !contains(&fresh_one, c));

        let mut fresh: BTreeSet<Cube> = BTreeSet::new();
        fresh.insert(fresh_one);

        let count_cubes1 = cubes.len();

        let mut affected = cubes.clone();
        affected.retain(|v| overlaps(v, &fresh_one));
        for a in &affected {
            cubes.remove(a);
        }

        let count_affected1 = affected.len();

        affected = split(affected, split_points(fresh.clone()));

        fresh = split(fresh, split_points(affected.clone()));

        affected = split(affected, split_points(fresh.clone()));

        fresh = split(fresh, split_points(affected.clone()));

        let count_affected2 = affected.len();

        if parts[0] == "on" {
            affected.extend(fresh);
        } else {
            for f in fresh {
                // println!("removing: {:?}", f);
                affected.remove(&f);
            }
        }

        cubes.extend(affected);

        let count_cubes2 = cubes.len();

        println!(
            "cubes1: {}, affected1: {}, cubes2: {}, affected2: {}",
            count_cubes1, count_affected1, count_cubes2, count_affected2
        );

        let mut volume: i64 = 0;

        for v in &cubes {
            let mut cv = 1;
            for i in 0..3 {
                cv *= v.d[i].max as i64 - v.d[i].min as i64;
            }
            volume += cv;
        }

        println!("cube count: {}, volume: {:?}", cubes.len(), volume);

        cubes = join(cubes);

        println!();
    }

    Ok(())
}
