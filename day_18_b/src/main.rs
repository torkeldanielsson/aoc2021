use core::fmt;
use std::{error::Error, fs};

#[derive(Debug, Copy, Clone)]
struct Val {
    is_val: bool,
    val: i32,
    c: char,
}

impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_val {
            write!(f, "{}", self.val)
        } else {
            write!(f, "{}", self.c)
        }
    }
}

/*
fn print_val_arr(vals: &Vec<Val>) {
    for v in vals {
        print!("{}", v);
    }
    println!();
}
*/

#[derive(Debug)]
struct Sn {
    number: Option<i32>,
    sub_nodes: Option<Box<(Sn, Sn)>>,
}

fn magnitude(sn: &Sn) -> i64 {
    if let Some(number) = sn.number {
        return number as i64;
    }
    return 3 * magnitude(&sn.sub_nodes.as_ref().unwrap().0)
        + 2 * magnitude(&sn.sub_nodes.as_ref().unwrap().1);
}

fn parse_sn(line: &[Val]) -> (Sn, usize) {
    if line[0].c != '[' {
        return (
            Sn {
                number: Some(line[0].to_string().parse::<i32>().unwrap()),
                sub_nodes: None,
            },
            1,
        );
    }

    let sn_1 = parse_sn(&line[1..]);

    assert!(line[1 + sn_1.1].c == ',');

    let sn_2 = parse_sn(&line[(1 + sn_1.1 + 1)..]);

    assert!(line[1 + sn_1.1 + 1 + sn_2.1].c == ']');

    (
        Sn {
            number: None,
            sub_nodes: Some(Box::new((sn_1.0, sn_2.0))),
        },
        1 + sn_1.1 + 1 + sn_2.1 + 1,
    )
}

fn reduce_and_get_magnitude(mut res: Vec<Val>, sn: &Vec<Val>) -> i32 {
    res.insert(
        0,
        Val {
            is_val: false,
            val: 0,
            c: '[',
        },
    );
    res.push(Val {
        is_val: false,
        val: 0,
        c: ',',
    });
    let mut weird_temp = sn.clone();
    res.append(&mut weird_temp);
    res.push(Val {
        is_val: false,
        val: 0,
        c: ']',
    });

    let mut something_happened = true;
    'main: while something_happened {
        something_happened = false;

        let mut depth = 0;
        for i in 0..res.len() - 3 {
            if res[i].c == '[' && res[i + 1].c != '[' && res[i + 3].c != '[' && depth >= 4 {
                let left_value = res[i + 1].val;
                let right_value = res[i + 3].val;
                let mut li = i;
                while li != 0 {
                    li -= 1;
                    if res[li].c != '[' && res[li].c != ',' && res[li].c != ']' {
                        res[li].val += left_value;
                        break;
                    }
                }
                let mut ri = i + 4;
                while ri != res.len() - 1 {
                    ri += 1;
                    if res[ri].c != '[' && res[ri].c != ',' && res[ri].c != ']' {
                        res[ri].val += right_value;
                        break;
                    }
                }
                res[i] = Val {
                    is_val: true,
                    val: 0,
                    c: ' ',
                };
                res.remove(i + 1);
                res.remove(i + 1);
                res.remove(i + 1);
                res.remove(i + 1);

                something_happened = true;
                continue 'main;
            }
            if res[i].c == '[' {
                depth += 1;
            }
            if res[i].c == ']' {
                depth -= 1;
            }
        }

        for i in 0..res.len() {
            if res[i].c != '[' && res[i].c != ',' && res[i].c != ']' {
                let value = res[i].val;
                if value > 9 {
                    let values = if value % 2 == 0 {
                        (value / 2, value / 2)
                    } else {
                        (value / 2, value / 2 + 1)
                    };
                    res.remove(i);
                    res.insert(
                        i,
                        Val {
                            is_val: false,
                            val: 0,
                            c: ']',
                        },
                    );
                    res.insert(
                        i,
                        Val {
                            is_val: true,
                            val: values.1,
                            c: ' ',
                        },
                    );
                    res.insert(
                        i,
                        Val {
                            is_val: false,
                            val: 0,
                            c: ',',
                        },
                    );
                    res.insert(
                        i,
                        Val {
                            is_val: true,
                            val: values.0,
                            c: ' ',
                        },
                    );
                    res.insert(
                        i,
                        Val {
                            is_val: false,
                            val: 0,
                            c: '[',
                        },
                    );
                    something_happened = true;
                    continue 'main;
                }
            }
        }
    }

    let sn = parse_sn(&res);

    magnitude(&sn.0) as i32
}

fn main() -> Result<(), Box<dyn Error>> {
    let sn: Vec<Vec<Val>> = fs::read_to_string("input")?
        .lines()
        .map(|s| {
            s.chars()
                .map(|c| match c {
                    '[' | ',' | ']' => Val {
                        is_val: false,
                        val: 0,
                        c: c,
                    },
                    _ => Val {
                        is_val: true,
                        val: c.to_string().parse::<u8>().unwrap() as i32,
                        c: ' ',
                    },
                })
                .collect()
        })
        .collect();

    let mut largest = 0;

    for i1 in 0..sn.len() - 1 {
        for i2 in i1 + 1..sn.len() {
            let magnitude = reduce_and_get_magnitude(sn[i1].clone(), &sn[i2]);
            if magnitude > largest {
                largest = magnitude;
                println!("{}", magnitude);
            }
            let magnitude = reduce_and_get_magnitude(sn[i2].clone(), &sn[i1]);
            if magnitude > largest {
                largest = magnitude;
                println!("{}", magnitude);
            }
        }
    }

    Ok(())
}
