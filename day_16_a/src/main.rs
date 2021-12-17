use std::{collections::HashMap, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input: Vec<String> = fs::read_to_string("input")?
        .split("\n\n")
        .map(|s| s.to_owned())
        .collect();

    Ok(())
}
