use super::*;
use petgraph::dot::*;
use petgraph::stable_graph::*;
use rayon::prelude::*;
use std::iter::FromIterator;
use std::{cmp, fmt};

#[derive(Clone, Debug)]
pub struct GraphSet {
    pub bracket: u64,
    pub game: u64,
    pub round: u64,
    pub position: u64,
    pub placeholders: (String, String),
}

impl fmt::Display for GraphSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "{:?}", self)
    }
}

#[macro_export]
macro_rules! parent_game {
    ( $( $x:expr ),* ) => {
        {

            $(
                $x.0.node_indices().find(|index| {$x.0[*index].bracket == $x.1 && $x.0[*index].round == $x.2 - 1 && $x.0[*index].position == $x.3 * 2 - $x.4 % 2}).unwrap()
            )*
        }
    };
}

fn set_scan(mut graph: StableGraph<GraphSet, &str>, bracket: u64, round: u64) -> Vec<u64> {
    let mut round_sets = graph
        .node_weights_mut()
        .map(|set| set.to_owned())
        .filter(|set| set.bracket == bracket && set.round == round)
        .collect::<Vec<GraphSet>>();
    // println!("{:?}", round_sets);
    round_sets.sort_by(|a, b| a.position.cmp(&b.position));
    // println!("{:?}", round_sets);
    let mut largest_games: Vec<(u64, u64)> = Vec::new();
    round_sets.chunks(2).for_each(|sets| {
        largest_games.push((cmp::max(sets[0].game, sets[1].game), sets[1].position / 2))
    });
    // println!("{:?}", largest_games);
    largest_games.sort_by(|a, b| a.0.cmp(&b.0));
    // println!("{:?}", largest_games);
    let mut i: u64 = 0;
    largest_games.iter_mut().for_each(|f| {
        i += 1;
        f.0 = i;
    });
    // println!("{:?}", largest_games);
    largest_games.sort_by(|a, b| a.1.cmp(&b.1));
    // println!("{:?}", largest_games);
    let result = largest_games.iter().map(|f| f.0).collect::<Vec<u64>>();
    // println!("{:?}", result);
    result
}

pub fn new_single_elim(players: Vec<&str>) -> StableGraph<GraphSet, &str> {
    let players_count = players.len();
    // TODO: Check for overflowing value
    let full_first_round_sets_count = players_count.next_power_of_two() / 2;
    let exponent = ((full_first_round_sets_count * 2) as f64).log2() as u64;
    let full_sets_count = (0..exponent).map(|x| 2_u64.pow(x as u32)).sum::<u64>() as usize;

    // println!(
    //     "{:?}: players_count\n{:?}: full_first_round_sets_count\n{:?}: first_round_sets_count\n{:?}: exponent\n{:?}: full_sets_count\n{:?}: sets_count\n{:?}: byes_count",
    //     players_count,
    //     full_first_round_sets_count,
    //     first_round_sets_count,
    //     exponent,
    //     full_sets_count,
    //     sets_count,
    //     byes_count
    // );

    let mut sets_graph =
        StableGraph::<GraphSet, &str>::with_capacity(full_sets_count, full_sets_count - 1);
    let mut game_number: u64 = 0;
    for i in 0..full_first_round_sets_count {
        game_number += 1;
        if triangle(exponent + 1, 2 * i as u64 + 2) <= players_count as u64 {
            sets_graph.add_node(GraphSet {
                bracket: 1 as u64,
                game: game_number,
                round: 1,
                position: i as u64 + 1,
                placeholders: (
                    players[triangle(exponent + 1, 2 * i as u64 + 1) as usize - 1].to_owned(),
                    players[triangle(exponent + 1, 2 * i as u64 + 2) as usize - 1].to_owned(),
                ),
            });
        } else {
            game_number -= 1;
            sets_graph.add_node(GraphSet {
                bracket: 1 as u64,
                game: 0,
                round: 1,
                position: i as u64 + 1,
                placeholders: (0.to_string(), 0.to_string()),
            });
        }
    }
    let mut game_number_offset: Vec<u64> = Vec::new();
    for j in 1..exponent as usize {
        game_number += game_number_offset.len() as u64;
        game_number_offset = set_scan(sets_graph.clone(), 1 as u64, j as u64);
        for i in 0..(full_first_round_sets_count >> j) {
            sets_graph.add_node(GraphSet {
                bracket: 1 as u64,
                game: game_number + game_number_offset[i],
                round: j as u64 + 1,
                position: i as u64 + 1,
                placeholders: (
                    if sets_graph
                        .node_weight(parent_game!((
                            sets_graph.clone(),
                            1 as u64,
                            j as u64 + 1,
                            i as u64 + 1,
                            1 as u64
                        )))
                        .unwrap()
                        .game
                        == 0
                    {
                        players[triangle(exponent - j as u64 + 1, 2 * i as u64 + 1) as usize - 1]
                            .to_owned()
                    } else {
                        "W".to_owned()
                            + &sets_graph
                                .node_weight(parent_game!((
                                    sets_graph.clone(),
                                    1 as u64,
                                    j as u64 + 1,
                                    i as u64 + 1,
                                    1 as u64
                                )))
                                .unwrap()
                                .game
                                .to_string()
                    },
                    if sets_graph
                        .node_weight(parent_game!((
                            sets_graph.clone(),
                            1 as u64,
                            j as u64 + 1,
                            i as u64 + 1,
                            2 as u64
                        )))
                        .unwrap()
                        .game
                        == 0
                    {
                        players[triangle(exponent - j as u64 + 1, 2 * i as u64 + 2) as usize - 1]
                            .to_owned()
                    } else {
                        "W".to_owned()
                            + &sets_graph
                                .node_weight(parent_game!((
                                    sets_graph.clone(),
                                    1 as u64,
                                    j as u64 + 1,
                                    i as u64 + 1,
                                    2 as u64
                                )))
                                .unwrap()
                                .game
                                .to_string()
                    },
                ),
            });
            sets_graph.add_edge(
                parent_game!((
                    sets_graph.clone(),
                    1 as u64,
                    j as u64 + 1,
                    i as u64 + 1,
                    1 as u64
                )),
                NodeIndex::from(sets_graph.node_count() as u32 - 1),
                "winner",
            );
            sets_graph.add_edge(
                parent_game!((
                    sets_graph.clone(),
                    1 as u64,
                    j as u64 + 1,
                    i as u64 + 1,
                    2 as u64
                )),
                NodeIndex::from(sets_graph.node_count() as u32 - 1),
                "winner",
            );
        }
    }
    let mut reverse_indices = sets_graph.node_indices().collect::<Vec<NodeIndex>>();
    // println!("{:?}", reverse_indices);
    reverse_indices.par_sort_by(|a, b| b.cmp(&a));
    // println!("{:?}", reverse_indices);
    reverse_indices.iter().for_each(|index| {
        if sets_graph[*index].game == 0 {
            // println!("{:?}", index);
            sets_graph.remove_node(*index);
        }
    });
    println!(
        "{:?}",
        Dot::with_config(&sets_graph, &[Config::EdgeNoLabel])
    );
    sets_graph
}

pub fn new_double_elim(players: Vec<&str>) -> StableGraph<GraphSet, &str> {
    let players_count = players.len();
    if players_count < 2 {
        panic!(
            "Tournament needs at least 2 players, found {}",
            players_count
        );
    }
    // TODO: Check for overflowing value
    let full_first_bracket_first_round_sets_count = (players_count as u64).next_power_of_two() / 2;
    let first_bracket_exponent =
        ((full_first_bracket_first_round_sets_count * 2) as f64).log2() as u64;
    let full_first_bracket_sets_count = (0..first_bracket_exponent)
        .map(|x| 2_u64.pow(x as u32))
        .sum::<u64>() as usize;
    // TODO: Check for overflowing value
    let full_second_bracket_first_round_sets_count = u64::pow(
        2,
        ((players_count as f64 - 1.0).log(2.0).floor()
            + (2.0 * (players_count as f64 - 1.0) / 3.0).log(2.0).floor()) as u32
            - 1,
    );
    let semifull_second_bracket_first_round_sets_count = u64::pow(
        2,
        (1.0 + (((players_count as f64 - 1.0) / 3.0).log(2.0)).floor()) as u32,
    );
    let second_bracket_exponent =
        ((full_second_bracket_first_round_sets_count) as f64).log2() as u64;
    let is_small_second_bracket = if u64::pow(2, first_bracket_exponent as u32 - 2)
        + u64::pow(2, first_bracket_exponent as u32 - 1)
        >= players_count as u64
    {
        1
    } else {
        0
    };
    let mut losers_brackets_rounds_sets_positions: Vec<u64> = Vec::with_capacity(
        (semifull_second_bracket_first_round_sets_count / (2 - is_small_second_bracket)) as usize,
    );
    let mut losers_brackets_rounds_sets_positions_vector_offset: Vec<usize> =
        Vec::with_capacity((first_bracket_exponent + 3 - is_small_second_bracket) as usize);
    losers_brackets_rounds_sets_positions_vector_offset.push(0);
    for j in is_small_second_bracket..(2 * first_bracket_exponent - 2) {
        for i in 1..=(semifull_second_bracket_first_round_sets_count
            * (is_small_second_bracket + 1))
            >> (j as f64 / 2.0).floor() as u64 + 1
        {
            losers_brackets_rounds_sets_positions
                .push(losers_expand(2 * first_bracket_exponent - 2 - j, i));
        }
        losers_brackets_rounds_sets_positions_vector_offset
            .push(losers_brackets_rounds_sets_positions.len());
    }

    println!(
        "{:?}: players_count\n{:?}: full_first_bracket_first_round_sets_count\n{:?}: first_bracket_exponent\n{:?}: full_first_bracket_sets_count\n{:?}: full_second_bracket_first_round_sets_count\n{:?}: semifull_second_bracket_first_round_sets_count\n{:?}: second_bracket_exponent\n{:?}: losers_brackets_rounds_sets_positions\n{:?}: losers_brackets_rounds_sets_positions_vector_offset",
        players_count,
        full_first_bracket_first_round_sets_count,
        first_bracket_exponent,
        full_first_bracket_sets_count,
        full_second_bracket_first_round_sets_count,
        semifull_second_bracket_first_round_sets_count,
        second_bracket_exponent,
        losers_brackets_rounds_sets_positions,
        losers_brackets_rounds_sets_positions_vector_offset,
    );

    let mut sets_graph = StableGraph::<GraphSet, &str>::with_capacity(
        full_first_bracket_sets_count,
        full_first_bracket_sets_count - 1,
    );
    let mut game_number: u64 = 0;
    for i in 1..=full_first_bracket_first_round_sets_count as u64 {
        game_number += 1;
        if triangle(first_bracket_exponent + 1, 2 * i) <= players_count as u64 {
            sets_graph.add_node(GraphSet {
                bracket: 1,
                game: game_number,
                round: 1,
                position: i,
                placeholders: (
                    players[triangle(first_bracket_exponent + 1, 2 * i - 1) as usize - 1]
                        .to_owned(),
                    players[triangle(first_bracket_exponent + 1, 2 * i) as usize - 1].to_owned(),
                ),
            });
        } else {
            game_number -= 1;
            sets_graph.add_node(GraphSet {
                bracket: 1,
                game: 0,
                round: 1,
                position: i,
                placeholders: (0.to_string(), 0.to_string()),
            });
        }
    }

    if is_small_second_bracket == 0 {
        for i in 1..=full_second_bracket_first_round_sets_count as u64 {
            game_number += 1;
            if losers_brackets_rounds_sets_positions
                [0..(semifull_second_bracket_first_round_sets_count / 2) as usize]
                .contains(&i)
            {
                if sets_graph
                    .node_weight(parent_game!((
                        sets_graph.clone(),
                        1,
                        2,
                        losers_brackets_rounds_sets_positions
                            .par_iter()
                            .position_any(|&p| p == i)
                            .unwrap() as u64
                            + 1,
                        1
                    )))
                    .unwrap()
                    .game
                    != 0
                    && sets_graph
                        .node_weight(parent_game!((
                            sets_graph.clone(),
                            1,
                            2,
                            losers_brackets_rounds_sets_positions
                                .par_iter()
                                .position_any(|&p| p == i)
                                .unwrap() as u64
                                + 1,
                            2
                        )))
                        .unwrap()
                        .game
                        != 0
                {
                    sets_graph.add_node(GraphSet {
                        bracket: 2,
                        game: game_number,
                        round: 1,
                        position: i,
                        placeholders: (
                            "L".to_owned()
                                + &sets_graph
                                    .node_weight(parent_game!((
                                        sets_graph.clone(),
                                        1,
                                        2,
                                        losers_brackets_rounds_sets_positions
                                            .par_iter()
                                            .position_any(|&p| p == i)
                                            .unwrap()
                                            as u64
                                            + 1,
                                        1
                                    )))
                                    .unwrap()
                                    .game
                                    .to_string(),
                            "L".to_owned()
                                + &sets_graph
                                    .node_weight(parent_game!((
                                        sets_graph.clone(),
                                        1,
                                        2,
                                        losers_brackets_rounds_sets_positions
                                            .par_iter()
                                            .position_any(|&p| p == i)
                                            .unwrap()
                                            as u64
                                            + 1,
                                        2
                                    )))
                                    .unwrap()
                                    .game
                                    .to_string(),
                        ),
                    });
                    sets_graph.add_edge(
                        parent_game!((
                            sets_graph.clone(),
                            1,
                            2,
                            losers_brackets_rounds_sets_positions
                                .par_iter()
                                .position_any(|&p| p == i)
                                .unwrap() as u64
                                + 1,
                            1
                        )),
                        NodeIndex::from(sets_graph.node_count() as u32 - 1),
                        "loser",
                    );
                    sets_graph.add_edge(
                        parent_game!((
                            sets_graph.clone(),
                            1,
                            2,
                            losers_brackets_rounds_sets_positions
                                .par_iter()
                                .position_any(|&p| p == i)
                                .unwrap() as u64
                                + 1,
                            2
                        )),
                        NodeIndex::from(sets_graph.node_count() as u32 - 1),
                        "loser",
                    );
                } else {
                    game_number -= 1;
                    sets_graph.add_node(GraphSet {
                        bracket: 2,
                        game: 0,
                        round: 1,
                        position: i,
                        placeholders: (0.to_string(), 0.to_string()),
                    });
                }
            } else {
                game_number -= 1;
                sets_graph.add_node(GraphSet {
                    bracket: 2,
                    game: 0,
                    round: 1,
                    position: i,
                    placeholders: (0.to_string(), 0.to_string()),
                });
            }
        }
    }

    let mut game_number_offset: Vec<u64> = Vec::new();
    let mut i_offset: u64 = 0;
    for j in 1..first_bracket_exponent + 1 {
        game_number += game_number_offset.len() as u64;
        if j != first_bracket_exponent {
            game_number_offset = set_scan(sets_graph.clone(), 1, j);
        } else {
            game_number_offset = vec![1];
            i_offset = 1;
        }
        for i in 0..(full_first_bracket_first_round_sets_count >> j) + i_offset {
            sets_graph.add_node(GraphSet {
                bracket: 1,
                game: game_number + game_number_offset[i as usize],
                round: j + 1,
                position: i + 1,
                placeholders: if j == first_bracket_exponent {
                    (
                        "W".to_owned()
                            + &sets_graph
                                .clone()
                                .node_weights_mut()
                                .find(|weight| weight.bracket == 1 && weight.round == j)
                                .unwrap()
                                .game
                                .to_string(),
                        "W".to_owned()
                            + &sets_graph
                                .clone()
                                .node_weights_mut()
                                .find(|weight| {
                                    weight.bracket == 2
                                        && weight.round == 2 * j - 2 - is_small_second_bracket
                                })
                                .unwrap()
                                .game
                                .to_string(),
                    )
                } else {
                    (
                        if sets_graph
                            .node_weight(parent_game!((sets_graph.clone(), 1, j + 1, i + 1, 1)))
                            .unwrap()
                            .game
                            == 0
                        {
                            players
                                [triangle(first_bracket_exponent - j + 1, 2 * i + 1) as usize - 1]
                                .to_owned()
                        } else {
                            "W".to_owned()
                                + &sets_graph
                                    .node_weight(parent_game!((
                                        sets_graph.clone(),
                                        1,
                                        j + 1,
                                        i + 1,
                                        1
                                    )))
                                    .unwrap()
                                    .game
                                    .to_string()
                        },
                        if sets_graph
                            .node_weight(parent_game!((sets_graph.clone(), 1, j + 1, i + 1, 2)))
                            .unwrap()
                            .game
                            == 0
                        {
                            players
                                [triangle(first_bracket_exponent - j + 1, 2 * i + 2) as usize - 1]
                                .to_owned()
                        } else {
                            "W".to_owned()
                                + &sets_graph
                                    .node_weight(parent_game!((
                                        sets_graph.clone(),
                                        1,
                                        j + 1,
                                        i + 1,
                                        2
                                    )))
                                    .unwrap()
                                    .game
                                    .to_string()
                        },
                    )
                },
            });
            if j == first_bracket_exponent {
                sets_graph.add_edge(
                    sets_graph
                        .node_indices()
                        .find(|index| {
                            sets_graph[*index].bracket == 1 && sets_graph[*index].round == j
                        })
                        .unwrap(),
                    NodeIndex::from(sets_graph.node_count() as u32 - 1),
                    "winner",
                );
                sets_graph.add_edge(
                    sets_graph
                        .node_indices()
                        .find(|index| {
                            sets_graph[*index].bracket == 2
                                && sets_graph[*index].round == 2 * j - 2 - is_small_second_bracket
                        })
                        .unwrap(),
                    NodeIndex::from(sets_graph.node_count() as u32 - 1),
                    "winner",
                );
            } else {
                sets_graph.add_edge(
                    parent_game!((sets_graph.clone(), 1, j + 1, i + 1, 1)),
                    NodeIndex::from(sets_graph.node_count() as u32 - 1),
                    "winner",
                );
                sets_graph.add_edge(
                    parent_game!((sets_graph.clone(), 1, j + 1, i + 1, 2)),
                    NodeIndex::from(sets_graph.node_count() as u32 - 1),
                    "winner",
                );
            }
        }
        if j != first_bracket_exponent {
            game_number += game_number_offset.len() as u64;
            game_number_offset = set_scan(sets_graph.clone(), 1, j);
            let mut position_offset: Vec<u64> = Vec::from_iter(
                1..=semifull_second_bracket_first_round_sets_count >> (j - is_small_second_bracket),
            );
            let mut removed_games: Vec<usize> = Vec::new();
            if j == 2 || j == 3 {
                let (a, b) = game_number_offset.split_at(game_number_offset.len() / 2);
                game_number_offset = [b, a].concat();
                let (a, b) = position_offset.split_at(position_offset.len() / 2);
                position_offset = [b, a].concat();
            }
            if j == 2 || (j % 2 == 1 && j != 3) {
                game_number_offset.reverse();
                position_offset.reverse();
            }
            if j == 1 && is_small_second_bracket == 1 {
                for i in (0..semifull_second_bracket_first_round_sets_count).rev() {
                    if sets_graph
                        .node_weight(parent_game!((sets_graph.clone(), 1, j + 1, i + 1, 2)))
                        .unwrap()
                        .game
                        == 0
                    {
                        game_number_offset.remove(i as usize);
                        removed_games.push(i as usize);
                    }
                }
                removed_games.reverse();
                let mut largest_games: Vec<(u64, u64)> = Vec::new();
                let mut i: u64 = 0;
                game_number_offset.iter().for_each(|&offset| {
                    i += 1;
                    largest_games.push((offset, i));
                });
                i = 0;
                largest_games.sort_by(|a, b| a.0.cmp(&b.0));
                largest_games.iter_mut().for_each(|f| {
                    i += 1;
                    f.0 = i;
                });
                largest_games.sort_by(|a, b| a.1.cmp(&b.1));
                game_number_offset = largest_games.iter().map(|f| f.0).collect::<Vec<u64>>();
                removed_games
                    .iter()
                    .for_each(|&game| game_number_offset.insert(game, 0));
            }
            println!("{:?}, {:?}", game_number_offset, position_offset);
            for i in
                0..(semifull_second_bracket_first_round_sets_count >> (j - is_small_second_bracket))
            {
                if (j == 1 && game_number_offset[i as usize] != 0) || j != 1 {
                    sets_graph.add_node(GraphSet {
                        bracket: 2,
                        game: game_number + game_number_offset[i as usize],
                        round: 2 * j - is_small_second_bracket,
                        position: losers_brackets_rounds_sets_positions
                            [losers_brackets_rounds_sets_positions_vector_offset
                                [j as usize * 2 - 1 - is_small_second_bracket as usize]
                                + i as usize],
                        placeholders: (
                            if j < first_bracket_exponent - 1 {
                                if j == 1 && is_small_second_bracket == 1 {
                                    "L".to_owned()
                                        + &sets_graph
                                            .node_weight(parent_game!((
                                                sets_graph.clone(),
                                                1,
                                                j + 2,
                                                (((semifull_second_bracket_first_round_sets_count
                                                    >> (j - is_small_second_bracket))
                                                    as f64
                                                    - i as f64)
                                                    / 2.0)
                                                    .ceil()
                                                    as u64,
                                                2 - i % 2
                                            )))
                                            .unwrap()
                                            .game
                                            .to_string()
                                } else {
                                    "L".to_owned()
                                        + &sets_graph
                                            .node_weight(
                                                sets_graph
                                                    .node_indices()
                                                    .find(|&index| {
                                                        sets_graph[index].bracket == 1
                                                            && sets_graph[index].round == j + 1
                                                            && sets_graph[index].position
                                                                == position_offset[i as usize]
                                                    })
                                                    .unwrap(),
                                            )
                                            .unwrap()
                                            .game
                                            .to_string()
                                }
                            } else {
                                "L".to_owned()
                                    + &sets_graph
                                        .clone()
                                        .node_weights_mut()
                                        .find(|weight| weight.bracket == 1 && weight.round == j + 1)
                                        .unwrap()
                                        .game
                                        .to_string()
                            },
                            if ((is_small_second_bracket == 1 && j > 1)
                                || (is_small_second_bracket == 0))
                                && sets_graph
                                    .node_weight(parent_game!((
                                        sets_graph.clone(),
                                        2,
                                        2 * j - is_small_second_bracket,
                                        losers_brackets_rounds_sets_positions
                                            [losers_brackets_rounds_sets_positions_vector_offset[j
                                                as usize
                                                * 2
                                                - 1
                                                - is_small_second_bracket as usize]
                                                + i as usize],
                                        2
                                    )))
                                    .unwrap()
                                    .game
                                    != 0
                            {
                                "W".to_owned()
                                    + &sets_graph
                                        .node_weight(parent_game!((
                                            sets_graph.clone(),
                                            2,
                                            2 * j - is_small_second_bracket,
                                            losers_brackets_rounds_sets_positions
                                                [losers_brackets_rounds_sets_positions_vector_offset
                                                    [j as usize * 2
                                                        - 1
                                                        - is_small_second_bracket as usize]
                                                    + i as usize],
                                            2
                                        )))
                                        .unwrap()
                                        .game
                                        .to_string()
                            } else {
                                "L".to_owned()
                                    + &sets_graph
                                        .node_weight(parent_game!((
                                            sets_graph.clone(),
                                            1,
                                            j + 1,
                                            i + 1,
                                            2
                                        )))
                                        .unwrap()
                                        .game
                                        .to_string()
                            },
                        ),
                    });
                    if j < first_bracket_exponent - 1 {
                        if j == 1 && is_small_second_bracket == 1 {
                            sets_graph.add_edge(
                                parent_game!((
                                    sets_graph.clone(),
                                    1,
                                    j + 2,
                                    (((semifull_second_bracket_first_round_sets_count
                                        >> (j - is_small_second_bracket))
                                        as f64
                                        - i as f64)
                                        / 2.0)
                                        .ceil() as u64,
                                    2 - i % 2
                                )),
                                NodeIndex::from(sets_graph.node_count() as u32 - 1),
                                "loser",
                            );
                        } else {
                            sets_graph.add_edge(
                                sets_graph
                                    .node_indices()
                                    .find(|&index| {
                                        sets_graph[index].bracket == 1
                                            && sets_graph[index].round == j + 1
                                            && sets_graph[index].position
                                                == position_offset[i as usize]
                                    })
                                    .unwrap(),
                                NodeIndex::from(sets_graph.node_count() as u32 - 1),
                                "loser",
                            );
                        }
                    } else {
                        sets_graph.add_edge(
                            sets_graph
                                .node_indices()
                                .find(|&index| {
                                    sets_graph[index].bracket == 1
                                        && sets_graph[index].round == j + 1
                                })
                                .unwrap(),
                            NodeIndex::from(sets_graph.node_count() as u32 - 1),
                            "loser",
                        );
                    };
                    if ((is_small_second_bracket == 1 && j > 1) || (is_small_second_bracket == 0))
                        && sets_graph
                            .node_weight(parent_game!((
                                sets_graph.clone(),
                                2,
                                2 * j - is_small_second_bracket,
                                losers_brackets_rounds_sets_positions
                                    [losers_brackets_rounds_sets_positions_vector_offset
                                        [j as usize * 2 - 1 - is_small_second_bracket as usize]
                                        + i as usize],
                                2
                            )))
                            .unwrap()
                            .game
                            != 0
                    {
                        sets_graph.add_edge(
                            parent_game!((
                                sets_graph.clone(),
                                2,
                                2 * j - is_small_second_bracket,
                                losers_brackets_rounds_sets_positions
                                    [losers_brackets_rounds_sets_positions_vector_offset
                                        [j as usize * 2 - 1 - is_small_second_bracket as usize]
                                        + i as usize],
                                2
                            )),
                            NodeIndex::from(sets_graph.node_count() as u32 - 1),
                            "winner",
                        );
                    } else {
                        sets_graph.add_edge(
                            parent_game!((sets_graph.clone(), 1, j + 1, i + 1, 2)),
                            NodeIndex::from(sets_graph.node_count() as u32 - 1),
                            "loser",
                        );
                    }
                } else {
                    sets_graph.add_node(GraphSet {
                        bracket: 2,
                        game: 0,
                        round: 2 * j - is_small_second_bracket,
                        position: losers_brackets_rounds_sets_positions
                            [losers_brackets_rounds_sets_positions_vector_offset
                                [j as usize * 2 - 1 - is_small_second_bracket as usize]
                                + i as usize],
                        placeholders: (0.to_string(), 0.to_string()),
                    });
                }
            }
            game_number -= removed_games.len() as u64;
        }
        if j < first_bracket_exponent - 1 {
            game_number += game_number_offset.len() as u64;
            game_number_offset = set_scan(sets_graph.clone(), 2, 2 * j - is_small_second_bracket);

            let mut removed_games: Vec<usize> = Vec::new();
            let mut new_games: Vec<u64>;

            if j == 1 && is_small_second_bracket == 1 {
                new_games = set_scan(sets_graph.clone(), 1, j + 1);
                new_games.reverse();
                for i in (0..semifull_second_bracket_first_round_sets_count
                    >> (j + 1 - is_small_second_bracket))
                    .rev()
                {
                    if sets_graph
                        .node_weight(parent_game!((
                            sets_graph.clone(),
                            2,
                            2 * j + 1 - is_small_second_bracket,
                            losers_brackets_rounds_sets_positions
                                [losers_brackets_rounds_sets_positions_vector_offset
                                    [j as usize * 2 - is_small_second_bracket as usize]
                                    + i as usize],
                            1
                        )))
                        .unwrap()
                        .game
                        == 0
                    {
                        game_number_offset.remove(i as usize);
                        removed_games.push(i as usize);
                    } else {
                        new_games.remove(i as usize);
                    }
                }
                removed_games.reverse();
                let mut largest_games: Vec<(u64, u64)> = Vec::new();
                let mut i: u64 = 0;
                new_games.iter().for_each(|&game| {
                    i += 1;
                    largest_games.push((game, i));
                });
                i = 0;
                largest_games.sort_by(|a, b| a.0.cmp(&b.0));
                largest_games.iter_mut().for_each(|f| {
                    i += 1;
                    f.0 = i;
                });
                largest_games.sort_by(|a, b| a.1.cmp(&b.1));
                removed_games.iter().enumerate().for_each(|(index, &game)| {
                    game_number_offset.insert(game, largest_games[index].0)
                });
            }

            println!("{:?}", game_number_offset);
            for i in 0..(semifull_second_bracket_first_round_sets_count
                >> (j + 1 - is_small_second_bracket))
            {
                sets_graph.add_node(GraphSet {
                    bracket: 2,
                    game: game_number + game_number_offset[i as usize],
                    round: 2 * j + 1 - is_small_second_bracket,
                    position: losers_brackets_rounds_sets_positions
                        [losers_brackets_rounds_sets_positions_vector_offset
                            [j as usize * 2 - is_small_second_bracket as usize]
                            + i as usize],
                    placeholders: (
                        if sets_graph
                            .node_weight(parent_game!((
                                sets_graph.clone(),
                                2,
                                2 * j + 1 - is_small_second_bracket,
                                losers_brackets_rounds_sets_positions
                                    [losers_brackets_rounds_sets_positions_vector_offset
                                        [j as usize * 2 - is_small_second_bracket as usize]
                                        + i as usize],
                                1
                            )))
                            .unwrap()
                            .game
                            != 0
                        {
                            "W".to_owned()
                                + &sets_graph
                                    .node_weight(parent_game!((
                                        sets_graph.clone(),
                                        2,
                                        2 * j + 1 - is_small_second_bracket,
                                        losers_brackets_rounds_sets_positions
                                            [losers_brackets_rounds_sets_positions_vector_offset[j
                                                as usize
                                                * 2
                                                - is_small_second_bracket as usize]
                                                + i as usize],
                                        1
                                    )))
                                    .unwrap()
                                    .game
                                    .to_string()
                        } else {
                            "L".to_owned()
                                + &sets_graph
                                    .node_weight(parent_game!((
                                        sets_graph.clone(),
                                        1,
                                        j + 2,
                                        (semifull_second_bracket_first_round_sets_count
                                            >> (j + 1 - is_small_second_bracket))
                                            - i,
                                        2
                                    )))
                                    .unwrap()
                                    .game
                                    .to_string()
                        },
                        if sets_graph
                            .node_weight(parent_game!((
                                sets_graph.clone(),
                                2,
                                2 * j + 1 - is_small_second_bracket,
                                losers_brackets_rounds_sets_positions
                                    [losers_brackets_rounds_sets_positions_vector_offset
                                        [j as usize * 2 - is_small_second_bracket as usize]
                                        + i as usize],
                                2
                            )))
                            .unwrap()
                            .game
                            != 0
                        {
                            "W".to_owned()
                                + &sets_graph
                                    .node_weight(parent_game!((
                                        sets_graph.clone(),
                                        2,
                                        2 * j + 1 - is_small_second_bracket,
                                        losers_brackets_rounds_sets_positions
                                            [losers_brackets_rounds_sets_positions_vector_offset[j
                                                as usize
                                                * 2
                                                - is_small_second_bracket as usize]
                                                + i as usize],
                                        2
                                    )))
                                    .unwrap()
                                    .game
                                    .to_string()
                        } else {
                            "L".to_owned()
                                + &sets_graph
                                    .node_weight(parent_game!((
                                        sets_graph.clone(),
                                        1,
                                        j + 2,
                                        (semifull_second_bracket_first_round_sets_count
                                            >> (j + 1 - is_small_second_bracket))
                                            - i,
                                        1
                                    )))
                                    .unwrap()
                                    .game
                                    .to_string()
                        },
                    ),
                });
                if sets_graph
                    .node_weight(parent_game!((
                        sets_graph.clone(),
                        2,
                        2 * j + 1 - is_small_second_bracket,
                        losers_brackets_rounds_sets_positions
                            [losers_brackets_rounds_sets_positions_vector_offset
                                [j as usize * 2 - is_small_second_bracket as usize]
                                + i as usize],
                        1
                    )))
                    .unwrap()
                    .game
                    != 0
                {
                    sets_graph.add_edge(
                        parent_game!((
                            sets_graph.clone(),
                            2,
                            2 * j + 1 - is_small_second_bracket,
                            losers_brackets_rounds_sets_positions
                                [losers_brackets_rounds_sets_positions_vector_offset
                                    [j as usize * 2 - is_small_second_bracket as usize]
                                    + i as usize],
                            1
                        )),
                        NodeIndex::from(sets_graph.node_count() as u32 - 1),
                        "winner",
                    );
                } else {
                    sets_graph.add_edge(
                        parent_game!((
                            sets_graph.clone(),
                            1,
                            j + 2,
                            (semifull_second_bracket_first_round_sets_count
                                >> (j + 1 - is_small_second_bracket))
                                - i,
                            2
                        )),
                        NodeIndex::from(sets_graph.node_count() as u32 - 1),
                        "loser",
                    );
                }
                if sets_graph
                    .node_weight(parent_game!((
                        sets_graph.clone(),
                        2,
                        2 * j + 1 - is_small_second_bracket,
                        losers_brackets_rounds_sets_positions
                            [losers_brackets_rounds_sets_positions_vector_offset
                                [j as usize * 2 - is_small_second_bracket as usize]
                                + i as usize],
                        2
                    )))
                    .unwrap()
                    .game
                    != 0
                {
                    sets_graph.add_edge(
                        parent_game!((
                            sets_graph.clone(),
                            2,
                            2 * j + 1 - is_small_second_bracket,
                            losers_brackets_rounds_sets_positions
                                [losers_brackets_rounds_sets_positions_vector_offset
                                    [j as usize * 2 - is_small_second_bracket as usize]
                                    + i as usize],
                            2
                        )),
                        NodeIndex::from(sets_graph.node_count() as u32 - 1),
                        "winner",
                    );
                } else {
                    sets_graph.add_edge(
                        parent_game!((
                            sets_graph.clone(),
                            1,
                            j + 2,
                            (semifull_second_bracket_first_round_sets_count
                                >> (j + 1 - is_small_second_bracket))
                                - i,
                            1
                        )),
                        NodeIndex::from(sets_graph.node_count() as u32 - 1),
                        "loser",
                    );
                }
            }
        }
    }
    let mut reverse_indices = sets_graph.node_indices().collect::<Vec<NodeIndex>>();
    // println!("{:?}", reverse_indices);
    reverse_indices.par_sort_by(|a, b| b.cmp(&a));
    // println!("{:?}", reverse_indices);
    reverse_indices.iter().for_each(|index| {
        if sets_graph[*index].game == 0 {
            // println!("{:?}", index);
            sets_graph.remove_node(*index);
        }
    });
    // let clean_graph = sets_graph.map(|_, n| (n.bracket, n.game), |_, e| e);
    println!("{:?}", Dot::new(&sets_graph));
    sets_graph
}
