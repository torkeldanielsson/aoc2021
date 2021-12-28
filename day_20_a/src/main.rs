use std::{collections::HashSet, error::Error, fs};

fn print_map(map: &HashSet<(i32, i32)>) -> ((i32, i32), (i32, i32)) {
    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut max_x = 0;
    let mut max_y = 0;
    for v in map {
        if v.0 < min_x {
            min_x = v.0;
        }
        if v.1 < min_y {
            min_y = v.1;
        }
        if v.0 > max_x {
            max_x = v.0;
        }
        if v.1 > max_y {
            max_y = v.1;
        }
    }

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if map.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();

    ((min_x, max_x), (min_y, max_y))
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;
    let parts: Vec<&str> = input.split("\n\n").collect();

    let mut algo: Vec<bool> = Vec::new();
    for c in parts[0].chars().into_iter() {
        match c {
            '.' => algo.push(false),
            '#' => algo.push(true),
            _ => panic!("!"),
        }
    }

    let mut map = HashSet::new();

    for line in parts[1].lines().enumerate() {
        for c in line.1.chars().enumerate() {
            match c.1 {
                '.' => {}
                '#' => {
                    map.insert((c.0 as i32, line.0 as i32));
                }
                _ => panic!("!"),
            }
        }
    }

    for _ in 0..2 {
        let range = print_map(&map);

        let old_map = map.clone();
        map.clear();

        for x in (range.0 .0 - 20)..=(range.0 .1 + 20) {
            for y in (range.1 .0 - 20)..=(range.1 .1 + 20) {
                let mut lookup: u32 = 0;
                for y_mod in (y - 1)..=(y + 1) {
                    for x_mod in (x - 1)..=(x + 1) {
                        lookup = lookup << 1;
                        if old_map.contains(&(x_mod, y_mod)) {
                            lookup |= 1;
                        }
                    }
                }
                if algo[lookup as usize] {
                    map.insert((x, y));
                }
            }
        }
    }

    let range = print_map(&map);

    let old_map = map.clone();
    map.clear();

    for v in old_map {
        if range.0 .0 + 25 < v.0
            && range.0 .1 - 25 > v.0
            && range.1 .0 + 25 < v.1
            && range.1 .1 - 25 > v.1
        {
            map.insert(v);
        }
    }
    print_map(&map);

    println!("res: {}", map.len());

    Ok(())
}
