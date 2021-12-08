use std::{ascii::AsciiExt, error::Error, fs};

fn solve(line: &str) -> i32 {
    let mut parts = line.splitn(2, '|');
    let input: Vec<&str> = parts.next().unwrap().split_whitespace().collect();
    let numbers: Vec<&str> = parts.next().unwrap().split_whitespace().collect();

    let d_1 = input
        .iter()
        .find(|&&x| x.len() == 2)
        .unwrap()
        .to_owned()
        .to_owned();
    let d_4 = input
        .iter()
        .find(|&&x| x.len() == 4)
        .unwrap()
        .to_owned()
        .to_owned();
    let d_7 = input
        .iter()
        .find(|&&x| x.len() == 3)
        .unwrap()
        .to_owned()
        .to_owned();
    let d_8 = input
        .iter()
        .find(|&&x| x.len() == 7)
        .unwrap()
        .to_owned()
        .to_owned();

    let mut is_not_in_2: char = ' ';
    let mut is_not_in_5: char = ' ';

    let d_6 = {
        let candidates: Vec<&&str> = input.iter().filter(|s| s.len() == 6).collect();
        let mut d_6 = String::new();
        for c in candidates {
            let c_chars: Vec<char> = c.chars().collect();
            let d_1_chars: Vec<char> = d_1.chars().collect();
            if !c_chars.contains(&d_1_chars[0]) || !c_chars.contains(&d_1_chars[1]) {
                d_6 = (*c).to_owned();
                if !c_chars.contains(&d_1_chars[0]) {
                    is_not_in_5 = d_1_chars[0];
                    is_not_in_2 = d_1_chars[1];
                } else {
                    is_not_in_5 = d_1_chars[1];
                    is_not_in_2 = d_1_chars[0];
                }
            }
        }
        d_6
    };

    let mut is_not_in_9: char = ' ';

    let (d_2, d_5, d_3) = {
        let candidates: Vec<&&str> = input.iter().filter(|s| s.len() == 5).collect();
        let mut d_2 = String::new();
        let mut d_3 = String::new();
        let mut d_5 = String::new();
        for c in candidates {
            let c_chars: Vec<char> = c.chars().collect();
            if !c_chars.contains(&is_not_in_2) {
                d_2 = (*c).to_owned();
            } else if !c_chars.contains(&is_not_in_5) {
                d_5 = (*c).to_owned();
                for cc in d_8.chars() {
                    if !d_5.contains(cc) && cc != is_not_in_5 {
                        is_not_in_9 = cc;
                    }
                }
            } else {
                d_3 = (*c).to_owned();
            }
        }

        (d_2, d_5, d_3)
    };

    let d_9 = {
        let candidates: Vec<&&str> = input.iter().filter(|s| s.len() == 6).collect();
        let mut d_9 = String::new();
        for c in candidates {
            let c_chars: Vec<char> = c.chars().collect();
            if !c_chars.contains(&is_not_in_9) {
                d_9 = (*c).to_owned();
            }
        }
        d_9
    };

    let d_0 = {
        let candidates: Vec<&&str> = input.iter().filter(|s| s.len() == 6).collect();
        let mut d_0 = String::new();
        for c in candidates {
            if c != &d_9 && c != &d_6 {
                d_0 = (*c).to_owned();
            }
        }
        d_0
    };

    println!(
        "{:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}",
        d_0, d_1, d_2, d_3, d_4, d_5, d_6, d_7, d_8, d_9
    );

    println!("{:?}", numbers);

    let numbers: Vec<i32> = numbers
        .iter()
        .map(|s| {
            if s == &d_0 {
                0
            } else if s == &d_1 {
                1
            } else if s == &d_2 {
                2
            } else if s == &d_3 {
                3
            } else if s == &d_4 {
                4
            } else if s == &d_5 {
                5
            } else if s == &d_6 {
                6
            } else if s == &d_7 {
                7
            } else if s == &d_8 {
                8
            } else {
                println!("{:?}", s);
                9
            }
        })
        .collect();

    println!("{:?}", numbers);

    0
}

fn main() -> Result<(), Box<dyn Error>> {
    solve(
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab |
cdfeb fcadb cdfeb cdbaf",
    );

    /*
    for line in fs::read_to_string("input_test")?.lines() {
        solve(line);
    }
    */

    Ok(())
}
