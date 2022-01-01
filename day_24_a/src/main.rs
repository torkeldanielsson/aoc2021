use std::{error::Error, fs};

fn reg(s: &str) -> usize {
    match s {
        "w" => 0,
        "x" => 1,
        "y" => 2,
        "z" => 3,
        _ => panic!(),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut registers = [0; 4];

    let mut input = vec![5, 4, 3, 2, 1];

    for line in fs::read_to_string("input_test")?.lines() {
        let parts: Vec<&str> = line.split(" ").collect();

        match parts[0] {
            "inp" => {
                registers[reg(parts[1])] = input.remove(0);
            }
            "add" => {
                registers[reg(parts[1])] += match parts[2] {
                    "w" | "x" | "y" | "z" => registers[reg(parts[2])],
                    _ => parts[2].parse::<i32>().unwrap(),
                };
            }
            "mul" => {
                registers[reg(parts[1])] *= match parts[2] {
                    "w" | "x" | "y" | "z" => registers[reg(parts[2])],
                    _ => parts[2].parse::<i32>().unwrap(),
                };
            }
            "div" => {
                registers[reg(parts[1])] /= match parts[2] {
                    "w" | "x" | "y" | "z" => registers[reg(parts[2])],
                    _ => parts[2].parse::<i32>().unwrap(),
                };
            }
            "mod" => {
                registers[reg(parts[1])] %= match parts[2] {
                    "w" | "x" | "y" | "z" => registers[reg(parts[2])],
                    _ => parts[2].parse::<i32>().unwrap(),
                };
            }
            "eql" => {
                let a = registers[reg(parts[1])];
                let b = match parts[2] {
                    "w" | "x" | "y" | "z" => registers[reg(parts[2])],
                    _ => parts[2].parse::<i32>().unwrap(),
                };
                if a == b {
                    registers[reg(parts[1])] = 1;
                } else {
                    registers[reg(parts[1])] = 0;
                }
            }
            _ => panic!(),
        }

        println!(
            "w: {}, x: {}, y: {}, z: {}",
            registers[0], registers[1], registers[2], registers[3]
        );
    }

    Ok(())
}
