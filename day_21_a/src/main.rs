use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut turn: usize = 0;
    let mut die: u64 = 0;

    let mut pos = vec![6, 7];

    let mut score = vec![0 as u64; 2];

    while score[0] < 1000 && score[1] < 1000 {
        let add1 = die % 100 + 1;
        die += 1;
        let add2 = die % 100 + 1;
        die += 1;
        let add3 = die % 100 + 1;
        die += 1;
        let player = turn % 2;
        pos[player] = (((pos[player] + add1 + add2 + add3) - 1) % 10) + 1;
        score[player] += pos[player];

        turn += 1;

        println!(
            "Player {} rolls {}+{}+{} and moves to space {}, for a total score of {}",
            player + 1,
            add1,
            add2,
            add3,
            pos[player],
            score[player]
        );
    }

    println!("{} * {} = {}", score[turn % 2], die, score[turn % 2] * die);

    Ok(())
}
