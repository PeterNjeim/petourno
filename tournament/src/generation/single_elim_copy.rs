use super::*;
use petgraph::dot::*;
use petgraph::stable_graph::*;
use petgraph::visit::EdgeRef;
use petgraph::EdgeDirection::Incoming;
// use rayon::prelude::*;
use std::iter::FromIterator;
use std::{cmp, fmt};

#[derive(Clone, Debug, PartialEq)]
pub struct GraphSet {
    pub bracket: u64,
    pub game: u64,
    pub round: u64,
    pub position: u64,
    pub placeholders: (String, String),
}

#[derive(Clone, Debug)]
pub struct SetEdge {
    pub outcome: String,
    pub position: u64,
}

impl fmt::Display for GraphSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

macro_rules! parent_game {
    ( $( $x:expr ),* ) => {
        {

            $(
                $x.0.node_indices().find(|index| {$x.0[*index].bracket == $x.1 && $x.0[*index].round == $x.2 - 1 && $x.0[*index].position == $x.3 * 2 - $x.4 % 2}).unwrap()
            )*
        }
    };
}

macro_rules! child_game {
    ( $( $x:expr ),* ) => {
        {

            $(
                $x.0.node_indices().find(|index| {$x.0[*index].bracket == $x.1 && $x.0[*index].round == $x.2 + 1 && $x.0[*index].position == ($x.3 + ($x.3 % 2)) / 2}).unwrap()
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

fn new_set_scan(mut graph: StableGraph<GraphSet, SetEdge>, bracket: u64, round: u64) -> Vec<u64> {
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

fn small_set_scan(mut graph: StableGraph<GraphSet, SetEdge>, bracket: u64, round: u64) -> Vec<u64> {
    let mut round_sets = graph
        .node_weights_mut()
        .map(|set| set.to_owned())
        .filter(|set| set.bracket == bracket && set.round == round)
        .collect::<Vec<GraphSet>>();
    // println!("{:?}", round_sets);
    round_sets.sort_by(|a, b| a.position.cmp(&b.position));
    // println!("{:?}", round_sets);
    let mut largest_games: Vec<(u64, u64)> = Vec::new();
    round_sets
        .iter()
        .for_each(|set| largest_games.push((set.game, set.position)));
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

/*
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
*/
/*
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
*/

pub fn new_single_elim(players: Vec<String>) -> StableGraph<GraphSet, &'static str> {
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
    reverse_indices.sort_by(|a, b| b.cmp(&a));
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

pub fn new_double_elim(players: Vec<String>) -> StableGraph<GraphSet, &'static str> {
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
                            .iter()
                            .position(|&p| p == i)
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
                                .iter()
                                .position(|&p| p == i)
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
                                            .iter()
                                            .position(|&p| p == i)
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
                                            .iter()
                                            .position(|&p| p == i)
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
                                .iter()
                                .position(|&p| p == i)
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
                                .iter()
                                .position(|&p| p == i)
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
    reverse_indices.sort_by(|a, b| b.cmp(&a));
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

pub fn elimination(players: Vec<String>, tuple: u64) -> StableGraph<GraphSet, SetEdge> {
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
        u32::pow(2, ((players_count as f64 - 1.0).log2()).floor() as u32) / 2 + 1,
    );
    let semifull_second_bracket_first_round_sets_count =
        u64::pow(2, ((players_count as f64 - 1.0).log2()).floor() as u32);
    let second_bracket_exponent =
        ((full_second_bracket_first_round_sets_count) as f64).log2() as u64;
    let mut losers_brackets_rounds_sets_positions: Vec<u64> =
        Vec::with_capacity((semifull_second_bracket_first_round_sets_count / 2) as usize);
    let mut losers_brackets_rounds_sets_positions_vector_offset: Vec<usize> =
        Vec::with_capacity((first_bracket_exponent + 3) as usize);
    losers_brackets_rounds_sets_positions_vector_offset.push(0);
    for j in 0..(2 * first_bracket_exponent - 2) {
        for i in 1..=semifull_second_bracket_first_round_sets_count
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

    let mut sets_graph = StableGraph::<GraphSet, SetEdge>::with_capacity(
        full_first_bracket_sets_count,
        full_first_bracket_sets_count - 1,
    );
    let mut game_number: u64 = 0;
    for i in 1..=full_first_bracket_first_round_sets_count as u64 {
        game_number += 1;
        sets_graph.add_node(GraphSet {
            bracket: 1,
            game: game_number,
            round: 1,
            position: i,
            placeholders: (
                players[triangle(first_bracket_exponent + 1, 2 * i - 1) as usize - 1].to_owned(),
                if triangle(first_bracket_exponent + 1, 2 * i) <= players_count as u64 {
                    players[triangle(first_bracket_exponent + 1, 2 * i) as usize - 1].to_owned()
                } else {
                    "0".to_owned()
                },
            ),
        });
    }

    losers_brackets_rounds_sets_positions
        [0..(semifull_second_bracket_first_round_sets_count / 2) as usize]
        .iter()
        .for_each(|&i| {
            game_number += 1;
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
                                    .iter()
                                    .position(|&p| p == i)
                                    .unwrap() as u64
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
                                    .iter()
                                    .position(|&p| p == i)
                                    .unwrap() as u64
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
                        .iter()
                        .position(|&p| p == i)
                        .unwrap() as u64
                        + 1,
                    1
                )),
                NodeIndex::from(sets_graph.node_count() as u32 - 1),
                SetEdge {
                    outcome: "loser".to_owned(),
                    position: 1,
                },
            );
            sets_graph.add_edge(
                parent_game!((
                    sets_graph.clone(),
                    1,
                    2,
                    losers_brackets_rounds_sets_positions
                        .iter()
                        .position(|&p| p == i)
                        .unwrap() as u64
                        + 1,
                    2
                )),
                NodeIndex::from(sets_graph.node_count() as u32 - 1),
                SetEdge {
                    outcome: "loser".to_owned(),
                    position: 2,
                },
            );
        });

    let mut game_number_offset: Vec<u64> = Vec::new();
    let mut i_offset: u64 = 0;
    for j in 1..first_bracket_exponent + 1 {
        game_number += game_number_offset.len() as u64;
        if j != first_bracket_exponent {
            game_number_offset = new_set_scan(sets_graph.clone(), 1, j);
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
                                .find(|weight| weight.bracket == 2 && weight.round == 2 * j - 2)
                                .unwrap()
                                .game
                                .to_string(),
                    )
                } else {
                    (
                        "W".to_owned()
                            + &sets_graph
                                .node_weight(parent_game!((sets_graph.clone(), 1, j + 1, i + 1, 1)))
                                .unwrap()
                                .game
                                .to_string(),
                        "W".to_owned()
                            + &sets_graph
                                .node_weight(parent_game!((sets_graph.clone(), 1, j + 1, i + 1, 2)))
                                .unwrap()
                                .game
                                .to_string(),
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
                    SetEdge {
                        outcome: "winner".to_owned(),
                        position: 1,
                    },
                );
                sets_graph.add_edge(
                    sets_graph
                        .node_indices()
                        .find(|index| {
                            sets_graph[*index].bracket == 2 && sets_graph[*index].round == 2 * j - 2
                        })
                        .unwrap(),
                    NodeIndex::from(sets_graph.node_count() as u32 - 1),
                    SetEdge {
                        outcome: "winner".to_owned(),
                        position: 2,
                    },
                );
            } else {
                sets_graph.add_edge(
                    parent_game!((sets_graph.clone(), 1, j + 1, i + 1, 1)),
                    NodeIndex::from(sets_graph.node_count() as u32 - 1),
                    SetEdge {
                        outcome: "winner".to_owned(),
                        position: 1,
                    },
                );
                sets_graph.add_edge(
                    parent_game!((sets_graph.clone(), 1, j + 1, i + 1, 2)),
                    NodeIndex::from(sets_graph.node_count() as u32 - 1),
                    SetEdge {
                        outcome: "winner".to_owned(),
                        position: 2,
                    },
                );
            }
        }
        if j != first_bracket_exponent {
            game_number += game_number_offset.len() as u64;
            game_number_offset = new_set_scan(sets_graph.clone(), 1, j);
            let mut position_offset: Vec<u64> =
                Vec::from_iter(1..=semifull_second_bracket_first_round_sets_count >> j);
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
            println!("{:?}, {:?}", game_number_offset, position_offset);
            for i in 0..(semifull_second_bracket_first_round_sets_count >> j) {
                sets_graph.add_node(GraphSet {
                    bracket: 2,
                    game: game_number + game_number_offset[i as usize],
                    round: 2 * j,
                    position: losers_brackets_rounds_sets_positions
                        [losers_brackets_rounds_sets_positions_vector_offset[j as usize * 2 - 1]
                            + i as usize],
                    placeholders: (
                        if j < first_bracket_exponent - 1 {
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
                        "W".to_owned()
                            + &sets_graph
                                .node_weight(parent_game!((
                                    sets_graph.clone(),
                                    2,
                                    2 * j,
                                    losers_brackets_rounds_sets_positions
                                        [losers_brackets_rounds_sets_positions_vector_offset
                                            [j as usize * 2 - 1]
                                            + i as usize],
                                    2
                                )))
                                .unwrap()
                                .game
                                .to_string(),
                    ),
                });
                if j < first_bracket_exponent - 1 {
                    sets_graph.add_edge(
                        sets_graph
                            .node_indices()
                            .find(|&index| {
                                sets_graph[index].bracket == 1
                                    && sets_graph[index].round == j + 1
                                    && sets_graph[index].position == position_offset[i as usize]
                            })
                            .unwrap(),
                        NodeIndex::from(sets_graph.node_count() as u32 - 1),
                        SetEdge {
                            outcome: "loser".to_owned(),
                            position: 1,
                        },
                    );
                } else {
                    sets_graph.add_edge(
                        sets_graph
                            .node_indices()
                            .find(|&index| {
                                sets_graph[index].bracket == 1 && sets_graph[index].round == j + 1
                            })
                            .unwrap(),
                        NodeIndex::from(sets_graph.node_count() as u32 - 1),
                        SetEdge {
                            outcome: "loser".to_owned(),
                            position: 1,
                        },
                    );
                };
                sets_graph.add_edge(
                    parent_game!((
                        sets_graph.clone(),
                        2,
                        2 * j,
                        losers_brackets_rounds_sets_positions
                            [losers_brackets_rounds_sets_positions_vector_offset
                                [j as usize * 2 - 1]
                                + i as usize],
                        2
                    )),
                    NodeIndex::from(sets_graph.node_count() as u32 - 1),
                    SetEdge {
                        outcome: "winner".to_owned(),
                        position: 2,
                    },
                );
            }
        }
        if j < first_bracket_exponent - 1 {
            game_number += game_number_offset.len() as u64;
            game_number_offset = new_set_scan(sets_graph.clone(), 2, 2 * j);

            println!("{:?}", game_number_offset);
            for i in 0..(semifull_second_bracket_first_round_sets_count >> (j + 1)) {
                sets_graph.add_node(GraphSet {
                    bracket: 2,
                    game: game_number + game_number_offset[i as usize],
                    round: 2 * j + 1,
                    position: losers_brackets_rounds_sets_positions
                        [losers_brackets_rounds_sets_positions_vector_offset[j as usize * 2]
                            + i as usize],
                    placeholders: (
                        "W".to_owned()
                            + &sets_graph
                                .node_weight(parent_game!((
                                    sets_graph.clone(),
                                    2,
                                    2 * j + 1,
                                    losers_brackets_rounds_sets_positions
                                        [losers_brackets_rounds_sets_positions_vector_offset
                                            [j as usize * 2]
                                            + i as usize],
                                    1
                                )))
                                .unwrap()
                                .game
                                .to_string(),
                        "W".to_owned()
                            + &sets_graph
                                .node_weight(parent_game!((
                                    sets_graph.clone(),
                                    2,
                                    2 * j + 1,
                                    losers_brackets_rounds_sets_positions
                                        [losers_brackets_rounds_sets_positions_vector_offset
                                            [j as usize * 2]
                                            + i as usize],
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
                        2,
                        2 * j + 1,
                        losers_brackets_rounds_sets_positions
                            [losers_brackets_rounds_sets_positions_vector_offset[j as usize * 2]
                                + i as usize],
                        1
                    )),
                    NodeIndex::from(sets_graph.node_count() as u32 - 1),
                    SetEdge {
                        outcome: "winner".to_owned(),
                        position: 1,
                    },
                );

                sets_graph.add_edge(
                    parent_game!((
                        sets_graph.clone(),
                        2,
                        2 * j + 1,
                        losers_brackets_rounds_sets_positions
                            [losers_brackets_rounds_sets_positions_vector_offset[j as usize * 2]
                                + i as usize],
                        2
                    )),
                    NodeIndex::from(sets_graph.node_count() as u32 - 1),
                    SetEdge {
                        outcome: "winner".to_owned(),
                        position: 2,
                    },
                );
            }
        }
    }

    sets_graph
        .clone()
        .node_weights_mut()
        .map(|set| set.to_owned())
        .filter(|n| n.bracket == 1 && n.round == 1 && n.placeholders.1 == "0")
        .for_each(|n| {
            let node_index = sets_graph
                .node_indices()
                .find(|&i| {
                    sets_graph[i].bracket == n.bracket
                        && sets_graph[i].round == n.round
                        && sets_graph[i].position == n.position
                })
                .unwrap();
            let temp_sets = sets_graph.clone();

            let later_games = sets_graph
                .node_indices()
                .filter(|&g| sets_graph[g].game > sets_graph[node_index].game)
                .collect::<Vec<NodeIndex>>();
            later_games.iter().for_each(|&g| sets_graph[g].game -= 1);

            sets_graph[node_index].game = 0;

            if EdgeRef::weight(
                &temp_sets
                    .edges(node_index)
                    .find(|e| e.weight().outcome == "winner")
                    .unwrap(),
            )
            .position
                == 1
            {
                sets_graph[EdgeRef::target(
                    &temp_sets
                        .edges(node_index)
                        .find(|e| e.weight().outcome == "winner")
                        .unwrap(),
                )]
                .placeholders
                .0 = players[triangle(first_bracket_exponent, n.position) as usize - 1].to_owned();
            } else {
                sets_graph[EdgeRef::target(
                    &temp_sets
                        .edges(node_index)
                        .find(|e| e.weight().outcome == "winner")
                        .unwrap(),
                )]
                .placeholders
                .1 = players[triangle(first_bracket_exponent, n.position) as usize - 1].to_owned();
            }

            if EdgeRef::weight(
                &temp_sets
                    .edges(node_index)
                    .find(|e| e.weight().outcome == "loser")
                    .unwrap(),
            )
            .position
                == 1
            {
                sets_graph[EdgeRef::target(
                    &temp_sets
                        .edges(node_index)
                        .find(|e| e.weight().outcome == "loser")
                        .unwrap(),
                )]
                .placeholders
                .0 = "0".to_owned();
            } else {
                sets_graph[EdgeRef::target(
                    &temp_sets
                        .edges(node_index)
                        .find(|e| e.weight().outcome == "loser")
                        .unwrap(),
                )]
                .placeholders
                .1 = "0".to_owned();
            }
        });

    sets_graph
        .clone()
        .node_weights_mut()
        .map(|set| set.to_owned())
        .filter(|n| {
            n.bracket == 2
                && n.round == 1
                && n.game != 0
                && (n.placeholders.0 == "0" || n.placeholders.1 == "0")
        })
        .for_each(|n| {
            let node_index = sets_graph
                .node_indices()
                .find(|&i| {
                    sets_graph[i].bracket == n.bracket
                        && sets_graph[i].round == n.round
                        && sets_graph[i].position == n.position
                })
                .unwrap();
            let temp_sets = sets_graph.clone();

            let later_games = sets_graph
                .node_indices()
                .filter(|&g| sets_graph[g].game > sets_graph[node_index].game)
                .collect::<Vec<NodeIndex>>();
            later_games.iter().for_each(|&g| sets_graph[g].game -= 1);

            sets_graph[node_index].game = 0;

            if n.placeholders.0 == "0" && n.placeholders.1 == "0" {
                if EdgeRef::weight(
                    &temp_sets
                        .edges(node_index)
                        .find(|e| e.weight().outcome == "winner")
                        .unwrap(),
                )
                .position
                    == 1
                {
                    sets_graph[EdgeRef::target(
                        &temp_sets
                            .edges(node_index)
                            .find(|e| e.weight().outcome == "winner")
                            .unwrap(),
                    )]
                    .placeholders
                    .0 = "0".to_owned();
                } else {
                    sets_graph[EdgeRef::target(
                        &temp_sets
                            .edges(node_index)
                            .find(|e| e.weight().outcome == "winner")
                            .unwrap(),
                    )]
                    .placeholders
                    .1 = "0".to_owned();
                }
            } else {
                if EdgeRef::weight(
                    &temp_sets
                        .edges(node_index)
                        .find(|e| e.weight().outcome == "winner")
                        .unwrap(),
                )
                .position
                    == 1
                {
                    sets_graph[EdgeRef::target(
                        &temp_sets
                            .edges(node_index)
                            .find(|e| e.weight().outcome == "winner")
                            .unwrap(),
                    )]
                    .placeholders
                    .0 = if n.placeholders.0 != "0" {
                        n.placeholders.0
                    } else {
                        n.placeholders.1
                    };
                } else {
                    sets_graph[EdgeRef::target(
                        &temp_sets
                            .edges(node_index)
                            .find(|e| e.weight().outcome == "winner")
                            .unwrap(),
                    )]
                    .placeholders
                    .1 = if n.placeholders.0 != "0" {
                        n.placeholders.0
                    } else {
                        n.placeholders.1
                    };
                }
            }
        });

    sets_graph
        .clone()
        .node_weights_mut()
        .map(|set| set.to_owned())
        .filter(|n| {
            n.bracket == 2
                && n.round == 2
                && n.game != 0
                && (n.placeholders.0 == "0" || n.placeholders.1 == "0")
        })
        .for_each(|n| {
            let node_index = sets_graph
                .node_indices()
                .find(|&i| {
                    sets_graph[i].bracket == n.bracket
                        && sets_graph[i].round == n.round
                        && sets_graph[i].position == n.position
                })
                .unwrap();
            let temp_sets = sets_graph.clone();

            let later_games = sets_graph
                .node_indices()
                .filter(|&g| sets_graph[g].game > sets_graph[node_index].game)
                .collect::<Vec<NodeIndex>>();
            later_games.iter().for_each(|&g| sets_graph[g].game -= 1);

            sets_graph[node_index].game = 0;

            if n.placeholders.0 == "0" && n.placeholders.1 == "0" {
                if EdgeRef::weight(
                    &temp_sets
                        .edges(node_index)
                        .find(|e| e.weight().outcome == "winner")
                        .unwrap(),
                )
                .position
                    == 1
                {
                    sets_graph[EdgeRef::target(
                        &temp_sets
                            .edges(node_index)
                            .find(|e| e.weight().outcome == "winner")
                            .unwrap(),
                    )]
                    .placeholders
                    .0 = "0".to_owned();
                } else {
                    sets_graph[EdgeRef::target(
                        &temp_sets
                            .edges(node_index)
                            .find(|e| e.weight().outcome == "winner")
                            .unwrap(),
                    )]
                    .placeholders
                    .1 = "0".to_owned();
                }
            } else {
                if EdgeRef::weight(
                    &temp_sets
                        .edges(node_index)
                        .find(|e| e.weight().outcome == "winner")
                        .unwrap(),
                )
                .position
                    == 1
                {
                    sets_graph[EdgeRef::target(
                        &temp_sets
                            .edges(node_index)
                            .find(|e| e.weight().outcome == "winner")
                            .unwrap(),
                    )]
                    .placeholders
                    .0 = if n.placeholders.0 != "0" {
                        n.placeholders.0
                    } else {
                        n.placeholders.1
                    };
                } else {
                    sets_graph[EdgeRef::target(
                        &temp_sets
                            .edges(node_index)
                            .find(|e| e.weight().outcome == "winner")
                            .unwrap(),
                    )]
                    .placeholders
                    .1 = if n.placeholders.0 != "0" {
                        n.placeholders.0
                    } else {
                        n.placeholders.1
                    };
                }
            }
        });

    let mut reverse_indices = sets_graph.node_indices().collect::<Vec<NodeIndex>>();
    // println!("{:?}", reverse_indices);
    reverse_indices.sort_by(|a, b| b.cmp(&a));
    // println!("{:?}", reverse_indices);
    reverse_indices.iter().for_each(|index| {
        if sets_graph[*index].game == 0 {
            // println!("{:?}", index);
            sets_graph.remove_node(*index);
        }
    });

    sets_graph
        .clone()
        .node_weights_mut()
        .map(|set| set.to_owned())
        .filter(|n| n.bracket == 1 && n.round == 1 && n.placeholders.1 == "0")
        .for_each(|n| {
            let node_index = sets_graph
                .node_indices()
                .find(|&i| {
                    sets_graph[i].bracket == n.bracket
                        && sets_graph[i].round == n.round
                        && sets_graph[i].position == n.position
                })
                .unwrap();
            let temp_sets = sets_graph.clone();

            let later_games = sets_graph
                .node_indices()
                .filter(|&g| sets_graph[g].game > sets_graph[node_index].game)
                .collect::<Vec<NodeIndex>>();
            later_games.iter().for_each(|&g| sets_graph[g].game -= 1);

            sets_graph[node_index].game = 0;

            if EdgeRef::weight(
                &temp_sets
                    .edges(node_index)
                    .find(|e| e.weight().outcome == "winner")
                    .unwrap(),
            )
            .position
                == 1
            {
                sets_graph[EdgeRef::target(
                    &temp_sets
                        .edges(node_index)
                        .find(|e| e.weight().outcome == "winner")
                        .unwrap(),
                )]
                .placeholders
                .0 = players[triangle(first_bracket_exponent, n.position) as usize - 1].to_owned();
            } else {
                sets_graph[EdgeRef::target(
                    &temp_sets
                        .edges(node_index)
                        .find(|e| e.weight().outcome == "winner")
                        .unwrap(),
                )]
                .placeholders
                .1 = players[triangle(first_bracket_exponent, n.position) as usize - 1].to_owned();
            }

            if EdgeRef::weight(
                &temp_sets
                    .edges(node_index)
                    .find(|e| e.weight().outcome == "loser")
                    .unwrap(),
            )
            .position
                == 1
            {
                sets_graph[EdgeRef::target(
                    &temp_sets
                        .edges(node_index)
                        .find(|e| e.weight().outcome == "loser")
                        .unwrap(),
                )]
                .placeholders
                .0 = "0".to_owned();
            } else {
                sets_graph[EdgeRef::target(
                    &temp_sets
                        .edges(node_index)
                        .find(|e| e.weight().outcome == "loser")
                        .unwrap(),
                )]
                .placeholders
                .1 = "0".to_owned();
            }
        });

    sets_graph
        .clone()
        .node_weights_mut()
        .map(|set| set.to_owned())
        .filter(|n| {
            n.bracket == 2
                && n.round == 1
                && n.game != 0
                && (n.placeholders.0 == "0" || n.placeholders.1 == "0")
        })
        .for_each(|n| {
            let node_index = sets_graph
                .node_indices()
                .find(|&i| {
                    sets_graph[i].bracket == n.bracket
                        && sets_graph[i].round == n.round
                        && sets_graph[i].position == n.position
                })
                .unwrap();
            let temp_sets = sets_graph.clone();

            let later_games = sets_graph
                .node_indices()
                .filter(|&g| sets_graph[g].game > sets_graph[node_index].game)
                .collect::<Vec<NodeIndex>>();
            later_games.iter().for_each(|&g| sets_graph[g].game -= 1);

            sets_graph[node_index].game = 0;

            if n.placeholders.0 == "0" && n.placeholders.1 == "0" {
                if EdgeRef::weight(
                    &temp_sets
                        .edges(node_index)
                        .find(|e| e.weight().outcome == "winner")
                        .unwrap(),
                )
                .position
                    == 1
                {
                    sets_graph[EdgeRef::target(
                        &temp_sets
                            .edges(node_index)
                            .find(|e| e.weight().outcome == "winner")
                            .unwrap(),
                    )]
                    .placeholders
                    .0 = "0".to_owned();
                } else {
                    sets_graph[EdgeRef::target(
                        &temp_sets
                            .edges(node_index)
                            .find(|e| e.weight().outcome == "winner")
                            .unwrap(),
                    )]
                    .placeholders
                    .1 = "0".to_owned();
                }
            } else {
                if EdgeRef::weight(
                    &temp_sets
                        .edges(node_index)
                        .find(|e| e.weight().outcome == "winner")
                        .unwrap(),
                )
                .position
                    == 1
                {
                    sets_graph[EdgeRef::target(
                        &temp_sets
                            .edges(node_index)
                            .find(|e| e.weight().outcome == "winner")
                            .unwrap(),
                    )]
                    .placeholders
                    .0 = if n.placeholders.0 != "0" {
                        n.placeholders.0
                    } else {
                        n.placeholders.1
                    };
                } else {
                    sets_graph[EdgeRef::target(
                        &temp_sets
                            .edges(node_index)
                            .find(|e| e.weight().outcome == "winner")
                            .unwrap(),
                    )]
                    .placeholders
                    .1 = if n.placeholders.0 != "0" {
                        n.placeholders.0
                    } else {
                        n.placeholders.1
                    };
                }
            }
        });

    sets_graph
        .clone()
        .node_weights_mut()
        .map(|set| set.to_owned())
        .filter(|n| {
            n.bracket == 2
                && n.round == 2
                && n.game != 0
                && (n.placeholders.0 == "0" || n.placeholders.1 == "0")
        })
        .for_each(|n| {
            let node_index = sets_graph
                .node_indices()
                .find(|&i| {
                    sets_graph[i].bracket == n.bracket
                        && sets_graph[i].round == n.round
                        && sets_graph[i].position == n.position
                })
                .unwrap();
            let temp_sets = sets_graph.clone();

            let later_games = sets_graph
                .node_indices()
                .filter(|&g| sets_graph[g].game > sets_graph[node_index].game)
                .collect::<Vec<NodeIndex>>();
            later_games.iter().for_each(|&g| sets_graph[g].game -= 1);

            sets_graph[node_index].game = 0;

            if n.placeholders.0 == "0" && n.placeholders.1 == "0" {
                if EdgeRef::weight(
                    &temp_sets
                        .edges(node_index)
                        .find(|e| e.weight().outcome == "winner")
                        .unwrap(),
                )
                .position
                    == 1
                {
                    sets_graph[EdgeRef::target(
                        &temp_sets
                            .edges(node_index)
                            .find(|e| e.weight().outcome == "winner")
                            .unwrap(),
                    )]
                    .placeholders
                    .0 = "0".to_owned();
                } else {
                    sets_graph[EdgeRef::target(
                        &temp_sets
                            .edges(node_index)
                            .find(|e| e.weight().outcome == "winner")
                            .unwrap(),
                    )]
                    .placeholders
                    .1 = "0".to_owned();
                }
            } else {
                if EdgeRef::weight(
                    &temp_sets
                        .edges(node_index)
                        .find(|e| e.weight().outcome == "winner")
                        .unwrap(),
                )
                .position
                    == 1
                {
                    sets_graph[EdgeRef::target(
                        &temp_sets
                            .edges(node_index)
                            .find(|e| e.weight().outcome == "winner")
                            .unwrap(),
                    )]
                    .placeholders
                    .0 = if n.placeholders.0 != "0" {
                        n.placeholders.0
                    } else {
                        n.placeholders.1
                    };
                } else {
                    sets_graph[EdgeRef::target(
                        &temp_sets
                            .edges(node_index)
                            .find(|e| e.weight().outcome == "winner")
                            .unwrap(),
                    )]
                    .placeholders
                    .1 = if n.placeholders.0 != "0" {
                        n.placeholders.0
                    } else {
                        n.placeholders.1
                    };
                }
            }
        });

    game_number = 3;
    game_number_offset = Vec::new();
    for j in 1..(first_bracket_exponent + 1) {
        game_number += game_number_offset.len() as u64;
        if j != first_bracket_exponent {
            game_number_offset = new_set_scan(sets_graph.clone(), 1, j);
            i_offset = 0;
        } else {
            game_number_offset = vec![1];
            i_offset = 1;
        }
        for i in 0..(full_first_bracket_first_round_sets_count >> j) + i_offset {
            println!("j+1:{}, i+1:{}", j + 1, i + 1);
            let target_set = sets_graph
                .node_weights_mut()
                .find(|set| set.bracket == 1 && set.round == j + 1 && set.position == i + 1)
                .unwrap();
            target_set.game = game_number + game_number_offset[i as usize];
        }
        if j != first_bracket_exponent {
            game_number += game_number_offset.len() as u64;
            game_number_offset = new_set_scan(sets_graph.clone(), 1, j);
            let mut position_offset: Vec<u64> =
                Vec::from_iter(1..=semifull_second_bracket_first_round_sets_count >> j);
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
            println!("{:?}, {:?}", game_number_offset, position_offset);
            for i in 0..(semifull_second_bracket_first_round_sets_count >> j) {
                let target_set = sets_graph
                    .node_weights_mut()
                    .find(|set| {
                        set.bracket == 2
                            && set.round == 2 * j
                            && set.position
                                == losers_brackets_rounds_sets_positions
                                    [losers_brackets_rounds_sets_positions_vector_offset
                                        [j as usize * 2 - 1]
                                        + i as usize]
                    })
                    .unwrap();
                target_set.game = game_number + game_number_offset[i as usize];
            }
        }
        if j < first_bracket_exponent - 1 {
            game_number += game_number_offset.len() as u64;
            game_number_offset = new_set_scan(sets_graph.clone(), 2, 2 * j);

            println!("{:?}", game_number_offset);
            for i in 0..(semifull_second_bracket_first_round_sets_count >> (j + 1)) {
                let target_set = sets_graph
                    .node_weights_mut()
                    .find(|set| {
                        set.bracket == 2
                            && set.round == 2 * j + 1
                            && set.position
                                == losers_brackets_rounds_sets_positions
                                    [losers_brackets_rounds_sets_positions_vector_offset
                                        [j as usize * 2]
                                        + i as usize]
                    })
                    .unwrap();
                target_set.game = game_number + game_number_offset[i as usize];
            }
        }
    }

    if sets_graph
        .node_weights_mut()
        .any(|n| n.bracket == 2 && n.round == 1)
        == false
    {
        sets_graph
            .clone()
            .node_weights_mut()
            .map(|set| set.to_owned())
            .filter(|n| n.bracket == 2 && n.game != 0)
            .for_each(|n| {
                let node_index = sets_graph
                    .node_indices()
                    .find(|&i| {
                        sets_graph[i].bracket == n.bracket
                            && sets_graph[i].round == n.round
                            && sets_graph[i].position == n.position
                    })
                    .unwrap();
                sets_graph[node_index].round -= 1;
            });
    }

    let mut reverse_indices = sets_graph.node_indices().collect::<Vec<NodeIndex>>();
    // println!("{:?}", reverse_indices);
    reverse_indices.sort_by(|a, b| b.cmp(&a));
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

// ! COMPLETE STEP 1.
pub fn new_elim(players: Vec<String>, tuple: u64) -> StableGraph<GraphSet, SetEdge> {
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
        u32::pow(2, ((players_count as f64 - 1.0).log2()).floor() as u32) / 2 + 1,
    );
    let semifull_second_bracket_first_round_sets_count =
        u64::pow(2, ((players_count as f64 - 1.0).log2()).floor() as u32);
    let second_bracket_exponent =
        ((full_second_bracket_first_round_sets_count) as f64).log2() as u64;
    let mut losers_brackets_rounds_sets_positions: Vec<u64> =
        Vec::with_capacity((semifull_second_bracket_first_round_sets_count / 2) as usize);
    let mut losers_brackets_rounds_sets_positions_vector_offset: Vec<usize> =
        Vec::with_capacity((first_bracket_exponent + 3) as usize);
    losers_brackets_rounds_sets_positions_vector_offset.push(0);
    for j in 0..(2 * first_bracket_exponent - 2) {
        for i in 1..=semifull_second_bracket_first_round_sets_count
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

    let mut sets_graph = StableGraph::<GraphSet, SetEdge>::with_capacity(
        full_first_bracket_sets_count,
        full_first_bracket_sets_count - 1,
    );
    // * COMPLETED STEP 1. (z)
    let mut game_number: u64 = 0;
    // * COMPLETED STEP 1. (a)
    for i in 1..=full_first_bracket_first_round_sets_count as u64 {
        game_number += 1;
        sets_graph.add_node(GraphSet {
            bracket: 1,
            game: if triangle(first_bracket_exponent + 1, 2 * i) <= players_count as u64 {
                game_number
            } else {
                0
            },
            round: 1,
            position: i,
            placeholders: (
                players[triangle(first_bracket_exponent + 1, 2 * i - 1) as usize - 1].to_owned(),
                if triangle(first_bracket_exponent + 1, 2 * i) <= players_count as u64 {
                    players[triangle(first_bracket_exponent + 1, 2 * i) as usize - 1].to_owned()
                } else {
                    "0".to_owned()
                },
            ),
        });
    }

    // * COMPLETED STEP 1. (b)
    for j in 2..=tuple {
        for i in 1..=full_first_bracket_first_round_sets_count as u64 {
            game_number += 1;
            sets_graph.add_node(GraphSet {
                bracket: j,
                game: if i % 2 == 0 { game_number } else { 0 },
                round: 1,
                position: i,
                placeholders: if i % 2 == 0 {
                    (
                        "L".to_owned()
                            + &sets_graph[sets_graph
                                .node_indices()
                                .find(|&n| {
                                    sets_graph[n].bracket == j - 1
                                        && sets_graph[n].round == 1
                                        && sets_graph[n].position == i - 1
                                })
                                .unwrap()]
                            .game
                            .to_string(),
                        "L".to_owned()
                            + &sets_graph[sets_graph
                                .node_indices()
                                .find(|&n| {
                                    sets_graph[n].bracket == j - 1
                                        && sets_graph[n].round == 1
                                        && sets_graph[n].position == i
                                })
                                .unwrap()]
                            .game
                            .to_string(),
                    )
                } else {
                    ("0".to_owned(), "0".to_owned())
                },
            });
            if i % 2 == 0 {
                sets_graph.add_edge(
                    sets_graph
                        .node_indices()
                        .find(|&n| {
                            sets_graph[n].bracket == j - 1
                                && sets_graph[n].round == 1
                                && sets_graph[n].position == i - 1
                        })
                        .unwrap(),
                    NodeIndex::from(sets_graph.node_count() as u32 - 1),
                    SetEdge {
                        outcome: "loser".to_owned(),
                        position: 1,
                    },
                );
                sets_graph.add_edge(
                    sets_graph
                        .node_indices()
                        .find(|&n| {
                            sets_graph[n].bracket == j - 1
                                && sets_graph[n].round == 1
                                && sets_graph[n].position == i
                        })
                        .unwrap(),
                    NodeIndex::from(sets_graph.node_count() as u32 - 1),
                    SetEdge {
                        outcome: "loser".to_owned(),
                        position: 2,
                    },
                );
            }
        }
    }

    // * COMPLETED STEP 1. (c)
    for k in 2..first_bracket_exponent {
        // * COMPLETED STEP 1. (c) I.
        for i in 1..=full_first_bracket_first_round_sets_count >> (k - 1) as u64 {
            println!("bracket:{}, round:{}, position:{}", 1, k, i);
            game_number += 1;
            sets_graph.add_node(GraphSet {
                bracket: 1,
                game: game_number,
                round: k,
                position: i,
                placeholders: (
                    "W".to_owned()
                        + &sets_graph[sets_graph
                            .node_indices()
                            .find(|&n| {
                                sets_graph[n].bracket == 1
                                    && sets_graph[n].round == k - 1
                                    && sets_graph[n].position == i * 2 - 1
                            })
                            .unwrap()]
                        .game
                        .to_string(),
                    "W".to_owned()
                        + &sets_graph[sets_graph
                            .node_indices()
                            .find(|&n| {
                                sets_graph[n].bracket == 1
                                    && sets_graph[n].round == k - 1
                                    && sets_graph[n].position == i * 2
                            })
                            .unwrap()]
                        .game
                        .to_string(),
                ),
            });
            sets_graph.add_edge(
                sets_graph
                    .node_indices()
                    .find(|&n| {
                        sets_graph[n].bracket == 1
                            && sets_graph[n].round == k - 1
                            && sets_graph[n].position == i * 2 - 1
                    })
                    .unwrap(),
                NodeIndex::from(sets_graph.node_count() as u32 - 1),
                SetEdge {
                    outcome: "winner".to_owned(),
                    position: 1,
                },
            );
            sets_graph.add_edge(
                sets_graph
                    .node_indices()
                    .find(|&n| {
                        sets_graph[n].bracket == 1
                            && sets_graph[n].round == k - 1
                            && sets_graph[n].position == i * 2
                    })
                    .unwrap(),
                NodeIndex::from(sets_graph.node_count() as u32 - 1),
                SetEdge {
                    outcome: "winner".to_owned(),
                    position: 2,
                },
            );
        }
        // * COMPLETED STEP 1. (c) II.
        let mut round_number: u64;
        let mut position_number: u64;
        for j in 2..=tuple {
            round_number = sets_graph[sets_graph
                .node_indices()
                .rfind(|&n| sets_graph[n].bracket == j)
                .unwrap()]
            .round
                + 1;
            // * COMPLETED STEP 1. (g)
            position_number = if j % 2 == 0 && (round_number - (k - 2)) % 4 == 2
                || j % 2 == 1 && (round_number - (k - 2)) % 4 == 0
            {
                (full_first_bracket_first_round_sets_count >> (k - 1)) + 1
            } else if (round_number - (k - 2)) % 4 == 3 {
                (full_first_bracket_first_round_sets_count >> k) + 1
            } else if j % 2 == 0 && (round_number - (k - 2)) % 4 == 0
                || j % 2 == 1 && (round_number - (k - 2)) % 4 == 2
            {
                ((full_first_bracket_first_round_sets_count >> k) + 1) - 1
            } else {
                (1) - 1
            };
            for i in 1..=full_first_bracket_first_round_sets_count >> (k - 1) as u64 {
                println!("bracket:{}, round:{}, position:{}", j, round_number, i);
                game_number += 1;
                position_number = if j % 2 == 0 && (round_number - (k - 2)) % 4 == 2
                    || j % 2 == 1 && (round_number - (k - 2)) % 4 == 0
                {
                    position_number - 1
                } else if (round_number - (k - 2)) % 4 == 3 {
                    if position_number == 1 {
                        full_first_bracket_first_round_sets_count >> (k - 1)
                    } else {
                        position_number - 1
                    }
                } else if j % 2 == 0 && (round_number - (k - 2)) % 4 == 0
                    || j % 2 == 1 && (round_number - (k - 2)) % 4 == 2
                {
                    if position_number == full_first_bracket_first_round_sets_count >> (k - 1) {
                        1
                    } else {
                        position_number + 1
                    }
                } else {
                    position_number + 1
                };
                sets_graph.add_node(GraphSet {
                    bracket: j,
                    game: game_number,
                    round: round_number,
                    position: i,
                    placeholders: (
                        "L".to_owned()
                            + &sets_graph[sets_graph
                                .node_indices()
                                .find(|&n| {
                                    sets_graph[n].bracket == j - 1
                                        && sets_graph[n].round
                                            == sets_graph[sets_graph
                                                .node_indices()
                                                .rfind(|&g| sets_graph[g].bracket == j - 1)
                                                .unwrap()]
                                            .round
                                                + 2
                                                - j
                                        && sets_graph[n].position == position_number
                                })
                                .unwrap()]
                            .game
                            .to_string(),
                        "W".to_owned()
                            + &sets_graph[sets_graph
                                .node_indices()
                                .find(|&n| {
                                    sets_graph[n].bracket == j
                                        && sets_graph[n].round == round_number - 1
                                        && sets_graph[n].position
                                            == i * (if k == 2 { 1 } else { 0 } + 1)
                                })
                                .unwrap()]
                            .game
                            .to_string(),
                    ),
                });
                sets_graph.add_edge(
                    sets_graph
                        .node_indices()
                        .find(|&n| {
                            sets_graph[n].bracket == j - 1
                                && sets_graph[n].round
                                    == sets_graph[sets_graph
                                        .node_indices()
                                        .rfind(|&g| sets_graph[g].bracket == j - 1)
                                        .unwrap()]
                                    .round
                                        + 2
                                        - j
                                && sets_graph[n].position == position_number
                        })
                        .unwrap(),
                    NodeIndex::from(sets_graph.node_count() as u32 - 1),
                    SetEdge {
                        outcome: "loser".to_owned(),
                        position: 1,
                    },
                );
                sets_graph.add_edge(
                    sets_graph
                        .node_indices()
                        .find(|&n| {
                            sets_graph[n].bracket == j
                                && sets_graph[n].round == round_number - 1
                                && sets_graph[n].position == i * (if k == 2 { 1 } else { 0 } + 1)
                        })
                        .unwrap(),
                    NodeIndex::from(sets_graph.node_count() as u32 - 1),
                    SetEdge {
                        outcome: "winner".to_owned(),
                        position: 2,
                    },
                );
            }
            // * COMPLETED STEP 1. (c) II. (1)
            round_number += 1;
            for i in 1..=full_first_bracket_first_round_sets_count >> k as u64 {
                println!("bracket:{}, round:{}, position:{}", j, round_number, i);
                game_number += 1;
                sets_graph.add_node(GraphSet {
                    bracket: j,
                    game: game_number,
                    round: round_number,
                    position: i,
                    placeholders: (
                        "W".to_owned()
                            + &sets_graph[sets_graph
                                .node_indices()
                                .find(|&n| {
                                    sets_graph[n].bracket == j
                                        && sets_graph[n].round == round_number - 1
                                        && sets_graph[n].position == i * 2 - 1
                                })
                                .unwrap()]
                            .game
                            .to_string(),
                        "W".to_owned()
                            + &sets_graph[sets_graph
                                .node_indices()
                                .find(|&n| {
                                    sets_graph[n].bracket == j
                                        && sets_graph[n].round == round_number - 1
                                        && sets_graph[n].position == i * 2
                                })
                                .unwrap()]
                            .game
                            .to_string(),
                    ),
                });
                sets_graph.add_edge(
                    sets_graph
                        .node_indices()
                        .find(|&n| {
                            sets_graph[n].bracket == j
                                && sets_graph[n].round == round_number - 1
                                && sets_graph[n].position == i * 2 - 1
                        })
                        .unwrap(),
                    NodeIndex::from(sets_graph.node_count() as u32 - 1),
                    SetEdge {
                        outcome: "winner".to_owned(),
                        position: 1,
                    },
                );
                sets_graph.add_edge(
                    sets_graph
                        .node_indices()
                        .find(|&n| {
                            sets_graph[n].bracket == j
                                && sets_graph[n].round == round_number - 1
                                && sets_graph[n].position == i * 2
                        })
                        .unwrap(),
                    NodeIndex::from(sets_graph.node_count() as u32 - 1),
                    SetEdge {
                        outcome: "winner".to_owned(),
                        position: 2,
                    },
                );
            }
            for l in 2..j {
                round_number += 1;
                position_number = if j % 2 == 0 && (round_number - (k - 1)) % 4 == 2
                    || j % 2 == 1 && (round_number - (k - 1)) % 4 == 0
                {
                    (full_first_bracket_first_round_sets_count >> k) + 1
                } else if (round_number - (k - 1)) % 4 == 3 {
                    (full_first_bracket_first_round_sets_count >> (k + 1)) + 1
                } else if j % 2 == 0 && (round_number - (k - 1)) % 4 == 0
                    || j % 2 == 1 && (round_number - (k - 1)) % 4 == 2
                {
                    ((full_first_bracket_first_round_sets_count >> (k + 1)) + 1) - 1
                } else {
                    (1) - 1
                };
                for i in 1..=full_first_bracket_first_round_sets_count >> k as u64 {
                    println!(
                        "bracket:{}, round:{}, position:{}, tuple_extra:{}",
                        j,
                        round_number,
                        i,
                        l - 1
                    );
                    game_number += 1;
                    position_number = if j % 2 == 0 && (round_number - (k - 1)) % 4 == 2
                        || j % 2 == 1 && (round_number - (k - 1)) % 4 == 0
                    {
                        position_number - 1
                    } else if (round_number - (k - 1)) % 4 == 3 {
                        if position_number == 1 {
                            full_first_bracket_first_round_sets_count >> k
                        } else {
                            position_number - 1
                        }
                    } else if j % 2 == 0 && (round_number - (k - 1)) % 4 == 0
                        || j % 2 == 1 && (round_number - (k - 1)) % 4 == 2
                    {
                        if position_number == full_first_bracket_first_round_sets_count >> k {
                            1
                        } else {
                            position_number + 1
                        }
                    } else {
                        position_number + 1
                    };
                    sets_graph.add_node(GraphSet {
                        bracket: j,
                        game: game_number,
                        round: round_number,
                        position: i,
                        placeholders: (
                            "L".to_owned()
                                + &sets_graph[sets_graph
                                    .node_indices()
                                    .find(|&n| {
                                        sets_graph[n].bracket == j - 1
                                            && sets_graph[n].round
                                                == &sets_graph[sets_graph
                                                    .node_indices()
                                                    .rfind(|&g| sets_graph[g].bracket == j - 1)
                                                    .unwrap()]
                                                .round
                                                    + l
                                                    - j
                                                    + 1
                                            && sets_graph[n].position == position_number
                                    })
                                    .unwrap()]
                                .game
                                .to_string(),
                            "W".to_owned()
                                + &sets_graph[sets_graph
                                    .node_indices()
                                    .find(|&n| {
                                        sets_graph[n].bracket == j
                                            && sets_graph[n].round == round_number - 1
                                            && sets_graph[n].position == i
                                    })
                                    .unwrap()]
                                .game
                                .to_string(),
                        ),
                    });
                    sets_graph.add_edge(
                        sets_graph
                            .node_indices()
                            .find(|&n| {
                                sets_graph[n].bracket == j - 1
                                    && sets_graph[n].round
                                        == &sets_graph[sets_graph
                                            .node_indices()
                                            .rfind(|&g| sets_graph[g].bracket == j - 1)
                                            .unwrap()]
                                        .round
                                            + l
                                            - j
                                            + 1
                                    && sets_graph[n].position == position_number
                            })
                            .unwrap(),
                        NodeIndex::from(sets_graph.node_count() as u32 - 1),
                        SetEdge {
                            outcome: "loser".to_owned(),
                            position: 1,
                        },
                    );
                    sets_graph.add_edge(
                        sets_graph
                            .node_indices()
                            .find(|&n| {
                                sets_graph[n].bracket == j
                                    && sets_graph[n].round == round_number - 1
                                    && sets_graph[n].position == i
                            })
                            .unwrap(),
                        NodeIndex::from(sets_graph.node_count() as u32 - 1),
                        SetEdge {
                            outcome: "winner".to_owned(),
                            position: 2,
                        },
                    );
                }
            }
        }
    }

    // * COMPLETED STEP 1. (d)
    for k in 1..=tuple {
        // * COMPLETED STEP 1. (d) I.
        println!(
            "bracket:{}, round:{}, position:{}",
            k,
            sets_graph[sets_graph
                .node_indices()
                .rfind(|&n| sets_graph[n].bracket == k)
                .unwrap()]
            .round
                + 1,
            1
        );
        let mut if_nec_indices: Vec<NodeIndex> = Vec::new();
        let mut if_nec_games: String = "".to_owned();
        if k != 1 {
            if_nec_indices = sets_graph
                .node_indices()
                .rev()
                .filter(|&n| sets_graph[n].bracket == k - 1)
                .enumerate()
                .filter(|&(index, _)| index < k as usize - 1)
                .map(|(_, b)| b)
                .collect::<Vec<NodeIndex>>()
                .into_iter()
                .rev()
                .collect::<Vec<NodeIndex>>();
            if_nec_games = if_nec_indices.iter().fold("".to_owned(), |acc, &index| {
                acc + "W" + &sets_graph[index].game.to_string() + "/"
            });
            if_nec_games.truncate(if_nec_games.len() - 1);
        }
        game_number += 1;
        sets_graph.add_node(GraphSet {
            bracket: k,
            game: game_number,
            round: sets_graph[sets_graph
                .node_indices()
                .rfind(|&n| sets_graph[n].bracket == k)
                .unwrap()]
            .round
                + 1,
            position: 1,
            placeholders: if k == 1 {
                (
                    "W".to_owned()
                        + &sets_graph[sets_graph
                            .node_indices()
                            .rfind(|&n| sets_graph[n].bracket == k && sets_graph[n].position == 1)
                            .unwrap()]
                        .game
                        .to_string(),
                    "W".to_owned()
                        + &sets_graph[sets_graph
                            .node_indices()
                            .rfind(|&n| sets_graph[n].bracket == k && sets_graph[n].position == 2)
                            .unwrap()]
                        .game
                        .to_string(),
                )
            } else {
                (
                    if_nec_games,
                    "W".to_owned()
                        + &sets_graph[sets_graph
                            .node_indices()
                            .rfind(|&n| sets_graph[n].bracket == k)
                            .unwrap()]
                        .game
                        .to_string(),
                )
            },
        });
        if k == 1 {
            sets_graph.add_edge(
                sets_graph
                    .node_indices()
                    .rev()
                    .filter(|&n| sets_graph[n].bracket == k && sets_graph[n].position == 1)
                    .enumerate()
                    .find(|&(index, _)| index == 1)
                    .map(|(_, n)| n)
                    .unwrap(),
                NodeIndex::from(sets_graph.node_count() as u32 - 1),
                SetEdge {
                    outcome: "winner".to_owned(),
                    position: 1,
                },
            );
            sets_graph.add_edge(
                sets_graph
                    .node_indices()
                    .rfind(|&n| sets_graph[n].bracket == k && sets_graph[n].position == 2)
                    .unwrap(),
                NodeIndex::from(sets_graph.node_count() as u32 - 1),
                SetEdge {
                    outcome: "winner".to_owned(),
                    position: 2,
                },
            );
        } else {
            if_nec_indices.iter().for_each(|&index| {
                sets_graph.add_edge(
                    index,
                    NodeIndex::from(sets_graph.node_count() as u32 - 1),
                    SetEdge {
                        outcome: "winner".to_owned(),
                        position: 1,
                    },
                );
            });
            sets_graph.add_edge(
                sets_graph
                    .node_indices()
                    .rev()
                    .filter(|&n| sets_graph[n].bracket == k)
                    .enumerate()
                    .find(|&(index, _)| index == 1)
                    .map(|(_, n)| n)
                    .unwrap(),
                NodeIndex::from(sets_graph.node_count() as u32 - 1),
                SetEdge {
                    outcome: "winner".to_owned(),
                    position: 2,
                },
            );
        }
        let mut round_number: u64;
        for j in 2..=tuple {
            // * COMPLETED STEP 1. (d) III.
            if j <= k {
                round_number = sets_graph[sets_graph
                    .node_indices()
                    .rfind(|&n| sets_graph[n].bracket == k)
                    .unwrap()]
                .round
                    + 1;
                println!("bracket:{}, round:{}, position:{}", k, round_number, 1);
                game_number += 1;
                sets_graph.add_node(GraphSet {
                    bracket: k,
                    game: game_number,
                    round: round_number,
                    position: 1,
                    placeholders: (
                        "W".to_owned()
                            + &sets_graph[sets_graph
                                .node_indices()
                                .rfind(|&n| sets_graph[n].bracket == k)
                                .unwrap()]
                            .game
                            .to_string(),
                        "L".to_owned()
                            + &sets_graph[sets_graph
                                .node_indices()
                                .rfind(|&n| sets_graph[n].bracket == k)
                                .unwrap()]
                            .game
                            .to_string(),
                    ),
                });
                sets_graph.add_edge(
                    sets_graph
                        .node_indices()
                        .rev()
                        .filter(|&n| sets_graph[n].bracket == k)
                        .enumerate()
                        .find(|&(index, _)| index == 1)
                        .map(|(_, n)| n)
                        .unwrap(),
                    NodeIndex::from(sets_graph.node_count() as u32 - 1),
                    SetEdge {
                        outcome: "winner".to_owned(),
                        position: 1,
                    },
                );
                sets_graph.add_edge(
                    sets_graph
                        .node_indices()
                        .rev()
                        .filter(|&n| sets_graph[n].bracket == k)
                        .enumerate()
                        .find(|&(index, _)| index == 1)
                        .map(|(_, n)| n)
                        .unwrap(),
                    NodeIndex::from(sets_graph.node_count() as u32 - 1),
                    SetEdge {
                        outcome: "loser".to_owned(),
                        position: 2,
                    },
                );
            }
            // * COMPLETED STEP 1. (d) IV.
            else if j == k + 1 {
                round_number = sets_graph[sets_graph
                    .node_indices()
                    .rfind(|&n| sets_graph[n].bracket == j)
                    .unwrap()]
                .round
                    + 1;
                println!("bracket:{}, round:{}, position:{}", j, round_number, 1);
                if_nec_indices = sets_graph
                    .node_indices()
                    .rev()
                    .filter(|&n| sets_graph[n].bracket == j - 1)
                    .enumerate()
                    .filter(|&(index, _)| index < j as usize - 1)
                    .map(|(_, b)| b)
                    .collect::<Vec<NodeIndex>>()
                    .into_iter()
                    .rev()
                    .collect::<Vec<NodeIndex>>();
                if_nec_games = if_nec_indices.iter().fold("".to_owned(), |acc, &index| {
                    acc + "L" + &sets_graph[index].game.to_string() + "/"
                });
                if_nec_games.truncate(if_nec_games.len() - 1);
                game_number += 1;
                sets_graph.add_node(GraphSet {
                    bracket: j,
                    game: game_number,
                    round: round_number,
                    position: 1,
                    placeholders: (
                        if_nec_games,
                        "W".to_owned()
                            + &sets_graph[sets_graph
                                .node_indices()
                                .rfind(|&n| sets_graph[n].bracket == j)
                                .unwrap()]
                            .game
                            .to_string(),
                    ),
                });
                if_nec_indices.iter().for_each(|&index| {
                    sets_graph.add_edge(
                        index,
                        NodeIndex::from(sets_graph.node_count() as u32 - 1),
                        SetEdge {
                            outcome: "loser".to_owned(),
                            position: 1,
                        },
                    );
                });
                sets_graph.add_edge(
                    sets_graph
                        .node_indices()
                        .rev()
                        .filter(|&n| sets_graph[n].bracket == j)
                        .enumerate()
                        .find(|&(index, _)| index == 1)
                        .map(|(_, n)| n)
                        .unwrap(),
                    NodeIndex::from(sets_graph.node_count() as u32 - 1),
                    SetEdge {
                        outcome: "winner".to_owned(),
                        position: 2,
                    },
                );
            }
            // * COMPLETED STEP 1. (d) II.
            else {
                round_number = sets_graph[sets_graph
                    .node_indices()
                    .rfind(|&n| sets_graph[n].bracket == j)
                    .unwrap()]
                .round
                    + 1;
                println!("bracket:{}, round:{}, position:{}", j, round_number, 1);
                game_number += 1;
                sets_graph.add_node(GraphSet {
                    bracket: j,
                    game: game_number,
                    round: round_number,
                    position: 1,
                    placeholders: (
                        "L".to_owned()
                            + &sets_graph[sets_graph
                                .node_indices()
                                .rfind(|&n| sets_graph[n].bracket == j - 1)
                                .unwrap()]
                            .game
                            .to_string(),
                        "W".to_owned()
                            + &sets_graph[sets_graph
                                .node_indices()
                                .rfind(|&n| sets_graph[n].bracket == j)
                                .unwrap()]
                            .game
                            .to_string(),
                    ),
                });
                sets_graph.add_edge(
                    sets_graph
                        .node_indices()
                        .rfind(|&n| sets_graph[n].bracket == j - 1)
                        .unwrap(),
                    NodeIndex::from(sets_graph.node_count() as u32 - 1),
                    SetEdge {
                        outcome: "loser".to_owned(),
                        position: 1,
                    },
                );
                sets_graph.add_edge(
                    sets_graph
                        .node_indices()
                        .rev()
                        .filter(|&n| sets_graph[n].bracket == j)
                        .enumerate()
                        .find(|&(index, _)| index == 1)
                        .map(|(_, n)| n)
                        .unwrap(),
                    NodeIndex::from(sets_graph.node_count() as u32 - 1),
                    SetEdge {
                        outcome: "winner".to_owned(),
                        position: 2,
                    },
                );
            }
        }
    }

    // * COMPLETED STEP 1. (e)
    sets_graph.clone().node_indices().for_each(|i| {
        // * COMPLETED STEP 1. (e) I.
        if sets_graph[i].game == 0 {
            if sets_graph[i].placeholders.0 == "0" && sets_graph[i].placeholders.1 == "0" {
                sets_graph.clone().edges(i).for_each(|e| {
                    sets_graph[e.target()].game = 0;
                    if e.weight().position == 1 {
                        sets_graph[e.target()].placeholders.0 = "0".to_owned();
                    } else {
                        sets_graph[e.target()].placeholders.1 = "0".to_owned();
                    }
                });
            } else if sets_graph[i].placeholders.0 == "0" || sets_graph[i].placeholders.1 == "0" {
                let winner_placeholder = if sets_graph[i].placeholders.0 == "0" {
                    sets_graph[i].placeholders.1.to_owned()
                } else {
                    sets_graph[i].placeholders.0.to_owned()
                };
                sets_graph.clone().edges(i).for_each(|e| {
                    if e.weight().position == 1 {
                        if e.weight().outcome == "winner" {
                            if sets_graph[i].bracket > 1 && winner_placeholder.starts_with("L") {
                                sets_graph.add_edge(
                                    sets_graph
                                        .edges_directed(i, Incoming)
                                        .find(|f| f.weight().outcome == "loser")
                                        .unwrap()
                                        .source(),
                                    e.target(),
                                    SetEdge {
                                        outcome: "loser".to_owned(),
                                        position: 1,
                                    },
                                );
                                sets_graph.remove_edge(e.id());
                            }
                            sets_graph[e.target()].placeholders.0 = winner_placeholder.to_owned();
                        } else {
                            sets_graph[e.target()].game = 0;
                            sets_graph[e.target()].placeholders.0 = "0".to_owned();
                        }
                    } else {
                        if e.weight().outcome == "winner" {
                            if sets_graph[i].bracket > 1 && winner_placeholder.starts_with("L") {
                                sets_graph.add_edge(
                                    sets_graph
                                        .edges_directed(i, Incoming)
                                        .find(|f| f.weight().outcome == "loser")
                                        .unwrap()
                                        .source(),
                                    e.target(),
                                    SetEdge {
                                        outcome: "loser".to_owned(),
                                        position: 2,
                                    },
                                );
                                sets_graph.remove_edge(e.id());
                            }
                            sets_graph[e.target()].placeholders.1 = winner_placeholder.to_owned();
                        } else {
                            sets_graph[e.target()].game = 0;
                            sets_graph[e.target()].placeholders.1 = "0".to_owned();
                        }
                    }
                });
            } else {
                println!("FAILURE!: {:?}", i);
            }
        }
    });

    // ! COMPLETED STEP 1. (f)
    let mut game_number_offset: Vec<u64> = Vec::new();
    let mut temp_game_number_offset: Vec<u64> = Vec::new();
    let mut k: u64 = 0;
    let mut m: u64 = 0;
    let mut position_number: u64 = 0;
    let mut game_zeros: Vec<NodeIndex> = Vec::new();
    sets_graph.clone().node_indices().for_each(|i| {
        if sets_graph[i].round != 1
            && sets_graph[i].round
                < sets_graph[sets_graph
                    .node_indices()
                    .rfind(|&n| sets_graph[n].bracket == sets_graph[i].bracket)
                    .unwrap()]
                .round
                    - sets_graph[i].bracket
        {
            if sets_graph[i].position == 1 {
                if sets_graph[i].bracket == 1
                    || sets_graph[i].bracket == 2 && (sets_graph[i].round - 1) % 2 == 0
                    || (sets_graph[i].round - 1) % sets_graph[i].bracket == 2 && sets_graph[i].round != sets_graph[sets_graph
                    .node_indices()
                    .rfind(|&n| sets_graph[n].bracket == sets_graph[i].bracket)
                    .unwrap()]
                .round
                    - sets_graph[i].bracket - 1
                {
                    game_number_offset = new_set_scan(
                        sets_graph.clone(),
                        sets_graph[i].bracket,
                        sets_graph[i].round - 1,
                    );
                } else {
                    game_number_offset = small_set_scan(
                        sets_graph.clone(),
                        sets_graph[i].bracket - 1,
sets_graph[sets_graph
                                        .edges_directed(i, Incoming).skip(1)
                                        .find(|f| f.weight().outcome == "loser")
                                        .unwrap()
                                        .source()].round,
                    );
                    println!("{:?}", sets_graph.edges_directed(i, Incoming).collect::<Vec<EdgeReference<SetEdge>>>());
                    println!("{:?}", game_number_offset);
                    println!("connected b & r: {}, {}, b & r: {}, {}", sets_graph[i].bracket - 1, sets_graph[sets_graph
                                        .edges_directed(i, Incoming)
                                        .find(|f| f.weight().outcome == "loser")
                                        .unwrap()
                                        .source()].round, sets_graph[i].bracket, sets_graph[i].round);
                    temp_game_number_offset = Vec::new();
                    k = sets_graph[sets_graph
                        .node_indices()
                        .rfind(|&n| n < i && sets_graph[n].bracket == 1)
                        .unwrap()]
                    .round;
                    m = if (sets_graph[i].round - 1) % sets_graph[i].bracket > 2
                        || (sets_graph[i].round - 1) % sets_graph[i].bracket == 0
                    {
                        1
                    } else {
                        2
                    };
                    position_number = if sets_graph[i].bracket % 2 == 0
                        && (sets_graph[i].round - (k - m)) % 4 == 2
                        || sets_graph[i].bracket % 2 == 1
                            && (sets_graph[i].round - (k - m)) % 4 == 0
                    {
                        (full_first_bracket_first_round_sets_count >> (k - m + 1)) + 1
                    } else if (sets_graph[i].round - (k - m)) % 4 == 3 {
                        println!("LOLOLOL");
                        (full_first_bracket_first_round_sets_count >> (k - m + 2)) + 1
                    } else if sets_graph[i].bracket % 2 == 0
                        && (sets_graph[i].round - (k - m)) % 4 == 0
                        || sets_graph[i].bracket % 2 == 1
                            && (sets_graph[i].round - (k - m)) % 4 == 2
                    {
                        ((full_first_bracket_first_round_sets_count >> (k - m + 2)) + 1) - 1
                    } else {
                        (1) - 1
                    };
                    println!("k: {}, m: {}", k, m);
                    game_number_offset.iter().for_each(|_| {
                                        position_number = if sets_graph[i].bracket % 2 == 0
                    && (sets_graph[i].round - (k - m)) % 4 == 2
                    || sets_graph[i].bracket % 2 == 1 && (sets_graph[i].round - (k - m)) % 4 == 0
                {
                    position_number - 1
                } else if (sets_graph[i].round - (k - m)) % 4 == 3 {
                    if position_number == 1 {
                        full_first_bracket_first_round_sets_count >> (k - m + 1)
                    } else {
                        position_number - 1
                    }
                } else if sets_graph[i].bracket % 2 == 0 && (sets_graph[i].round - (k - m)) % 4 == 0
                    || sets_graph[i].bracket % 2 == 1 && (sets_graph[i].round - (k - m)) % 4 == 2
                {
                    if position_number == full_first_bracket_first_round_sets_count >> (k - m + 1) {
                        1
                    } else {
                        position_number + 1
                    }
                } else {
                    position_number + 1
                };
                println!("position_number: {}", position_number);
                    temp_game_number_offset.push(game_number_offset[position_number as usize - 1]);
                    }
                );
                    game_number_offset = temp_game_number_offset.clone();
                }
            }
                println!("connected_bracket: {}, connected_round: {}, bracket: {}, round: {}, position: {}, game: {}, game_number_offset: {}, game_number_offsets: {:?}",
                sets_graph[i].bracket - if sets_graph[i].bracket == 1
                    || sets_graph[i].bracket == 2 && (sets_graph[i].round - 1) % 2 == 0
                    || (sets_graph[i].round - 1) % sets_graph[i].bracket == 2
                {0} else {1}, if sets_graph[i].bracket == 1
                    || sets_graph[i].bracket == 2 && (sets_graph[i].round - 1) % 2 == 0
                    || (sets_graph[i].round - 1) % sets_graph[i].bracket == 2
                {
                    sets_graph[i].round - 1
                } else {
                    sets_graph[sets_graph
                                        .edges_directed(i, Incoming)
                                        .find(|f| f.weight().outcome == "loser")
                                        .unwrap()
                                        .source()].round}, sets_graph[i].bracket, sets_graph[i].round, sets_graph[i].position, sets_graph[i].game, game_number_offset[sets_graph[i].position as usize - 1], game_number_offset,
                );

                if sets_graph[i].game != 0 {
                    sets_graph[i].game = sets_graph[i].game
                        + game_number_offset[sets_graph[i].position as usize - 1]
                        - sets_graph[i].position;
                } else {
                    sets_graph
                    .node_indices()
                    .filter(|&g| sets_graph[g].game != 0 && if sets_graph[g].bracket == sets_graph[i].bracket && sets_graph[g].round == sets_graph[i].round {game_number_offset[sets_graph[g].position as usize - 1] > game_number_offset[sets_graph[i].position as usize - 1]} else { g > i })
                    .collect::<Vec<NodeIndex>>()
                    .iter()
                    .for_each(|&g| sets_graph[g].game -= 1);
                    game_zeros.push(i);
                }
        }
    });

    game_zeros.iter().for_each(|&i| {
        sets_graph.remove_node(i);
    });

    // * COMPLETED STEP 1. (e) II.
    sets_graph.clone().node_indices().for_each(|i| {
        if sets_graph[i].game == 0 {
            sets_graph
                .node_indices()
                .filter(|&g| g > i && sets_graph[g].game != 0)
                .collect::<Vec<NodeIndex>>()
                .iter()
                .for_each(|&g| sets_graph[g].game -= 1);
        }
    });

    // * COMPLETED STEP 1. (e) III.
    sets_graph.clone().node_indices().rev().for_each(|i| {
        if sets_graph[i].game == 0 {
            sets_graph.remove_node(i);
        }
    });

    // * COMPLETED STEP 1. (e) IV.
    for j in 1..=tuple {
        let mut i_offset: u64 = 0;
        let init_i: u64 = sets_graph[sets_graph
            .node_indices()
            .rfind(|&n| sets_graph[n].bracket == j)
            .unwrap()]
        .round;
        for i in 1..=sets_graph[sets_graph
            .node_indices()
            .rfind(|&n| sets_graph[n].bracket == j)
            .unwrap()]
        .round
        {
            if sets_graph
                .node_indices()
                .find(|&n| sets_graph[n].bracket == j && sets_graph[n].round == i - i_offset)
                .is_none()
            {
                if i < init_i - i_offset {
                    println!("bracket: {}, round: {}", j, i);
                }
                sets_graph
                    .node_indices()
                    .filter(|&n| sets_graph[n].bracket == j && sets_graph[n].round > i - i_offset)
                    .collect::<Vec<NodeIndex>>()
                    .iter()
                    .for_each(|&n| sets_graph[n].round -= 1);
                i_offset += 1;
            }
        }
    }

    // ! COMPLETED STEP 1. (h)
    sets_graph.clone().node_indices().for_each(|n| {
        let mut k = n;
        for i in 1..=sets_graph[sets_graph
            .node_indices()
            .rfind(|&g| sets_graph[g].bracket == sets_graph[n].bracket)
            .unwrap()]
        .round
            - sets_graph[n].round
        {
            println!("bracket: {}, round: {}, {:?}", sets_graph[n].bracket, i, k);
            let t = sets_graph
                .edges(k)
                .find(|e| e.weight().outcome == "winner")
                .unwrap()
                .target();
            if sets_graph
                .edges(k)
                .find(|e| e.weight().outcome == "winner" && e.weight().position == 1)
                .is_some()
            {
                sets_graph[t].placeholders.0 = "W".to_owned() + &sets_graph[k].game.to_string();
            } else if sets_graph
                .edges(k)
                .find(|e| e.weight().outcome == "winner" && e.weight().position == 2)
                .is_some()
            {
                sets_graph[t].placeholders.1 = "W".to_owned() + &sets_graph[k].game.to_string();
            }
            k = sets_graph
                .edges(k)
                .find(|e| e.weight().outcome == "winner")
                .unwrap()
                .target();
        }
        k = n;
        for i in 1..=tuple - sets_graph[n].bracket {
            println!("bracket: {}, round: {}, {:?}", sets_graph[n].bracket, i, k);
            let t = sets_graph
                .edges(k)
                .find(|e| e.weight().outcome == "loser")
                .unwrap()
                .target();
            if sets_graph
                .edges(k)
                .find(|e| e.weight().outcome == "loser" && e.weight().position == 1)
                .is_some()
            {
                sets_graph[t].placeholders.0 = "L".to_owned() + &sets_graph[k].game.to_string();
            } else if sets_graph
                .edges(k)
                .find(|e| e.weight().outcome == "loser" && e.weight().position == 2)
                .is_some()
            {
                sets_graph[t].placeholders.1 = "L".to_owned() + &sets_graph[k].game.to_string();
            }
            k = sets_graph
                .edges(k)
                .find(|e| e.weight().outcome == "loser")
                .unwrap()
                .target();
        }
    });

    // * COMPLETED STEP 1. (i)
    // sets_graph.clone().node_indices().for_each(|n| {
    //     let mut position: u64 = 1;
    //     let mut offset: u64 = 0;
    //     let mut k = n;
    //     for i in 1..=sets_graph[sets_graph
    //         .node_indices()
    //         .rfind(|&g| sets_graph[g].bracket == sets_graph[n].bracket)
    //         .unwrap()]
    //     .round
    //         - sets_graph[n].round
    //     {
    //         println!(
    //             "bracket: {}, round: {}, {:?}, position: {}, offset: {}",
    //             sets_graph[n].bracket, i, k, position, offset
    //         );
    //         if sets_graph
    //             .edges(k)
    //             .find(|e| e.weight().outcome == "winner" && e.weight().position == 1)
    //             .is_some()
    //         {
    //             offset += position;
    //         }
    //         position *= 2;
    //         k = sets_graph
    //             .edges(k)
    //             .find(|e| e.weight().outcome == "winner")
    //             .unwrap()
    //             .target();
    //     }
    //     position -= offset;
    //     sets_graph[n].position = position;
    // });

    // ? BREAK =============================================================== ? \\
    // ! =============================================================== BREAK ! \\

    {
        // let mut reverse_indices = sets_graph.node_indices().collect::<Vec<NodeIndex>>();
        // // println!("{:?}", reverse_indices);
        // reverse_indices.sort_by(|a, b| b.cmp(&a));
        // // println!("{:?}", reverse_indices);
        // reverse_indices.iter().for_each(|index| {
        //     if sets_graph[*index].game == 0 {
        //         // println!("{:?}", index);
        //         sets_graph.remove_node(*index);
        //     }
        // });

        // sets_graph
        //     .clone()
        //     .node_weights_mut()
        //     .map(|set| set.to_owned())
        //     .filter(|n| n.bracket == 1 && n.round == 1 && n.placeholders.1 == "0")
        //     .for_each(|n| {
        //         let node_index = sets_graph
        //             .node_indices()
        //             .find(|&i| {
        //                 sets_graph[i].bracket == n.bracket
        //                     && sets_graph[i].round == n.round
        //                     && sets_graph[i].position == n.position
        //             })
        //             .unwrap();
        //         let temp_sets = sets_graph.clone();

        //         let later_games = sets_graph
        //             .node_indices()
        //             .filter(|&g| sets_graph[g].game > sets_graph[node_index].game)
        //             .collect::<Vec<NodeIndex>>();
        //         later_games.iter().for_each(|&g| sets_graph[g].game -= 1);

        //         sets_graph[node_index].game = 0;

        //         if EdgeRef::weight(
        //             &temp_sets
        //                 .edges(node_index)
        //                 .find(|e| e.weight().outcome == "winner")
        //                 .unwrap(),
        //         )
        //         .position
        //             == 1
        //         {
        //             sets_graph[EdgeRef::target(
        //                 &temp_sets
        //                     .edges(node_index)
        //                     .find(|e| e.weight().outcome == "winner")
        //                     .unwrap(),
        //             )]
        //             .placeholders
        //             .0 = players[triangle(first_bracket_exponent, n.position) as usize - 1].to_owned();
        //         } else {
        //             sets_graph[EdgeRef::target(
        //                 &temp_sets
        //                     .edges(node_index)
        //                     .find(|e| e.weight().outcome == "winner")
        //                     .unwrap(),
        //             )]
        //             .placeholders
        //             .1 = players[triangle(first_bracket_exponent, n.position) as usize - 1].to_owned();
        //         }

        //         if EdgeRef::weight(
        //             &temp_sets
        //                 .edges(node_index)
        //                 .find(|e| e.weight().outcome == "loser")
        //                 .unwrap(),
        //         )
        //         .position
        //             == 1
        //         {
        //             sets_graph[EdgeRef::target(
        //                 &temp_sets
        //                     .edges(node_index)
        //                     .find(|e| e.weight().outcome == "loser")
        //                     .unwrap(),
        //             )]
        //             .placeholders
        //             .0 = "0".to_owned();
        //         } else {
        //             sets_graph[EdgeRef::target(
        //                 &temp_sets
        //                     .edges(node_index)
        //                     .find(|e| e.weight().outcome == "loser")
        //                     .unwrap(),
        //             )]
        //             .placeholders
        //             .1 = "0".to_owned();
        //         }
        //     });

        // sets_graph
        //     .clone()
        //     .node_weights_mut()
        //     .map(|set| set.to_owned())
        //     .filter(|n| {
        //         n.bracket == 2
        //             && n.round == 1
        //             && n.game != 0
        //             && (n.placeholders.0 == "0" || n.placeholders.1 == "0")
        //     })
        //     .for_each(|n| {
        //         let node_index = sets_graph
        //             .node_indices()
        //             .find(|&i| {
        //                 sets_graph[i].bracket == n.bracket
        //                     && sets_graph[i].round == n.round
        //                     && sets_graph[i].position == n.position
        //             })
        //             .unwrap();
        //         let temp_sets = sets_graph.clone();

        //         let later_games = sets_graph
        //             .node_indices()
        //             .filter(|&g| sets_graph[g].game > sets_graph[node_index].game)
        //             .collect::<Vec<NodeIndex>>();
        //         later_games.iter().for_each(|&g| sets_graph[g].game -= 1);

        //         sets_graph[node_index].game = 0;

        //         if n.placeholders.0 == "0" && n.placeholders.1 == "0" {
        //             if EdgeRef::weight(
        //                 &temp_sets
        //                     .edges(node_index)
        //                     .find(|e| e.weight().outcome == "winner")
        //                     .unwrap(),
        //             )
        //             .position
        //                 == 1
        //             {
        //                 sets_graph[EdgeRef::target(
        //                     &temp_sets
        //                         .edges(node_index)
        //                         .find(|e| e.weight().outcome == "winner")
        //                         .unwrap(),
        //                 )]
        //                 .placeholders
        //                 .0 = "0".to_owned();
        //             } else {
        //                 sets_graph[EdgeRef::target(
        //                     &temp_sets
        //                         .edges(node_index)
        //                         .find(|e| e.weight().outcome == "winner")
        //                         .unwrap(),
        //                 )]
        //                 .placeholders
        //                 .1 = "0".to_owned();
        //             }
        //         } else {
        //             if EdgeRef::weight(
        //                 &temp_sets
        //                     .edges(node_index)
        //                     .find(|e| e.weight().outcome == "winner")
        //                     .unwrap(),
        //             )
        //             .position
        //                 == 1
        //             {
        //                 sets_graph[EdgeRef::target(
        //                     &temp_sets
        //                         .edges(node_index)
        //                         .find(|e| e.weight().outcome == "winner")
        //                         .unwrap(),
        //                 )]
        //                 .placeholders
        //                 .0 = if n.placeholders.0 != "0" {
        //                     n.placeholders.0
        //                 } else {
        //                     n.placeholders.1
        //                 };
        //             } else {
        //                 sets_graph[EdgeRef::target(
        //                     &temp_sets
        //                         .edges(node_index)
        //                         .find(|e| e.weight().outcome == "winner")
        //                         .unwrap(),
        //                 )]
        //                 .placeholders
        //                 .1 = if n.placeholders.0 != "0" {
        //                     n.placeholders.0
        //                 } else {
        //                     n.placeholders.1
        //                 };
        //             }
        //         }
        //     });

        // sets_graph
        //     .clone()
        //     .node_weights_mut()
        //     .map(|set| set.to_owned())
        //     .filter(|n| {
        //         n.bracket == 2
        //             && n.round == 2
        //             && n.game != 0
        //             && (n.placeholders.0 == "0" || n.placeholders.1 == "0")
        //     })
        //     .for_each(|n| {
        //         let node_index = sets_graph
        //             .node_indices()
        //             .find(|&i| {
        //                 sets_graph[i].bracket == n.bracket
        //                     && sets_graph[i].round == n.round
        //                     && sets_graph[i].position == n.position
        //             })
        //             .unwrap();
        //         let temp_sets = sets_graph.clone();

        //         let later_games = sets_graph
        //             .node_indices()
        //             .filter(|&g| sets_graph[g].game > sets_graph[node_index].game)
        //             .collect::<Vec<NodeIndex>>();
        //         later_games.iter().for_each(|&g| sets_graph[g].game -= 1);

        //         sets_graph[node_index].game = 0;

        //         if n.placeholders.0 == "0" && n.placeholders.1 == "0" {
        //             if EdgeRef::weight(
        //                 &temp_sets
        //                     .edges(node_index)
        //                     .find(|e| e.weight().outcome == "winner")
        //                     .unwrap(),
        //             )
        //             .position
        //                 == 1
        //             {
        //                 sets_graph[EdgeRef::target(
        //                     &temp_sets
        //                         .edges(node_index)
        //                         .find(|e| e.weight().outcome == "winner")
        //                         .unwrap(),
        //                 )]
        //                 .placeholders
        //                 .0 = "0".to_owned();
        //             } else {
        //                 sets_graph[EdgeRef::target(
        //                     &temp_sets
        //                         .edges(node_index)
        //                         .find(|e| e.weight().outcome == "winner")
        //                         .unwrap(),
        //                 )]
        //                 .placeholders
        //                 .1 = "0".to_owned();
        //             }
        //         } else {
        //             if EdgeRef::weight(
        //                 &temp_sets
        //                     .edges(node_index)
        //                     .find(|e| e.weight().outcome == "winner")
        //                     .unwrap(),
        //             )
        //             .position
        //                 == 1
        //             {
        //                 sets_graph[EdgeRef::target(
        //                     &temp_sets
        //                         .edges(node_index)
        //                         .find(|e| e.weight().outcome == "winner")
        //                         .unwrap(),
        //                 )]
        //                 .placeholders
        //                 .0 = if n.placeholders.0 != "0" {
        //                     n.placeholders.0
        //                 } else {
        //                     n.placeholders.1
        //                 };
        //             } else {
        //                 sets_graph[EdgeRef::target(
        //                     &temp_sets
        //                         .edges(node_index)
        //                         .find(|e| e.weight().outcome == "winner")
        //                         .unwrap(),
        //                 )]
        //                 .placeholders
        //                 .1 = if n.placeholders.0 != "0" {
        //                     n.placeholders.0
        //                 } else {
        //                     n.placeholders.1
        //                 };
        //             }
        //         }
        //     });

        // let mut reverse_indices = sets_graph.node_indices().collect::<Vec<NodeIndex>>();
        // // println!("{:?}", reverse_indices);
        // reverse_indices.sort_by(|a, b| b.cmp(&a));
        // // println!("{:?}", reverse_indices);
        // reverse_indices.iter().for_each(|index| {
        //     if sets_graph[*index].game == 0 {
        //         // println!("{:?}", index);
        //         sets_graph.remove_node(*index);
        //     }
        // });

        // sets_graph
        //     .clone()
        //     .node_weights_mut()
        //     .map(|set| set.to_owned())
        //     .filter(|n| n.bracket == 1 && n.round == 1 && n.placeholders.1 == "0")
        //     .for_each(|n| {
        //         let node_index = sets_graph
        //             .node_indices()
        //             .find(|&i| {
        //                 sets_graph[i].bracket == n.bracket
        //                     && sets_graph[i].round == n.round
        //                     && sets_graph[i].position == n.position
        //             })
        //             .unwrap();
        //         let temp_sets = sets_graph.clone();

        //         let later_games = sets_graph
        //             .node_indices()
        //             .filter(|&g| sets_graph[g].game > sets_graph[node_index].game)
        //             .collect::<Vec<NodeIndex>>();
        //         later_games.iter().for_each(|&g| sets_graph[g].game -= 1);

        //         sets_graph[node_index].game = 0;

        //         if EdgeRef::weight(
        //             &temp_sets
        //                 .edges(node_index)
        //                 .find(|e| e.weight().outcome == "winner")
        //                 .unwrap(),
        //         )
        //         .position
        //             == 1
        //         {
        //             sets_graph[EdgeRef::target(
        //                 &temp_sets
        //                     .edges(node_index)
        //                     .find(|e| e.weight().outcome == "winner")
        //                     .unwrap(),
        //             )]
        //             .placeholders
        //             .0 = players[triangle(first_bracket_exponent, n.position) as usize - 1].to_owned();
        //         } else {
        //             sets_graph[EdgeRef::target(
        //                 &temp_sets
        //                     .edges(node_index)
        //                     .find(|e| e.weight().outcome == "winner")
        //                     .unwrap(),
        //             )]
        //             .placeholders
        //             .1 = players[triangle(first_bracket_exponent, n.position) as usize - 1].to_owned();
        //         }

        //         if EdgeRef::weight(
        //             &temp_sets
        //                 .edges(node_index)
        //                 .find(|e| e.weight().outcome == "loser")
        //                 .unwrap(),
        //         )
        //         .position
        //             == 1
        //         {
        //             sets_graph[EdgeRef::target(
        //                 &temp_sets
        //                     .edges(node_index)
        //                     .find(|e| e.weight().outcome == "loser")
        //                     .unwrap(),
        //             )]
        //             .placeholders
        //             .0 = "0".to_owned();
        //         } else {
        //             sets_graph[EdgeRef::target(
        //                 &temp_sets
        //                     .edges(node_index)
        //                     .find(|e| e.weight().outcome == "loser")
        //                     .unwrap(),
        //             )]
        //             .placeholders
        //             .1 = "0".to_owned();
        //         }
        //     });

        // sets_graph
        //     .clone()
        //     .node_weights_mut()
        //     .map(|set| set.to_owned())
        //     .filter(|n| {
        //         n.bracket == 2
        //             && n.round == 1
        //             && n.game != 0
        //             && (n.placeholders.0 == "0" || n.placeholders.1 == "0")
        //     })
        //     .for_each(|n| {
        //         let node_index = sets_graph
        //             .node_indices()
        //             .find(|&i| {
        //                 sets_graph[i].bracket == n.bracket
        //                     && sets_graph[i].round == n.round
        //                     && sets_graph[i].position == n.position
        //             })
        //             .unwrap();
        //         let temp_sets = sets_graph.clone();

        //         let later_games = sets_graph
        //             .node_indices()
        //             .filter(|&g| sets_graph[g].game > sets_graph[node_index].game)
        //             .collect::<Vec<NodeIndex>>();
        //         later_games.iter().for_each(|&g| sets_graph[g].game -= 1);

        //         sets_graph[node_index].game = 0;

        //         if n.placeholders.0 == "0" && n.placeholders.1 == "0" {
        //             if EdgeRef::weight(
        //                 &temp_sets
        //                     .edges(node_index)
        //                     .find(|e| e.weight().outcome == "winner")
        //                     .unwrap(),
        //             )
        //             .position
        //                 == 1
        //             {
        //                 sets_graph[EdgeRef::target(
        //                     &temp_sets
        //                         .edges(node_index)
        //                         .find(|e| e.weight().outcome == "winner")
        //                         .unwrap(),
        //                 )]
        //                 .placeholders
        //                 .0 = "0".to_owned();
        //             } else {
        //                 sets_graph[EdgeRef::target(
        //                     &temp_sets
        //                         .edges(node_index)
        //                         .find(|e| e.weight().outcome == "winner")
        //                         .unwrap(),
        //                 )]
        //                 .placeholders
        //                 .1 = "0".to_owned();
        //             }
        //         } else {
        //             if EdgeRef::weight(
        //                 &temp_sets
        //                     .edges(node_index)
        //                     .find(|e| e.weight().outcome == "winner")
        //                     .unwrap(),
        //             )
        //             .position
        //                 == 1
        //             {
        //                 sets_graph[EdgeRef::target(
        //                     &temp_sets
        //                         .edges(node_index)
        //                         .find(|e| e.weight().outcome == "winner")
        //                         .unwrap(),
        //                 )]
        //                 .placeholders
        //                 .0 = if n.placeholders.0 != "0" {
        //                     n.placeholders.0
        //                 } else {
        //                     n.placeholders.1
        //                 };
        //             } else {
        //                 sets_graph[EdgeRef::target(
        //                     &temp_sets
        //                         .edges(node_index)
        //                         .find(|e| e.weight().outcome == "winner")
        //                         .unwrap(),
        //                 )]
        //                 .placeholders
        //                 .1 = if n.placeholders.0 != "0" {
        //                     n.placeholders.0
        //                 } else {
        //                     n.placeholders.1
        //                 };
        //             }
        //         }
        //     });

        // sets_graph
        //     .clone()
        //     .node_weights_mut()
        //     .map(|set| set.to_owned())
        //     .filter(|n| {
        //         n.bracket == 2
        //             && n.round == 2
        //             && n.game != 0
        //             && (n.placeholders.0 == "0" || n.placeholders.1 == "0")
        //     })
        //     .for_each(|n| {
        //         let node_index = sets_graph
        //             .node_indices()
        //             .find(|&i| {
        //                 sets_graph[i].bracket == n.bracket
        //                     && sets_graph[i].round == n.round
        //                     && sets_graph[i].position == n.position
        //             })
        //             .unwrap();
        //         let temp_sets = sets_graph.clone();

        //         let later_games = sets_graph
        //             .node_indices()
        //             .filter(|&g| sets_graph[g].game > sets_graph[node_index].game)
        //             .collect::<Vec<NodeIndex>>();
        //         later_games.iter().for_each(|&g| sets_graph[g].game -= 1);

        //         sets_graph[node_index].game = 0;

        //         if n.placeholders.0 == "0" && n.placeholders.1 == "0" {
        //             if EdgeRef::weight(
        //                 &temp_sets
        //                     .edges(node_index)
        //                     .find(|e| e.weight().outcome == "winner")
        //                     .unwrap(),
        //             )
        //             .position
        //                 == 1
        //             {
        //                 sets_graph[EdgeRef::target(
        //                     &temp_sets
        //                         .edges(node_index)
        //                         .find(|e| e.weight().outcome == "winner")
        //                         .unwrap(),
        //                 )]
        //                 .placeholders
        //                 .0 = "0".to_owned();
        //             } else {
        //                 sets_graph[EdgeRef::target(
        //                     &temp_sets
        //                         .edges(node_index)
        //                         .find(|e| e.weight().outcome == "winner")
        //                         .unwrap(),
        //                 )]
        //                 .placeholders
        //                 .1 = "0".to_owned();
        //             }
        //         } else {
        //             if EdgeRef::weight(
        //                 &temp_sets
        //                     .edges(node_index)
        //                     .find(|e| e.weight().outcome == "winner")
        //                     .unwrap(),
        //             )
        //             .position
        //                 == 1
        //             {
        //                 sets_graph[EdgeRef::target(
        //                     &temp_sets
        //                         .edges(node_index)
        //                         .find(|e| e.weight().outcome == "winner")
        //                         .unwrap(),
        //                 )]
        //                 .placeholders
        //                 .0 = if n.placeholders.0 != "0" {
        //                     n.placeholders.0
        //                 } else {
        //                     n.placeholders.1
        //                 };
        //             } else {
        //                 sets_graph[EdgeRef::target(
        //                     &temp_sets
        //                         .edges(node_index)
        //                         .find(|e| e.weight().outcome == "winner")
        //                         .unwrap(),
        //                 )]
        //                 .placeholders
        //                 .1 = if n.placeholders.0 != "0" {
        //                     n.placeholders.0
        //                 } else {
        //                     n.placeholders.1
        //                 };
        //             }
        //         }
        //     });

        // game_number = 3;
        // let mut game_number_offset = Vec::new();
        // let mut i_offset: u64;
        // for j in 1..(first_bracket_exponent + 1) {
        //     game_number += game_number_offset.len() as u64;
        //     if j != first_bracket_exponent {
        //         game_number_offset = new_set_scan(sets_graph.clone(), 1, j);
        //         i_offset = 0;
        //     } else {
        //         game_number_offset = vec![1];
        //         i_offset = 1;
        //     }
        //     for i in 0..(full_first_bracket_first_round_sets_count >> j) + i_offset {
        //         println!("j+1:{}, i+1:{}", j + 1, i + 1);
        //         let target_set = sets_graph
        //             .node_weights_mut()
        //             .find(|set| set.bracket == 1 && set.round == j + 1 && set.position == i + 1)
        //             .unwrap();
        //         target_set.game = game_number + game_number_offset[i as usize];
        //     }
        //     if j != first_bracket_exponent {
        //         game_number += game_number_offset.len() as u64;
        //         game_number_offset = new_set_scan(sets_graph.clone(), 1, j);
        //         let mut position_offset: Vec<u64> =
        //             Vec::from_iter(1..=semifull_second_bracket_first_round_sets_count >> j);
        //         if j == 2 || j == 3 {
        //             let (a, b) = game_number_offset.split_at(game_number_offset.len() / 2);
        //             game_number_offset = [b, a].concat();
        //             let (a, b) = position_offset.split_at(position_offset.len() / 2);
        //             position_offset = [b, a].concat();
        //         }
        //         if j == 2 || (j % 2 == 1 && j != 3) {
        //             game_number_offset.reverse();
        //             position_offset.reverse();
        //         }
        //         println!("{:?}, {:?}", game_number_offset, position_offset);
        //         for i in 0..(semifull_second_bracket_first_round_sets_count >> j) {
        //             let target_set = sets_graph
        //                 .node_weights_mut()
        //                 .find(|set| {
        //                     set.bracket == 2
        //                         && set.round == 2 * j
        //                         && set.position
        //                             == losers_brackets_rounds_sets_positions
        //                                 [losers_brackets_rounds_sets_positions_vector_offset
        //                                     [j as usize * 2 - 1]
        //                                     + i as usize]
        //                 })
        //                 .unwrap();
        //             target_set.game = game_number + game_number_offset[i as usize];
        //         }
        //     }
        //     if j < first_bracket_exponent - 1 {
        //         game_number += game_number_offset.len() as u64;
        //         game_number_offset = new_set_scan(sets_graph.clone(), 2, 2 * j);

        //         println!("{:?}", game_number_offset);
        //         for i in 0..(semifull_second_bracket_first_round_sets_count >> (j + 1)) {
        //             let target_set = sets_graph
        //                 .node_weights_mut()
        //                 .find(|set| {
        //                     set.bracket == 2
        //                         && set.round == 2 * j + 1
        //                         && set.position
        //                             == losers_brackets_rounds_sets_positions
        //                                 [losers_brackets_rounds_sets_positions_vector_offset
        //                                     [j as usize * 2]
        //                                     + i as usize]
        //                 })
        //                 .unwrap();
        //             target_set.game = game_number + game_number_offset[i as usize];
        //         }
        //     }
        // }

        // if sets_graph
        //     .node_weights_mut()
        //     .any(|n| n.bracket == 2 && n.round == 1)
        //     == false
        // {
        //     sets_graph
        //         .clone()
        //         .node_weights_mut()
        //         .map(|set| set.to_owned())
        //         .filter(|n| n.bracket == 2 && n.game != 0)
        //         .for_each(|n| {
        //             let node_index = sets_graph
        //                 .node_indices()
        //                 .find(|&i| {
        //                     sets_graph[i].bracket == n.bracket
        //                         && sets_graph[i].round == n.round
        //                         && sets_graph[i].position == n.position
        //                 })
        //                 .unwrap();
        //             sets_graph[node_index].round -= 1;
        //         });
        // }

        // let mut reverse_indices = sets_graph.node_indices().collect::<Vec<NodeIndex>>();
        // // println!("{:?}", reverse_indices);
        // reverse_indices.sort_by(|a, b| b.cmp(&a));
        // // println!("{:?}", reverse_indices);
        // reverse_indices.iter().for_each(|index| {
        //     if sets_graph[*index].game == 0 {
        //         // println!("{:?}", index);
        //         sets_graph.remove_node(*index);
        //     }
        // });

        // let clean_graph = sets_graph.map(|_, n| (n.bracket, n.game), |_, e| e);
    }
    println!("{:?}", Dot::new(&sets_graph));
    sets_graph
}
