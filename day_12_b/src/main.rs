use std::{collections::HashMap, collections::HashSet, error::Error, fs};

fn solve(
    counter: &mut i32,
    mut history: Vec<String>,
    has_used_double: bool,
    rules: &HashMap<String, HashSet<String>>,
) {
    let nexts = rules.get(history.last().unwrap()).unwrap();
    // println!("{:?}", nexts);
    for next in nexts {
        if next != "start" {
            if next == "end" {
                history.push(next.to_owned());
                // println!("{:?}", history);
                *counter += 1;
            } else {
                if next.chars().next().unwrap().is_ascii_lowercase()
                    && history.contains(next)
                    && !has_used_double
                {
                    let mut new_history = history.clone();
                    new_history.push(next.to_owned());
                    solve(counter, new_history, true, rules);
                }

                if (next.chars().next().unwrap().is_ascii_lowercase() && !history.contains(next))
                    || next.chars().next().unwrap().is_ascii_uppercase()
                {
                    let mut new_history = history.clone();
                    new_history.push(next.to_owned());
                    solve(counter, new_history, has_used_double, rules);
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut rules: HashMap<String, HashSet<String>> = HashMap::new();

    for line in fs::read_to_string("input")?.lines() {
        let parts: Vec<&str> = line.split("-").collect();
        if !rules.contains_key(parts[0]) {
            rules.insert(parts[0].to_owned(), HashSet::new());
        }
        rules.get_mut(parts[0]).unwrap().insert(parts[1].to_owned());
    }

    {
        let stupid_copy = rules.clone();
        for (from, tos) in &stupid_copy {
            for to in tos {
                if !rules.contains_key(to) {
                    rules.insert(to.to_string(), HashSet::new());
                }

                rules.get_mut(to).unwrap().insert(from.to_owned());
            }
        }
    }

    // println!("{:?}", rules);

    let mut counter = 0;

    solve(&mut counter, vec!["start".to_owned()], false, &rules);

    println!("count: {:?}", counter);

    Ok(())
}
