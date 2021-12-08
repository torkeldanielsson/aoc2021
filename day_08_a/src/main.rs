use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let res: Vec<Vec<usize>> = fs::read_to_string("input")?
        .lines()
        .map(|s| s.split("|").nth(1).unwrap())
        .map(|s| {
            s.split_whitespace()
                .map(|ss| ss.trim().len())
                .filter(|&n| n == 2 || n == 3 || n == 4 || n == 7)
                .collect()
        })
        .collect();

    println!("{:?}", res.concat().len());

    Ok(())
}
