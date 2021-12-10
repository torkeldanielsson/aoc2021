use std::{error::Error, fs};

fn illegal_score(c: char) -> i32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("whut"),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut score = 0;

    for line in input.lines() {
        let mut stack = Vec::new();
        for c in line.chars() {
            match c {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                '<' => stack.push('>'),
                ')' => {
                    if stack.pop() != Some(')') {
                        println!("{} => {} ({}) ({:?})", line, illegal_score(c), score, stack);
                        score += illegal_score(c);
                        break;
                    }
                }
                ']' => {
                    if stack.pop() != Some(']') {
                        println!("{} => {} ({}) ({:?})", line, illegal_score(c), score, stack);
                        score += illegal_score(c);
                        break;
                    }
                }
                '}' => {
                    if stack.pop() != Some('}') {
                        println!("{} => {} ({}) ({:?})", line, illegal_score(c), score, stack);
                        score += illegal_score(c);
                        break;
                    }
                }
                '>' => {
                    if stack.pop() != Some('>') {
                        println!("{} => {} ({}) ({:?})", line, illegal_score(c), score, stack);
                        score += illegal_score(c);
                        break;
                    }
                }
                _ => panic!("wut"),
            }
        }
    }

    println!("{}", score);

    Ok(())
}
