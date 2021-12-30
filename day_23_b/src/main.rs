use std::collections::HashSet;

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
    visited_states: &mut HashSet<(Vec<Vec<char>>, Vec<char>, i32)>,
    mut history: Vec<(Vec<Vec<char>>, Vec<char>, i32)>,
) {
    let home = vec!['A', 'B', 'C', 'D'];

    let state = (rooms.clone(), hallway.clone(), score);

    history.push(state.clone());

    if visited_states.contains(&state) {
        return;
    }
    visited_states.insert(state);

    {
        let mut all_ok = true;
        'chk_outer: for r in 0..4 {
            for i in 0..4 {
                if rooms[r][i] != home[r] {
                    all_ok = false;
                    break 'chk_outer;
                }
            }
        }

        if all_ok {
            scores.push(score);

            if score < *min_score {
                *min_score = score;

                for step in history {
                    println!("#############  score: {}", step.2);
                    println!(
                        "#{}{} {} {} {} {}{}#",
                        step.1[0], step.1[1], step.1[2], step.1[3], step.1[4], step.1[5], step.1[6]
                    );
                    println!(
                        "###{}#{}#{}#{}###",
                        step.0[0][1], step.0[1][1], step.0[2][1], step.0[3][1]
                    );
                    println!(
                        "  #{}#{}#{}#{}#",
                        step.0[0][0], step.0[1][0], step.0[2][0], step.0[3][0]
                    );
                    println!("  ########");
                    println!();
                }

                println!("{}", score);
            }
            return;
        }
    }

    for r in 0..4 {
        for i in 0..4 {
            let mut ok_to_move = false;

            if i == 0
                && (rooms[r][0] != home[r]
                    && rooms[r][0] != ' '
                    && rooms[r][1] == ' '
                    && rooms[r][2] == ' '
                    && rooms[r][3] == ' ')
            {
                ok_to_move = true;
            }
            if i == 1
                && ((rooms[r][1] != home[r]
                    && rooms[r][1] != ' '
                    && rooms[r][2] == ' '
                    && rooms[r][3] == ' ')
                    || (rooms[r][1] == home[r]
                        && rooms[r][0] != home[r]
                        && rooms[r][2] == ' '
                        && rooms[r][3] == ' '))
            {
                ok_to_move = true;
            }
            if i == 2
                && ((rooms[r][2] != home[r] && rooms[r][2] != ' ' && rooms[r][3] == ' ')
                    || (rooms[r][2] == home[r]
                        && (rooms[r][0] != home[r] || rooms[r][1] != home[r])
                        && rooms[r][3] == ' '))
            {
                ok_to_move = true;
            }
            if i == 3
                && ((rooms[r][3] != home[r] && rooms[r][3] != ' ')
                    || (rooms[r][3] == home[r]
                        && (rooms[r][0] != home[r]
                            || rooms[r][1] != home[r]
                            || rooms[r][2] != home[r])))
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
                    let distance = vec![6, 8, 10, 12];

                    permutate(
                        rooms_clone,
                        hallway_clone,
                        scores,
                        score + score_val(rooms[r][i]) * (distance[r] - i as i32),
                        min_score,
                        visited_states,
                        history.clone(),
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
                    let distance = vec![5, 7, 9, 11];
                    permutate(
                        rooms_clone,
                        hallway_clone,
                        scores,
                        score + score_val(rooms[r][i]) * (distance[r] - i as i32),
                        min_score,
                        visited_states,
                        history.clone(),
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
                    let distance = vec![5, 5, 7, 9];
                    permutate(
                        rooms_clone,
                        hallway_clone,
                        scores,
                        score + score_val(rooms[r][i]) * (distance[r] - i as i32),
                        min_score,
                        visited_states,
                        history.clone(),
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
                    let distance = vec![7, 5, 5, 7];
                    permutate(
                        rooms_clone,
                        hallway_clone,
                        scores,
                        score + score_val(rooms[r][i]) * (distance[r] - i as i32),
                        min_score,
                        visited_states,
                        history.clone(),
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
                    let distance = vec![9, 7, 5, 5];
                    permutate(
                        rooms_clone,
                        hallway_clone,
                        scores,
                        score + score_val(rooms[r][i]) * (distance[r] - i as i32),
                        min_score,
                        visited_states,
                        history.clone(),
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
                    let distance = vec![11, 9, 7, 5];
                    permutate(
                        rooms_clone,
                        hallway_clone,
                        scores,
                        score + score_val(rooms[r][i]) * (distance[r] - i as i32),
                        min_score,
                        visited_states,
                        history.clone(),
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
                    let distance = vec![12, 10, 8, 6];
                    permutate(
                        rooms_clone,
                        hallway_clone,
                        scores,
                        score + score_val(rooms[r][i]) * (distance[r] - i as i32),
                        min_score,
                        visited_states,
                        history.clone(),
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

            let i = if rooms[r][0] == ' '
                && rooms[r][1] == ' '
                && rooms[r][2] == ' '
                && rooms[r][3] == ' '
            {
                0
            } else if rooms[r][0] == home[r]
                && rooms[r][1] == ' '
                && rooms[r][2] == ' '
                && rooms[r][3] == ' '
            {
                1
            } else if rooms[r][0] == home[r]
                && rooms[r][1] == home[r]
                && rooms[r][2] == ' '
                && rooms[r][3] == ' '
            {
                2
            } else {
                3
            };
            if i == 3
                && (rooms[r][3] != ' '
                    || rooms[r][0] != home[r]
                    || rooms[r][1] != home[r]
                    || rooms[r][2] != home[r])
            {
                continue 'h_loop;
            }

            let r_distance = match h {
                0 => vec![6, 8, 10, 12],
                1 => vec![5, 7, 9, 11],
                2 => vec![5, 5, 7, 9],
                3 => vec![7, 5, 5, 7],
                4 => vec![9, 7, 5, 5],
                5 => vec![11, 9, 7, 5],
                6 => vec![12, 10, 8, 6],
                _ => panic!(),
            };

            if h == 0 && hallway[1] != ' ' {
                continue 'h_loop;
            }
            if h <= 1 && r >= 1 && hallway[2] != ' ' {
                continue 'h_loop;
            }
            if h <= 2 && r >= 2 && hallway[3] != ' ' {
                continue 'h_loop;
            }
            if h <= 3 && r >= 3 && hallway[4] != ' ' {
                continue 'h_loop;
            }

            if h == 6 && hallway[5] != ' ' {
                continue 'h_loop;
            }
            if h >= 5 && r <= 2 && hallway[4] != ' ' {
                continue 'h_loop;
            }
            if h >= 4 && r <= 1 && hallway[3] != ' ' {
                continue 'h_loop;
            }
            if h >= 3 && r <= 0 && hallway[2] != ' ' {
                continue 'h_loop;
            }

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
                visited_states,
                history.clone(),
            );
        }
    }
}

fn main() {
    /*
    let rooms = vec![
        vec!['A', 'D', 'D', 'B'],
        vec!['D', 'B', 'C', 'C'],
        vec!['C', 'A', 'B', 'B'],
        vec!['A', 'C', 'A', 'D'],
    ];
    */
    let rooms = vec![
        vec!['C', 'D', 'D', 'A'],
        vec!['D', 'B', 'C', 'D'],
        vec!['B', 'A', 'B', 'A'],
        vec!['B', 'C', 'A', 'C'],
    ];

    let hallway = vec![' '; 7];

    let mut scores = Vec::new();

    let mut min_score = i32::MAX;

    let mut visited_states = HashSet::new();

    permutate(
        rooms,
        hallway,
        &mut scores,
        0,
        &mut min_score,
        &mut visited_states,
        Vec::new(),
    );

    println!("lowest: {}", min_score);
    println!("visited_states count:  {}", visited_states.len());
}
