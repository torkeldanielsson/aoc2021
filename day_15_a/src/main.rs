use glam::{const_ivec2, ivec2, IVec2};
use std::thread;
use std::{collections::HashMap, fs};

const STACK_SIZE: usize = 4 * 1024 * 1024;
/*
fn print_map(map: &HashMap<IVec2, i32>) {
    let size = 10;

    for y in 0..size {
        for x in 0..size {
            print!("{}, ", map[&const_ivec2!([x, y])]);
        }
        println!();
    }

    println!();
}*/

fn set_scores(pos: IVec2, map: &HashMap<IVec2, i32>, scores: &mut HashMap<IVec2, i32>) {
    let score = scores[&pos];

    let neighbours = [ivec2(0, 1), ivec2(-1, 0), ivec2(1, 0), ivec2(0, -1)];

    let mut neighbour_vals = Vec::new();

    for neighbour in &neighbours {
        let neighbour = pos + *neighbour;
        if let Some(distance) = map.get(&neighbour) {
            neighbour_vals.push((neighbour, distance));
        }
    }

    neighbour_vals.sort_by(|p1, p2| p1.1.cmp(&p2.1));

    for neighbour in &neighbours {
        let neighbour = pos + *neighbour;
        if let Some(distance) = map.get(&neighbour) {
            if !scores.contains_key(&neighbour) || scores[&neighbour] > score + *distance {
                *scores.entry(neighbour).or_default() = score + *distance;
                set_scores(neighbour, map, scores);
            }
        }
    }
}

fn run() {
    let input = fs::read_to_string("input").unwrap();

    let mut map = HashMap::new();

    let mut max_x = 0;
    let mut max_y = 0;

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert(
                const_ivec2!([x as i32, y as i32]),
                c.to_string().parse::<i32>().unwrap(),
            );
            if x > max_x {
                max_x = x;
            }
        }
        if y > max_y {
            max_y = y;
        }
    }

    // print_map(&map);

    let mut scores = HashMap::new();

    let start = ivec2(0, 0);
    let end = ivec2(max_x as i32, max_y as i32);

    scores.insert(start, 0);

    set_scores(start, &map, &mut scores);

    // print_map(&scores);

    println!("{}", scores[&end]);
}

fn main() {
    // Spawn thread with explicit stack size
    let child = thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(run)
        .unwrap();

    // Wait for thread to join
    child.join().unwrap();
}
