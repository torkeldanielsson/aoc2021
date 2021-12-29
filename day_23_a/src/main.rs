fn score_val(c: char) -> i32 {
    match c {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
        _ => panic!("? {}", c),
    }
}

fn permutate(
    rooms: Vec<Vec<char>>,
    hallway: Vec<char>,
    scores: &mut Vec<i32>,
    score: i32,
    min_score: &mut i32,
) {
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

        if score < *min_score {
            *min_score = score;
            println!("{}", score);
        }
        return;
    }

    
        println!("#############    score: {}", score);
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
    

    let home = vec!['A', 'B', 'C', 'D'];

    for r in 0..4 {
        for i in 0..=1 {
            let mut ok_to_move = false;

            if i == 0 && (rooms[r][0] != home[r] && rooms[r][0] != ' ' && rooms[r][1] == ' ') {
                ok_to_move = true;
            }
            if i == 1
                && ((rooms[r][1] != home[r] && rooms[r][1] != ' ')
                    || (rooms[r][1] == home[r] && rooms[r][0] != home[r]))
            {
                ok_to_move = true;
            }

            if ok_to_move {
                if (r == 0 && hallway[0] == ' ' && hallway[1] == ' ')
                    || (r == 1 && hallway[0] == ' ' && hallway[1] == ' ' && hallway[2] == ' ')
                    || (r == 2
                        && hallway[0] == ' '
                        && hallway[1] == ' '
                        && hallway[2] == ' '
                        && hallway[3] == ' ')
                    || (r == 3
                        && hallway[0] == ' '
                        && hallway[1] == ' '
                        && hallway[2] == ' '
                        && hallway[3] == ' '
                        && hallway[4] == ' ')
                {
                    let mut rooms_clone = rooms.clone();
                    rooms_clone[r][i] = ' ';
                    let mut hallway_clone = hallway.clone();
                    hallway_clone[0] = rooms[r][i];
                    let distance = vec![4, 6, 8, 10];
                    permutate(
                        rooms_clone,
                        hallway_clone,
                        scores,
                        score + score_val(rooms[r][i]) * (distance[r] - i as i32),
                        min_score,
                    );
                }
                if (r == 0 && hallway[1] == ' ')
                    || (r == 1 && hallway[1] == ' ' && hallway[2] == ' ')
                    || (r == 2 && hallway[1] == ' ' && hallway[2] == ' ' && hallway[3] == ' ')
                    || (r == 3
                        && hallway[1] == ' '
                        && hallway[2] == ' '
                        && hallway[3] == ' '
                        && hallway[4] == ' ')
                {
                    let mut rooms_clone = rooms.clone();
                    rooms_clone[r][i] = ' ';
                    let mut hallway_clone = hallway.clone();
                    hallway_clone[1] = rooms[r][i];
                    let distance = vec![3, 5, 7, 9];
                    permutate(
                        rooms_clone,
                        hallway_clone,
                        scores,
                        score + score_val(rooms[r][i]) * (distance[r] - i as i32),
                        min_score,
                    );
                }
                if ((r == 0 || r == 1) && hallway[2] == ' ')
                    || (r == 2 && hallway[2] == ' ' && hallway[3] == ' ')
                    || (r == 3 && hallway[2] == ' ' && hallway[3] == ' ' && hallway[4] == ' ')
                {
                    let mut rooms_clone = rooms.clone();
                    rooms_clone[r][i] = ' ';
                    let mut hallway_clone = hallway.clone();
                    hallway_clone[2] = rooms[r][i];
                    let distance = vec![3, 3, 5, 7];
                    permutate(
                        rooms_clone,
                        hallway_clone,
                        scores,
                        score + score_val(rooms[r][i]) * (distance[r] - i as i32),
                        min_score,
                    );
                }
                if (r == 0 && hallway[2] == ' ' && hallway[3] == ' ')
                    || ((r == 1 || r == 2) && hallway[3] == ' ')
                    || (r == 3 && hallway[3] == ' ' && hallway[4] == ' ')
                {
                    let mut rooms_clone = rooms.clone();
                    rooms_clone[r][i] = ' ';
                    let mut hallway_clone = hallway.clone();
                    hallway_clone[3] = rooms[r][i];
                    let distance = vec![5, 3, 3, 5];
                    permutate(
                        rooms_clone,
                        hallway_clone,
                        scores,
                        score + score_val(rooms[r][i]) * (distance[r] - i as i32),
                        min_score,
                    );
                }
                if (r == 0 && hallway[2] == ' ' && hallway[3] == ' ' && hallway[4] == ' ')
                    || (r == 1 && hallway[3] == ' ' && hallway[4] == ' ')
                    || ((r == 2 || r == 3) && hallway[4] == ' ')
                {
                    let mut rooms_clone = rooms.clone();
                    rooms_clone[r][i] = ' ';
                    let mut hallway_clone = hallway.clone();
                    hallway_clone[4] = rooms[r][i];
                    let distance = vec![7, 5, 3, 3];
                    permutate(
                        rooms_clone,
                        hallway_clone,
                        scores,
                        score + score_val(rooms[r][i]) * (distance[r] - i as i32),
                        min_score,
                    );
                }
                if (r == 0
                    && hallway[2] == ' '
                    && hallway[3] == ' '
                    && hallway[4] == ' '
                    && hallway[5] == ' ')
                    || (r == 1 && hallway[3] == ' ' && hallway[4] == ' ' && hallway[5] == ' ')
                    || (r == 2 && hallway[4] == ' ' && hallway[5] == ' ')
                    || (r == 3 && hallway[5] == ' ')
                {
                    let mut rooms_clone = rooms.clone();
                    rooms_clone[r][i] = ' ';
                    let mut hallway_clone = hallway.clone();
                    hallway_clone[5] = rooms[r][i];
                    let distance = vec![9, 7, 5, 3];
                    permutate(
                        rooms_clone,
                        hallway_clone,
                        scores,
                        score + score_val(rooms[r][i]) * (distance[r] - i as i32),
                        min_score,
                    );
                }
                if (r == 0
                    && hallway[2] == ' '
                    && hallway[3] == ' '
                    && hallway[4] == ' '
                    && hallway[5] == ' '
                    && hallway[6] == ' ')
                    || (r == 1
                        && hallway[3] == ' '
                        && hallway[4] == ' '
                        && hallway[5] == ' '
                        && hallway[6] == ' ')
                    || (r == 2 && hallway[4] == ' ' && hallway[5] == ' ' && hallway[6] == ' ')
                    || (r == 3 && hallway[5] == ' ' && hallway[6] == ' ')
                {
                    let mut rooms_clone = rooms.clone();
                    rooms_clone[r][i] = ' ';
                    let mut hallway_clone = hallway.clone();
                    hallway_clone[6] = rooms[r][i];
                    let distance = vec![10, 8, 6, 4];
                    permutate(
                        rooms_clone,
                        hallway_clone,
                        scores,
                        score + score_val(rooms[r][i]) * (distance[r] - i as i32),
                        min_score,
                    );
                }
            }
        }
    }

    'h_loop: for h in 0..7 {
        if hallway[h] != ' ' {
            let r = match hallway[h] {
                'A' => 0,
                'B' => 1,
                'C' => 2,
                'D' => 3,
                _ => panic!(),
            };

            let i = if rooms[r][0] == ' ' && rooms[r][1] == ' ' {
                0
            } else {
                1
            };
            if i == 1 && (rooms[r][1] != ' ' || rooms[r][0] != home[r]) {
                continue 'h_loop;
            }

            let r_distance = match h {
                0 => vec![4, 6, 8, 10],
                1 => vec![3, 5, 7, 9],
                2 => vec![3, 3, 5, 7],
                3 => vec![5, 3, 3, 5],
                4 => vec![7, 5, 3, 3],
                5 => vec![9, 7, 5, 3],
                6 => vec![10, 8, 6, 4],
                _ => panic!(),
            };

            let mut rooms_clone = rooms.clone();
            rooms_clone[r][i] = hallway[h];
            let mut hallway_clone = hallway.clone();
            hallway_clone[h] = ' ';
            permutate(
                rooms_clone,
                hallway_clone,
                scores,
                score + score_val(hallway[h]) * (r_distance[r] - i as i32),
                min_score,
            );
        }
    }
}

fn main() {
    let rooms = vec![
        vec!['A', 'B'],
        vec!['D', 'C'],
        vec!['C', 'B'],
        vec!['A', 'D'],
    ];

    let hallway = vec![' '; 7];

    let mut scores = Vec::new();

    let mut min_score = i32::MAX;

    permutate(rooms, hallway, &mut scores, 0, &mut min_score);

    println!("scores: {:?}", scores);
}
