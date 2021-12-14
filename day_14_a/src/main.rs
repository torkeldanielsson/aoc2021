use std::{collections::HashMap, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input: Vec<String> = fs::read_to_string("input")?
        .split("\n\n")
        .map(|s| s.to_owned())
        .collect();

    let mut polymer: Vec<char> = input[0].chars().collect();

    // println!("Template: {:?}", polymer);

    let mut rules: Vec<(char, char, char)> = Vec::new();
    for rule in input[1].lines() {
        let rule: Vec<char> = rule.replace(" -> ", "").chars().collect();
        rules.push((rule[0], rule[1], rule[2]));
    }

    for step in 1..=10 {
        let old_polymer = polymer.clone();
        polymer.clear();
        for i in 0..(old_polymer.len() - 1) {
            polymer.push(old_polymer[i]);
            for rule in &rules {
                if old_polymer[i] == rule.0 && old_polymer[i + 1] == rule.1 {
                    polymer.push(rule.2);
                }
            }
        }
        polymer.push(old_polymer[old_polymer.len() - 1]);

        // println!("After step {}: {:?}", step, polymer);

        println!("After step {}, length: {}", step, polymer.len());
    }

    let mut map = HashMap::new();

    for c in polymer {
        if map.contains_key(&c) {
            map.insert(c, map[&c] + 1);
        } else {
            map.insert(c, 1);
        }
    }

    println!("{:?}", map);

    let mut just_numbers = Vec::new();
    for p in map {
        just_numbers.push(p.1);
    }

    just_numbers.sort();

    println!(
        "{:?}",
        just_numbers.last().unwrap() - just_numbers.first().unwrap()
    );

    Ok(())
}
