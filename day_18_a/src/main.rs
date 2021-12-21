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
        20 => 'K',
        21 => 'L',
        22 => 'M',
        23 => 'N',
        24 => 'O',
        25 => 'P',
        26 => 'Q',
        27 => 'R',
        28 => 'S',
        29 => 'T',
        _ => panic!("illegal number: {}", val),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut sn: Vec<Vec<char>> = fs::read_to_string("input_test")?
        .lines()
        .map(|s| s.chars().collect())
        .collect();

    let mut res = sn[0].to_owned();
    sn.remove(0);

    for sn in &mut sn {
        res.insert(0, '[');
        res.push(',');
        res.append(sn);
        res.push(']');

        let mut something_happened = true;
        'main: while something_happened {
            something_happened = false;

            let mut depth = 0;
            for i in 0..res.len() - 3 {
                if res[i] == '[' && res[i + 1] != '[' && res[i + 3] != '[' && depth >= 4 {
                    res.insert(i, '_');
                    println!(
                        "explode before: {:?} ({})",
                        String::from_iter(res.iter()),
                        i
                    );
                    res.remove(i);
                    let left_value =
                        u32::from_str_radix(&res[i + 1].to_string(), 30).unwrap() as u8;
                    let right_value =
                        u32::from_str_radix(&res[i + 3].to_string(), 30).unwrap() as u8;
                    println!("l: {}, r: {}", left_value, right_value);
                    let mut li = i;
                    while li != 0 {
                        li -= 1;
                        if res[li] != '[' && res[li] != ',' && res[li] != ']' {
                            let mut value = res[li].to_string().parse::<u8>().unwrap();
                            value += left_value;
                            res[li] = to_char_radix_20(value);
                            break;
                        }
                    }
                    let mut ri = i + 4;
                    while ri != res.len() - 1 {
                        ri += 1;
                        if res[ri] != '[' && res[ri] != ',' && res[ri] != ']' {
                            let mut value =
                                u32::from_str_radix(&res[ri].to_string(), 30).unwrap() as u8;
                            value += right_value;
                            res[ri] = to_char_radix_20(value);
                            break;
                        }
                    }
                    res[i] = '0';
                    res.remove(i + 1);
                    res.remove(i + 1);
                    res.remove(i + 1);
                    res.remove(i + 1);
                    println!("explode after: {:?}", String::from_iter(res.iter()));
                    something_happened = true;
                    continue 'main;
                }
                if res[i] == '[' {
                    depth += 1;
                }
                if res[i] == ']' {
                    depth -= 1;
                }
            }

            for i in 0..res.len() {
                if res[i] != '[' && res[i] != ',' && res[i] != ']' {
                    let value = u32::from_str_radix(&res[i].to_string(), 30).unwrap();
                    if value > 9 {
                        println!("split before: {:?}", String::from_iter(res.iter()));
                        let values = if value % 2 == 0 {
                            (value / 2, value / 2)
                        } else {
                            (value / 2, value / 2 + 1)
                        };
                        res.remove(i);
                        res.insert(i, ']');
                        res.insert(i, to_char_radix_20(values.1 as u8));
                        res.insert(i, ',');
                        res.insert(i, to_char_radix_20(values.0 as u8));
                        res.insert(i, '[');
                        println!("split after: {:?}", String::from_iter(res.iter()));
                        something_happened = true;
                        continue 'main;
                    }
                }
            }
        }
        println!("final: {:?}", String::from_iter(res.iter()));
    }

    Ok(())
}
