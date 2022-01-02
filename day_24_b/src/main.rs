use std::{
    collections::HashSet,
    error::Error,
    fs,
    io::{self, Write},
};

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
    let mut first_four = Vec::new();

    for a in 1..=9 {
        for b in 1..=9 {
            for c in 1..=9 {
                for d in 1..=9 {
                    let mut input = vec![a, b, c, d];
                    let mut registers = [0 as i64; 4];
                    for line in fs::read_to_string("input_test")?.lines() {
                        let parts: Vec<&str> = line.split(" ").collect();

                        match parts[0] {
                            "inp" => {
                                registers[reg(parts[1])] = input.remove(0);
                            }
                            "add" => {
                                registers[reg(parts[1])] += match parts[2] {
                                    "w" | "x" | "y" | "z" => registers[reg(parts[2])],
                                    _ => parts[2].parse::<i64>().unwrap(),
                                };
                            }
                            "mul" => {
                                registers[reg(parts[1])] *= match parts[2] {
                                    "w" | "x" | "y" | "z" => registers[reg(parts[2])],
                                    _ => parts[2].parse::<i64>().unwrap(),
                                };
                            }
                            "div" => {
                                registers[reg(parts[1])] /= match parts[2] {
                                    "w" | "x" | "y" | "z" => registers[reg(parts[2])],
                                    _ => parts[2].parse::<i64>().unwrap(),
                                };
                            }
                            "mod" => {
                                registers[reg(parts[1])] %= match parts[2] {
                                    "w" | "x" | "y" | "z" => registers[reg(parts[2])],
                                    _ => parts[2].parse::<i64>().unwrap(),
                                };
                            }
                            "eql" => {
                                let a = registers[reg(parts[1])];
                                let b = match parts[2] {
                                    "w" | "x" | "y" | "z" => registers[reg(parts[2])],
                                    _ => parts[2].parse::<i64>().unwrap(),
                                };
                                if a == b {
                                    registers[reg(parts[1])] = 1;
                                } else {
                                    registers[reg(parts[1])] = 0;
                                }
                            }
                            _ => panic!(),
                        }
                    }

                    if registers[3] < 3000 {
                        println!(
                            "input: {}{}{}{} => w: {}, x: {}, y: {}, z: {}",
                            a, b, c, d, registers[0], registers[1], registers[2], registers[3]
                        );
                        first_four.push(vec![a, b, c, d]);
                    }
                }
            }
        }
    }

    let mut second_four = HashSet::new();

    for ff in &first_four[0..2] {
        for a in 1..=9 {
            for b in 1..=9 {
                for c in 1..=9 {
                    for d in 1..=9 {
                        let mut input = vec![ff[0], ff[1], ff[2], ff[3], a, b, c, d];
                        let mut registers = [0 as i64; 4];
                        for line in fs::read_to_string("input_test_8")?.lines() {
                            let parts: Vec<&str> = line.split(" ").collect();

                            match parts[0] {
                                "inp" => {
                                    registers[reg(parts[1])] = input.remove(0);
                                }
                                "add" => {
                                    registers[reg(parts[1])] += match parts[2] {
                                        "w" | "x" | "y" | "z" => registers[reg(parts[2])],
                                        _ => parts[2].parse::<i64>().unwrap(),
                                    };
                                }
                                "mul" => {
                                    registers[reg(parts[1])] *= match parts[2] {
                                        "w" | "x" | "y" | "z" => registers[reg(parts[2])],
                                        _ => parts[2].parse::<i64>().unwrap(),
                                    };
                                }
                                "div" => {
                                    registers[reg(parts[1])] /= match parts[2] {
                                        "w" | "x" | "y" | "z" => registers[reg(parts[2])],
                                        _ => parts[2].parse::<i64>().unwrap(),
                                    };
                                }
                                "mod" => {
                                    registers[reg(parts[1])] %= match parts[2] {
                                        "w" | "x" | "y" | "z" => registers[reg(parts[2])],
                                        _ => parts[2].parse::<i64>().unwrap(),
                                    };
                                }
                                "eql" => {
                                    let a = registers[reg(parts[1])];
                                    let b = match parts[2] {
                                        "w" | "x" | "y" | "z" => registers[reg(parts[2])],
                                        _ => parts[2].parse::<i64>().unwrap(),
                                    };
                                    if a == b {
                                        registers[reg(parts[1])] = 1;
                                    } else {
                                        registers[reg(parts[1])] = 0;
                                    }
                                }
                                _ => panic!(),
                            }
                        }

                        if registers[3] < 1000 {
                            println!(
                                "B! input: {}{}{}{} => w: {}, x: {}, y: {}, z: {}",
                                a, b, c, d, registers[0], registers[1], registers[2], registers[3]
                            );
                            second_four.insert(vec![a, b, c, d]);
                        }
                    }
                }
            }
        }
    }

    let mut third_four = HashSet::new();

    {
        let ff = &first_four[0];
        for sf in &second_four {
            for a in 1..=9 {
                for b in 1..=9 {
                    for c in 1..=9 {
                        for d in 1..=9 {
                            let mut input = vec![
                                ff[0], ff[1], ff[2], ff[3], sf[0], sf[1], sf[2], sf[3], a, b, c, d,
                            ];
                            let mut registers = [0 as i64; 4];
                            for line in fs::read_to_string("input_test_12")?.lines() {
                                let parts: Vec<&str> = line.split(" ").collect();

                                match parts[0] {
                                    "inp" => {
                                        registers[reg(parts[1])] = input.remove(0);
                                    }
                                    "add" => {
                                        registers[reg(parts[1])] += match parts[2] {
                                            "w" | "x" | "y" | "z" => registers[reg(parts[2])],
                                            _ => parts[2].parse::<i64>().unwrap(),
                                        };
                                    }
                                    "mul" => {
                                        registers[reg(parts[1])] *= match parts[2] {
                                            "w" | "x" | "y" | "z" => registers[reg(parts[2])],
                                            _ => parts[2].parse::<i64>().unwrap(),
                                        };
                                    }
                                    "div" => {
                                        registers[reg(parts[1])] /= match parts[2] {
                                            "w" | "x" | "y" | "z" => registers[reg(parts[2])],
                                            _ => parts[2].parse::<i64>().unwrap(),
                                        };
                                    }
                                    "mod" => {
                                        registers[reg(parts[1])] %= match parts[2] {
                                            "w" | "x" | "y" | "z" => registers[reg(parts[2])],
                                            _ => parts[2].parse::<i64>().unwrap(),
                                        };
                                    }
                                    "eql" => {
                                        let a = registers[reg(parts[1])];
                                        let b = match parts[2] {
                                            "w" | "x" | "y" | "z" => registers[reg(parts[2])],
                                            _ => parts[2].parse::<i64>().unwrap(),
                                        };
                                        if a == b {
                                            registers[reg(parts[1])] = 1;
                                        } else {
                                            registers[reg(parts[1])] = 0;
                                        }
                                    }
                                    _ => panic!(),
                                }
                            }

                            if registers[3] < 1000 {
                                println!(
                                    "C! input: {}{}{}{} => w: {}, x: {}, y: {}, z: {}",
                                    a,
                                    b,
                                    c,
                                    d,
                                    registers[0],
                                    registers[1],
                                    registers[2],
                                    registers[3]
                                );
                                third_four.insert(vec![a, b, c, d]);
                            }
                        }
                    }
                }
            }
            break;
        }
    }

    println!("ff len: {}", first_four.len());

    let mut ok_numbers = Vec::new();

    for ff in &first_four {
        for sf in &second_four {
            for tf in &third_four {
                for a in 1..=9 {
                    for b in 1..=9 {
                        let mut input = vec![
                            ff[0], ff[1], ff[2], ff[3], sf[0], sf[1], sf[2], sf[3], tf[0], tf[1],
                            tf[2], tf[3], a, b,
                        ];
                        let input_copy = input.clone();
                        let mut registers = [0 as i64; 4];
                        for line in fs::read_to_string("input")?.lines() {
                            let parts: Vec<&str> = line.split(" ").collect();

                            match parts[0] {
                                "inp" => {
                                    registers[reg(parts[1])] = input.remove(0);
                                }
                                "add" => {
                                    registers[reg(parts[1])] += match parts[2] {
                                        "w" | "x" | "y" | "z" => registers[reg(parts[2])],
                                        _ => parts[2].parse::<i64>().unwrap(),
                                    };
                                }
                                "mul" => {
                                    registers[reg(parts[1])] *= match parts[2] {
                                        "w" | "x" | "y" | "z" => registers[reg(parts[2])],
                                        _ => parts[2].parse::<i64>().unwrap(),
                                    };
                                }
                                "div" => {
                                    registers[reg(parts[1])] /= match parts[2] {
                                        "w" | "x" | "y" | "z" => registers[reg(parts[2])],
                                        _ => parts[2].parse::<i64>().unwrap(),
                                    };
                                }
                                "mod" => {
                                    registers[reg(parts[1])] %= match parts[2] {
                                        "w" | "x" | "y" | "z" => registers[reg(parts[2])],
                                        _ => parts[2].parse::<i64>().unwrap(),
                                    };
                                }
                                "eql" => {
                                    let a = registers[reg(parts[1])];
                                    let b = match parts[2] {
                                        "w" | "x" | "y" | "z" => registers[reg(parts[2])],
                                        _ => parts[2].parse::<i64>().unwrap(),
                                    };
                                    if a == b {
                                        registers[reg(parts[1])] = 1;
                                    } else {
                                        registers[reg(parts[1])] = 0;
                                    }
                                }
                                _ => panic!(),
                            }
                        }

                        if registers[3] == 0 {
                            println!("input: {:?} is ok!", input_copy);
                            let mut num = 0;
                            for n in input_copy {
                                num *= 10;
                                num += n;
                            }
                            ok_numbers.push(num);
                        }
                    }
                }
            }
        }
        print!(".");
        io::stdout().flush().unwrap();
    }

    ok_numbers.sort();

    println!("smallest: {}", ok_numbers[0]);
    println!("largest: {}", ok_numbers.last().unwrap());

    Ok(())
}
