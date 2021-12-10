use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut scores = Vec::new();

    for line in input.lines() {
        let mut stack = Vec::new();
        let mut is_ok = true;

        let mut score: i64 = 0;
        for c in line.chars() {
            match c {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                '<' => stack.push('>'),
                ')' => {
                    if stack.pop() != Some(')') {
                        is_ok = false;
                        break;
                    }
                }
                ']' => {
                    if stack.pop() != Some(']') {
                        is_ok = false;
                        break;
                    }
                }
                '}' => {
                    if stack.pop() != Some('}') {
                        is_ok = false;
                        break;
                    }
                }
                '>' => {
                    if stack.pop() != Some('>') {
                        is_ok = false;
                        break;
                    }
                }
                _ => panic!("wut"),
            }
        }

        stack.reverse();

        if is_ok && stack.len() != 0 {
            for c in &stack {
                score = 5 * score
                    + match c {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => panic!("whut"),
                    };
            }
            scores.push(score);

            println!("{} {:?} {}", line, stack, score);
        }
    }

    scores.sort();

    println!("{:?}", scores);

    println!("{}", scores[scores.len() / 2]);

    Ok(())
}
