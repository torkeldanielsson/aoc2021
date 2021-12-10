use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let mut population = vec![0; 9];
    for n in fs::read_to_string("input")?
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
    {
        population[n] += 1;
    }

    for _step in 0..256 {
        let spawning = population[0];
        for i in 1..population.len() {
            population[i - 1] = population[i];
        }
        population[8] = spawning;
        population[6] += spawning;
    }

    println!("{:?} {}", population, population.iter().sum::<i64>());

    Ok(())
}
