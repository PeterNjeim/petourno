pub fn single_elimination(form: Form, result: String) -> Result {
    let players = form.players;

    if result == "sets" {
        return Result::Sets(players);
    }

    let mut p_count = players.len() as f64;
    let mut res = 0;
    while p_count > 1.0 {
        res += 1;
        p_count = p_count / 2.0;
    }

    if result == "res" {
        return Result::Res(res);
    }

    let b_size = 1 << (res);

    if result == "bSize" {
        return Result::BSize(b_size as u64);
    }

    let byes = b_size - players.len();

    if result == "byes" {
        return Result::Byes(byes as u64);
    }

    let sets_r1 = players.len() - (b_size >> 1);

    if result == "setsR1" {
        return Result::SetsR1(sets_r1 as u64);
    }

    let mut sets_array: Vec<String> = Vec::with_capacity(b_size);
    unsafe { sets_array.set_len(b_size) }
    let mut sets_array2: Vec<String> = Vec::with_capacity(b_size >> 1);
    unsafe { sets_array2.set_len(b_size >> 1) }
    let mut order_array: Vec<String> = Vec::with_capacity(b_size >> 1);
    unsafe { order_array.set_len(b_size >> 1) }
    let mut order_array2: Vec<String> = Vec::with_capacity(b_size >> 2);
    unsafe { order_array2.set_len(b_size >> 2) }

    for i in 0..b_size {
        if i < byes {
            sets_array[i] = "\n".to_string();
        } else if i < players.len() {
            sets_array[i] = players[i].clone();
        } else {
            sets_array[i] = "\n".to_string();
        }
    }
    for i in 0..(b_size >> 1) {
        if i < byes {
            sets_array2[i] = players[i].clone();
        } else {
            sets_array2[i] = "\n".to_string();
        }
    }

    if result == "setsArray" {
        return Result::SetsArray(sets_array);
    }

    if result == "setsArray2" {
        return Result::SetsArray2(sets_array2);
    }

    for i in 0..(b_size >> 1) {
        order_array[i] = sets_array[(triangle(res + 1, 2 * i as u64 + 1) - 1) as usize].clone()
            + "\n"
            + &sets_array[(triangle(res + 1, 2 * i as u64 + 2) - 1) as usize];
    }
    for i in 0..(b_size >> 2) {
        order_array2[i] = sets_array2[(triangle(res, 2 * i as u64 + 1) - 1) as usize].clone()
            + "\n"
            + &sets_array2[(triangle(res, 2 * i as u64 + 2) - 1) as usize];
    }

    let mut sets_players: Vec<Set> = Vec::new();

    order_array.iter().enumerate().for_each(|(index, set)| {
        if !set.lines().map(|s| s.to_owned()).collect::<Vec<String>>()[0].is_empty() {
            sets_players.push(Set {
                bracket: 0,
                round: 0,
                position: index as u64,
                players: vec![
                    set.lines().map(|s| s.to_owned()).collect::<Vec<String>>()[0].clone(),
                    set.lines().map(|s| s.to_owned()).collect::<Vec<String>>()[1].clone(),
                ],
                scores: vec![0, 0],
                winners: vec!["".to_string(), "".to_string()],
                losers: vec!["".to_string(), "".to_string()],
                outcomes: vec!["".to_string(), "eliminated".to_string()],
                status: "preview".to_string(),
            })
        }
    });

    order_array2.iter().enumerate().for_each(|(index, set)| {
        if !set.lines().map(|s| s.to_owned()).collect::<Vec<String>>()[1].is_empty() {
            sets_players.push(Set {
                bracket: 0,
                round: 1,
                position: index as u64,
                players: vec![
                    set.lines().map(|s| s.to_owned()).collect::<Vec<String>>()[0].clone(),
                    set.lines().map(|s| s.to_owned()).collect::<Vec<String>>()[1].clone(),
                ],
                scores: vec![0, 0],
                winners: vec!["".to_string(), "".to_string()],
                losers: vec!["".to_string(), "".to_string()],
                outcomes: vec!["".to_string(), "eliminated".to_string()],
                status: "preview".to_string(),
            })
        }
    });

    let mut bye_array: Vec<u64> = Vec::new();
    order_array2.iter().enumerate().for_each(|(index, set)| {
        if set.lines().map(|s| s.to_owned()).collect::<Vec<String>>()[1].is_empty() {
            sets_players.push(Set {
                bracket: 0,
                round: 1,
                position: index as u64,
                players: vec![
                    set.lines().map(|s| s.to_owned()).collect::<Vec<String>>()[0].clone(),
                    set.lines().map(|s| s.to_owned()).collect::<Vec<String>>()[1].clone(),
                ],
                scores: vec![0, 0],
                winners: vec!["".to_string(), "".to_string()],
                losers: vec!["".to_string(), "".to_string()],
                outcomes: vec!["".to_string(), "eliminated".to_string()],
                status: "preview".to_string(),
            });
            if index % 2 == 0 {
                bye_array.push(index as u64 / 2);
            }
            if sets_players
                .iter()
                .position(|obj| obj.round == 0 && obj.position == index as u64 * 2)
                .is_some()
            {
                let sets_players_index = sets_players
                    .iter()
                    .position(|obj| obj.round == 0 && obj.position == index as u64 * 2)
                    .unwrap();
                sets_players[sets_players_index].outcomes[0] = (sets_players.len() - 1).to_string();
            }
            if sets_players
                .iter()
                .position(|obj| obj.round == 0 && obj.position == index as u64 * 2 + 1)
                .is_some()
            {
                let sets_players_index = sets_players
                    .iter()
                    .position(|obj| obj.round == 0 && obj.position == index as u64 * 2 + 1)
                    .unwrap();
                sets_players[sets_players_index].outcomes[0] = (sets_players.len() - 1).to_string();
            }
        }
    });

    for i in 2..res {
        for j in 0..(b_size >> (i + 1)) {
            if !bye_array.contains(&(j as u64)) {
                sets_players.push(Set {
                    bracket: 0,
                    round: i as u64,
                    position: j as u64,
                    players: vec!["".to_string(), "".to_string()],
                    scores: vec![0, 0],
                    winners: vec!["".to_string(), "".to_string()],
                    losers: vec!["".to_string(), "".to_string()],
                    outcomes: vec!["".to_string(), "eliminated".to_string()],
                    status: "preview".to_string(),
                });
                if sets_players
                    .iter()
                    .position(|obj| obj.round == i as u64 - 1 && obj.position == j as u64 * 2)
                    .is_some()
                {
                    let sets_players_index = sets_players
                        .iter()
                        .position(|obj| obj.round == i as u64 - 1 && obj.position == j as u64 * 2)
                        .unwrap();
                    sets_players[sets_players_index].outcomes[0] =
                        (sets_players.len() - 1).to_string();
                }
                if sets_players
                    .iter()
                    .position(|obj| obj.round == i as u64 - 1 && obj.position == j as u64 * 2 + 1)
                    .is_some()
                {
                    let sets_players_index = sets_players
                        .iter()
                        .position(|obj| {
                            obj.round == i as u64 - 1 && obj.position == j as u64 * 2 + 1
                        })
                        .unwrap();
                    sets_players[sets_players_index].outcomes[0] =
                        (sets_players.len() - 1).to_string();
                }
            }
        }

        for j in 0..(b_size >> (i + 1)) {
            if bye_array.contains(&(j as u64)) {
                sets_players.push(Set {
                    bracket: 0,
                    round: i as u64,
                    position: j as u64,
                    players: vec!["".to_string(), "".to_string()],
                    scores: vec![0, 0],
                    winners: vec!["".to_string(), "".to_string()],
                    losers: vec!["".to_string(), "".to_string()],
                    outcomes: vec!["".to_string(), "eliminated".to_string()],
                    status: "preview".to_string(),
                });
                if j % 2 == 0 {
                    bye_array[j] = j as u64 / 2;
                }
                if sets_players
                    .iter()
                    .position(|obj| obj.round == i as u64 - 1 && obj.position == j as u64 * 2)
                    .is_some()
                {
                    let sets_players_index = sets_players
                        .iter()
                        .position(|obj| obj.round == i as u64 - 1 && obj.position == j as u64 * 2)
                        .unwrap();
                    sets_players[sets_players_index].outcomes[0] =
                        (sets_players.len() - 1).to_string();
                }
                if sets_players
                    .iter()
                    .position(|obj| obj.round == i as u64 - 1 && obj.position == j as u64 * 2 + 1)
                    .is_some()
                {
                    let sets_players_index = sets_players
                        .iter()
                        .position(|obj| {
                            obj.round == i as u64 - 1 && obj.position == j as u64 * 2 + 1
                        })
                        .unwrap();
                    sets_players[sets_players_index].outcomes[0] =
                        (sets_players.len() - 1).to_string();
                }
            }
        }
    }

    let sets_players_index = sets_players.len() - 1;
    sets_players[sets_players_index].outcomes[0] = "won".to_string();

    let json_tournament = Tournament {
        name: form.name,
        description: form.description,
        game: form.game,
        tuple: form.tuple,
        style: form.style,
        players: players,
        sets: sets_players,
    };

    Result::JsonTournament(json_tournament)
}
