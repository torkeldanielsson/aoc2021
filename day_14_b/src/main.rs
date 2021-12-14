use std::{collections::HashMap, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input: Vec<String> = fs::read_to_string("input")?
        .split("\n\n")
        .map(|s| s.to_owned())
        .collect();

    let polymer: Vec<char> = input[0].chars().collect();

    let mut letter_sums: HashMap<char, i64> = HashMap::new();

    for c in &polymer {
        *letter_sums.entry(*c).or_insert(0) += 1;
    }

    let mut pair_map: HashMap<(char, char), i64> = HashMap::new();

    for i in 0..(polymer.len() - 1) {
        *pair_map.entry((polymer[i], polymer[i + 1])).or_insert(0) += 1;
    }

    let mut rules: Vec<((char, char), char)> = Vec::new();
    for rule in input[1].lines() {
        let rule: Vec<char> = rule.replace(" -> ", "").chars().collect();
        rules.push(((rule[0], rule[1]), rule[2]));
    }

    for _step in 1..=40 {
        let old_pair_map = pair_map.clone();
        pair_map.clear();
        'outer: for pair in old_pair_map {
            for rule in &rules {
                if pair.0 == rule.0 {
                    *letter_sums.entry(rule.1).or_insert(0) += pair.1;
                    *pair_map.entry((pair.0 .0, rule.1)).or_insert(0) += pair.1;
                    *pair_map.entry((rule.1, pair.0 .1)).or_insert(0) += pair.1;
                    continue 'outer;
                }
            }
            *pair_map.entry(pair.0).or_insert(0) += pair.1;
        }
    }

    println!("{:?}", letter_sums);

    let mut just_numbers = Vec::new();
    for p in letter_sums {
        just_numbers.push(p.1);
    }

    just_numbers.sort();

    println!(
        "{:?}",
        just_numbers.last().unwrap() - just_numbers.first().unwrap()
    );

    Ok(())
}
