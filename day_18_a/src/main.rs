use std::{error::Error, fs};

#[derive(Debug)]
struct Sn {
    number: Option<i32>,
    sub_nodes: Option<Box<(Sn, Sn)>>,
}

fn parse_sn(line: &[char]) -> (Sn, usize) {
    if line[0] != '[' {
        return (
            Sn {
                number: Some(line[0].to_string().parse::<i32>().unwrap()),
                sub_nodes: None,
            },
            1,
        );
    }

    let sn_1 = parse_sn(&line[1..]);

    assert!(line[1 + sn_1.1] == ',');

    let sn_2 = parse_sn(&line[(1 + sn_1.1 + 1)..]);

    assert!(line[1 + sn_1.1 + 1 + sn_2.1] == ']');

    (
        Sn {
            number: None,
            sub_nodes: Some(Box::new((sn_1.0, sn_2.0))),
        },
        1 + sn_1.1 + 1 + sn_2.1 + 1,
    )
}

fn explode(sn: &mut Sn, depth: i32) -> bool {
    println!("test level {}", depth);

    if depth >= 4 {
        if let Some(sub_nodes) = &mut sn.sub_nodes {
            if sub_nodes.0.number.is_some() && sub_nodes.1.number.is_some() {
                return true;
            }
        }
    }

    if let Some(sub_nodes) = &mut sn.sub_nodes {
        if explode(&mut sub_nodes.0, depth + 1) {
            return true;
        }
        if explode(&mut sub_nodes.1, depth + 1) {
            return true;
        }
    }
    false
}

fn main() -> Result<(), Box<dyn Error>> {
    for line in fs::read_to_string("input_test")?.lines() {
        let chars: Vec<char> = line.chars().collect();
        let sn = parse_sn(&chars);

        println!("{:?}", sn.0);

        let mut sn = sn.0;

        if explode(&mut sn, 1) {
            println!("boom");
        } else {
            println!("no boom");
        }

        //  let mut something_happened = true;
        //  while something_happened {
        //      something_happened = explode(&mut sn, 1);
        //  }
    }

    Ok(())
}
