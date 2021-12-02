use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut pos = (0, 0);

    for line in input.lines().map(|s| s.trim()).filter(|s| !s.is_empty()) {
        let parts: Vec<&str> = line.split_whitespace().collect();

        let steps = parts[1].parse::<i32>().unwrap();

        match parts[0] {
            "forward" => pos.0 += steps,
            "up" => pos.1 -= steps,
            "down" => pos.1 += steps,
            _ => panic!(),
        };
    }

    println!("{:?}, mul: {}", pos, pos.0 * pos.1);

    Ok(())
}
