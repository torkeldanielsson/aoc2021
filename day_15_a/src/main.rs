use glam::{const_ivec2, ivec2, IVec2};
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs,
};

fn main() -> Result<(), Box<dyn Error>> {
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

    let start = ivec2(0, 0);
    let end = ivec2(max_x as i32, max_y as i32);

    let mut q: HashSet<IVec2> = HashSet::new();
    let mut dist: HashMap<IVec2, i32> = HashMap::new();
    let mut prev: HashMap<IVec2, i32> = HashMap::new();

    for p in &map {
        q.insert(*p.0);
        dist.insert(*p.0, i32::MAX);
    }

    dist.insert(start, 0);

    while !q.is_empty() {
        let mut u = IVec2::new(-1, -1);
        let mut min_dist = i32::MAX;
        for qp in &q {
            if dist[qp] < min_dist {
                min_dist = dist[qp];
                u = *qp;
            }
        }
        q.remove(&u);

        if u == end {
            println!("prev : {}", prev[&u]);
            println!("dist : {}", dist[&u]);
            return Ok(());
        }

        for neighbour in &[ivec2(0, 1), ivec2(-1, 0), ivec2(1, 0), ivec2(0, -1)] {
            let v = u + *neighbour;
            if q.contains(&v) {
                let alt = dist[&u] + map[&v];
                if alt < dist[&v] {
                    dist.insert(v, alt);
                    prev.insert(v, alt);
                }
            }
        }
    }

    Ok(())
}
