fn score_val(c: char) -> i32 {
    match c {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
        _ => panic!("? {}", c),
    }
}

fn permutate(rooms: Vec<Vec<char>>, hallway: Vec<char>, scores: &mut Vec<i32>, score: i32) {
    if rooms[0][0] == 'A'
        && rooms[0][1] == 'A'
        && rooms[1][0] == 'B'
        && rooms[1][1] == 'B'
        && rooms[2][0] == 'C'
        && rooms[2][1] == 'C'
        && rooms[3][0] == 'D'
        && rooms[3][1] == 'D'
    {
        scores.push(score);
        return;
    }

    println!("#############");
    println!(
        "#{}{} {} {} {} {}{}#",
        hallway[0], hallway[1], hallway[2], hallway[3], hallway[4], hallway[5], hallway[6]
    );
    println!(
        "###{}#{}#{}#{}###",
        rooms[0][1], rooms[1][1], rooms[2][1], rooms[3][1]
    );
    println!(
        "  #{}#{}#{}#{}#",
        rooms[0][0], rooms[1][0], rooms[2][0], rooms[3][0]
    );
    println!("  ########");
    println!();

    for i in 0..=1 {
        let mut ok_to_move = false;

        if i == 0 && (rooms[0][0] != 'A' && rooms[0][0] != ' ' && rooms[0][1] == ' ') {
            ok_to_move = true;
        }
        if i == 1
            && ((rooms[0][1] != 'A' && rooms[0][1] != ' ')
                || (rooms[0][1] == 'A' && rooms[0][0] != 'A'))
        {
            ok_to_move = true;
        }

        if ok_to_move {
            if hallway[0] == ' ' && hallway[1] == ' ' {
                let mut rooms_clone = rooms.clone();
                rooms_clone[0][i] = ' ';
                let mut hallway_clone = hallway.clone();
                hallway_clone[0] = rooms[0][i];
                permutate(
                    rooms_clone,
                    hallway_clone,
                    scores,
                    score + score_val(rooms[0][i]) * (4 - i as i32),
                );

                let mut rooms_clone = rooms.clone();
                rooms_clone[0][i] = ' ';
                let mut hallway_clone = hallway.clone();
                hallway_clone[1] = rooms[0][i];
                permutate(
                    rooms_clone,
                    hallway_clone,
                    scores,
                    score + score_val(rooms[0][i]) * (3 - i as i32),
                );
            }
            if hallway[2] == ' ' {
                let mut rooms_clone = rooms.clone();
                rooms_clone[0][i] = ' ';
                let mut hallway_clone = hallway.clone();
                hallway_clone[2] = rooms[0][i];
                permutate(
                    rooms_clone,
                    hallway_clone,
                    scores,
                    score + score_val(rooms[0][i]) * (3 - i as i32),
                );
            }
            if hallway[2] == ' ' && hallway[3] == ' ' {
                let mut rooms_clone = rooms.clone();
                rooms_clone[0][i] = ' ';
                let mut hallway_clone = hallway.clone();
                hallway_clone[3] = rooms[0][i];
                permutate(
                    rooms_clone,
                    hallway_clone,
                    scores,
                    score + score_val(rooms[0][i]) * (5 - i as i32),
                );
            }
            if hallway[2] == ' ' && hallway[3] == ' ' && hallway[4] == ' ' {
                let mut rooms_clone = rooms.clone();
                rooms_clone[0][i] = ' ';
                let mut hallway_clone = hallway.clone();
                hallway_clone[4] = rooms[0][i];
                permutate(
                    rooms_clone,
                    hallway_clone,
                    scores,
                    score + score_val(rooms[0][i]) * (7 - i as i32),
                );
            }
            if hallway[2] == ' ' && hallway[3] == ' ' && hallway[4] == ' ' && hallway[5] == ' ' {
                let mut rooms_clone = rooms.clone();
                rooms_clone[0][i] = ' ';
                let mut hallway_clone = hallway.clone();
                hallway_clone[5] = rooms[0][i];
                permutate(
                    rooms_clone,
                    hallway_clone,
                    scores,
                    score + score_val(rooms[0][i]) * (9 - i as i32),
                );
            }
            if hallway[2] == ' '
                && hallway[3] == ' '
                && hallway[4] == ' '
                && hallway[5] == ' '
                && hallway[6] == ' '
            {
                let mut rooms_clone = rooms.clone();
                rooms_clone[0][i] = ' ';
                let mut hallway_clone = hallway.clone();
                hallway_clone[6] = rooms[0][i];
                permutate(
                    rooms_clone,
                    hallway_clone,
                    scores,
                    score + score_val(rooms[0][i]) * (10 - i as i32),
                );
            }
        }
    }
}

fn main() {
    let rooms = vec![
        vec!['B', 'A'],
        vec!['D', 'C'],
        vec!['C', 'B'],
        vec!['D', 'A'],
    ];

    let hallway = vec![' '; 7];

    let mut scores = Vec::new();

    permutate(rooms, hallway, &mut scores, 0);

    println!("scores: {:?}", scores);
}
