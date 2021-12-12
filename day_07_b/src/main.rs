use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let numbers: Vec<i32> = fs::read_to_string("input")?
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let mut best_val = -1;
    let mut best_fuel = i32::MAX;

    for val in -1000..2000 {
        let mut fuel = 0;
        for n in &numbers {
            let steps = (val - *n).abs();
            for step in 0..steps {
                fuel += step + 1;
            }
        }
        if fuel < best_fuel {
            best_val = val;
            best_fuel = fuel;
        }
    }

    println!("best val: {} ({} fuel)", best_val, best_fuel);

    Ok(())
}
