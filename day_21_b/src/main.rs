use std::{collections::HashMap, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let mut universes: HashMap<(u8, u8, u8, u8), u64> = HashMap::new();

    // (score1, pos1, score2, pos2)
    universes.insert((0, 5, 0, 6), 1);

    let mut is_done = false;
    let mut is_player1_turn = true;

    let mut player_1_wins: u64 = 0;
    let mut player_2_wins: u64 = 0;

    while !is_done {
        is_done = true;

        let old_universes = universes.clone();
        universes.clear();

        for uc in old_universes {
            let u = &uc.0;
            if u.0 >= 21 {
                player_1_wins += uc.1 as u64;
            } else if u.2 >= 21 {
                player_2_wins += uc.1 as u64;
            } else {
                is_done = false;

                if is_player1_turn {
                    *universes.entry((u.0 + (u.1 + 3) % 10 + 1, (u.1 + 3) % 10, u.2, u.3)).or_insert(0) += 1 * uc.1;
                    *universes.entry((u.0 + (u.1 + 4) % 10 + 1, (u.1 + 4) % 10, u.2, u.3)).or_insert(0) += 3 * uc.1;
                    *universes.entry((u.0 + (u.1 + 5) % 10 + 1, (u.1 + 5) % 10, u.2, u.3)).or_insert(0) += 6 * uc.1;
                    *universes.entry((u.0 + (u.1 + 6) % 10 + 1, (u.1 + 6) % 10, u.2, u.3)).or_insert(0) += 7 * uc.1;
                    *universes.entry((u.0 + (u.1 + 7) % 10 + 1, (u.1 + 7) % 10, u.2, u.3)).or_insert(0) += 6 * uc.1;
                    *universes.entry((u.0 + (u.1 + 8) % 10 + 1, (u.1 + 8) % 10, u.2, u.3)).or_insert(0) += 3 * uc.1;
                    *universes.entry((u.0 + (u.1 + 9) % 10 + 1, (u.1 + 9) % 10, u.2, u.3)).or_insert(0) += 1 * uc.1;
                } else {
                    *universes.entry((u.0, u.1, u.2 + (u.3 + 3) % 10 + 1, (u.3 + 3) % 10)).or_insert(0) += 1 * uc.1;
                    *universes.entry((u.0, u.1, u.2 + (u.3 + 4) % 10 + 1, (u.3 + 4) % 10)).or_insert(0) += 3 * uc.1;
                    *universes.entry((u.0, u.1, u.2 + (u.3 + 5) % 10 + 1, (u.3 + 5) % 10)).or_insert(0) += 6 * uc.1;
                    *universes.entry((u.0, u.1, u.2 + (u.3 + 6) % 10 + 1, (u.3 + 6) % 10)).or_insert(0) += 7 * uc.1;
                    *universes.entry((u.0, u.1, u.2 + (u.3 + 7) % 10 + 1, (u.3 + 7) % 10)).or_insert(0) += 6 * uc.1;
                    *universes.entry((u.0, u.1, u.2 + (u.3 + 8) % 10 + 1, (u.3 + 8) % 10)).or_insert(0) += 3 * uc.1;
                    *universes.entry((u.0, u.1, u.2 + (u.3 + 9) % 10 + 1, (u.3 + 9) % 10)).or_insert(0) += 1 * uc.1;
                }
            }
        }

        is_player1_turn = !is_player1_turn;

        println!("{} - {}", player_1_wins, player_2_wins);
    }

    /*
        let mut vs = Vec::new();

        for i1 in 1..=3 {
            for i2 in 1..=3 {
                for i3 in 1..=3 {
                    vs.push(i1 + i2 + i3);
                }
            }
        }

        for i in 3..=9 {
            let mut sum = 0;
            for v in &vs {
                if *v == i {
                    sum += 1;
                }
            }
            println!("{}: {}", i, sum);
        }

    3: 1
    4: 3
    5: 6
    6: 7
    7: 6
    8: 3
    9: 1
        */

    Ok(())
}
