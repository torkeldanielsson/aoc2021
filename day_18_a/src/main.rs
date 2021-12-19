use std::{error::Error, fs};

fn to_char_radix_20(val: u8) -> char {
    match val {
        1 => '1',
        2 => '2',
        3 => '3',
        4 => '4',
        5 => '5',
        6 => '6',
        7 => '7',
        8 => '8',
        9 => '9',
        10 => 'A',
        11 => 'B',
        12 => 'C',
        13 => 'D',
        14 => 'E',
        15 => 'F',
        16 => 'G',
        17 => 'H',
        18 => 'I',
        19 => 'J',
        _ => panic!("illegal number"),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut sn: Vec<Vec<char>> = fs::read_to_string("input_test")?
        .lines()
        .map(|s| s.chars().collect())
        .collect();

    for sn in &mut sn {
        let mut something_happened = true;
        'main: while something_happened {
            something_happened = false;

            let mut depth = 0;
            for i in 0..sn.len() - 3 {
                if sn[i] == '[' && sn[i + 1] != '[' && sn[i + 3] != '[' && depth >= 4 {
                    println!("explode before: {:?}", String::from_iter(sn.iter()));
                    let left_value = sn[i + 1].to_string().parse::<u8>().unwrap();
                    let right_value = sn[i + 3].to_string().parse::<u8>().unwrap();
                    println!("l: {}, r: {}", left_value, right_value);
                    let mut li = i;
                    while li != 0 {
                        li -= 1;
                        if sn[li] != '[' && sn[li] != ',' && sn[li] != ']' {
                            let mut value = sn[li].to_string().parse::<u8>().unwrap();
                            value += left_value;
                            sn[li] = to_char_radix_20(value);
                            break;
                        }
                    }
                    let mut ri = i + 4;
                    while ri != sn.len() - 1 {
                        ri += 1;
                        if sn[ri] != '[' && sn[ri] != ',' && sn[ri] != ']' {
                            let mut value = sn[ri].to_string().parse::<u8>().unwrap();
                            value += right_value;
                            sn[ri] = to_char_radix_20(value);
                            break;
                        }
                    }
                    sn[i] = '0';
                    sn.remove(i + 1);
                    sn.remove(i + 1);
                    sn.remove(i + 1);
                    sn.remove(i + 1);
                    println!("explode after: {:?}", String::from_iter(sn.iter()));
                    something_happened = true;
                    continue 'main;
                }
                if sn[i] == '[' {
                    depth += 1;
                }
                if sn[i] == ']' {
                    depth -= 1;
                }
            }

            for i in 0..sn.len() {
                if sn[i] != '[' && sn[i] != ',' && sn[i] != ']' {
                    let value = u32::from_str_radix(&sn[i].to_string(), 20).unwrap();
                    if value > 9 {
                        println!("split before: {:?}", String::from_iter(sn.iter()));
                        let values = if value % 2 == 0 {
                            (value / 2, value / 2)
                        } else {
                            (value / 2, value / 2 + 1)
                        };
                        sn.remove(i);
                        sn.insert(i, ']');
                        sn.insert(i, to_char_radix_20(values.1 as u8));
                        sn.insert(i, ',');
                        sn.insert(i, to_char_radix_20(values.0 as u8));
                        sn.insert(i, '[');
                        println!("split after: {:?}", String::from_iter(sn.iter()));
                        something_happened = true;
                        continue 'main;
                    }
                }
            }
        }
    }

    println!("final: {:?}", String::from_iter(sn[0].iter()));

    Ok(())
}
