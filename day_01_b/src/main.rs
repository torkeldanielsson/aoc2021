use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let numbers: Vec<i64> = input.lines().map(|s| s.parse::<i64>().unwrap()).collect();

    let mut triplets: Vec<i64> = Vec::new();

    numbers.iter().enumerate().for_each(|(i, x)| {
        while triplets.len() <= i {
            triplets.push(0);
        }
        for j in 0..3 {
            if i as i32 - j >= 0 {
                triplets[i as usize - j as usize] += x;
            }
        }
    });

    // println!("{:?}", triplets);

    let mut increases = 0;

    triplets.iter().enumerate().for_each(|(i, x)| {
        if i as i32 - 1 >= 0 {
            if triplets[i - 1] < *x {
                increases += 1;
            }
        }
    });

    println!("{}", increases);

    Ok(())
}
