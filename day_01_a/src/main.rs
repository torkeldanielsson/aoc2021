use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut last_digit = 9999;
    let mut increases = 0;

    for line in input.lines() {
        let digit = line.parse::<i64>()?;

        if digit > last_digit {
            increases += 1;
        }

        last_digit = digit;
    }

    println!("{}", increases);

    Ok(())
}
