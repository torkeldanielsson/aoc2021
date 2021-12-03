use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut indices = vec![0; 12];

    for line in input.lines() {
        let digits: Vec<char> = line.chars().collect();
        for (i, d) in digits.iter().enumerate() {
            if *d == '1' {
                indices[i] += 1;
            }
        }
    }

    println!("{:?}", indices);

    let line_count = input.lines().count();

    println!("{}", line_count / 2);

    let mut res: u32 = 0;
    let mut mask: u32 = 0;

    for i in 0..12 {
        if indices[i] > line_count / 2 {
            res |= 1 << (11 - i);
            print!("1");
        } else {
            print!("0");
        }
        mask |= 1 << i;
    }
    print!("\n");

    println!("{}", res * ((!res) & mask));

    Ok(())
}
