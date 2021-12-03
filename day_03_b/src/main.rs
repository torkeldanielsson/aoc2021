use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let width = 12;

    let mut values = Vec::new();

    let val1 = {
        for line in input.lines() {
            let digits: Vec<char> = line.chars().collect();

            let mut value = Vec::new();

            for digit in digits {
                match digit {
                    '1' => value.push(1),
                    '0' => value.push(0),
                    _ => println!("ERROR"),
                }
            }

            values.push(value);
        }

        for i in 0..width {
            let numbers_left: usize = values.len();
            let mut digit_count = 0;
            for value in &values {
                if value[i] == 1 {
                    digit_count += 1;
                }
            }
            if digit_count >= numbers_left - digit_count {
                values.retain(|value| value[i] == 1);
            } else {
                values.retain(|value| value[i] == 0);
            }

            println!("{:?}", values);
        }

        let value = &values[0];

        println!("{:?}", value);

        let mut res: u32 = 0;

        for i in 0..width {
            res |= value[i] << (width - 1 - i);
        }

        res
    };

    let val2 = {
        for line in input.lines() {
            let digits: Vec<char> = line.chars().collect();

            let mut value = Vec::new();

            for digit in digits {
                match digit {
                    '1' => value.push(1),
                    '0' => value.push(0),
                    _ => println!("ERROR"),
                }
            }

            values.push(value);
        }

        for i in 0..width {
            let numbers_left: usize = values.len();
            let mut digit_count = 0;
            for value in &values {
                if value[i] == 1 {
                    digit_count += 1;
                }
            }
            if digit_count < numbers_left - digit_count {
                values.retain(|value| value[i] == 1);
            } else {
                values.retain(|value| value[i] == 0);
            }

            println!("{:?}", values);

            if values.len() == 1 {
                break;
            }
        }

        let value = &values[0];

        println!("{:?}", value);

        let mut res: u32 = 0;

        for i in 0..width {
            res |= value[i] << (width - 1 - i);
        }

        res
    };

    println!("{}", val1 * val2);

    Ok(())
}
