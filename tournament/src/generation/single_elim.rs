use super::*;
use petgraph::dot::*;
use petgraph::stable_graph::*;
use petgraph::visit::EdgeRef;
use petgraph::visit::IntoEdges;
use petgraph::visit::IntoEdgesDirected;
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

fn order_normalize(list: &Vec<Vec<u64>>) -> Vec<u64> {
    let mut largest_games: Vec<(u64, u64)> = Vec::new();
    list.iter()
        .enumerate()
        .for_each(|(index, item)| largest_games.push((*item.iter().max().unwrap(), index as u64)));
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

    println!(
        "{:?}: players_count\n{:?}: full_first_bracket_first_round_sets_count\n{:?}: first_bracket_exponent\n{:?}: full_first_bracket_sets_count",
        players_count,
        full_first_bracket_first_round_sets_count,
        first_bracket_exponent,
        full_first_bracket_sets_count,
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
                    "".to_owned()
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
                    ("".to_owned(), "".to_owned())
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
            if sets_graph
                .edges_directed(i, Incoming)
                .find(|e| e.weight().position == 1 && sets_graph[e.source()].game == 0)
                .is_some()
                && sets_graph
                    .edges_directed(i, Incoming)
                    .find(|e| e.weight().position == 2 && sets_graph[e.source()].game == 0)
                    .is_some()
            {
                sets_graph.clone().edges(i).for_each(|e| {
                    sets_graph[e.target()].game = 0;
                });
                println!("{:?}", sets_graph[i]);
            } else {
                let winner_placeholder = if sets_graph
                    .edges_directed(i, Incoming)
                    .find(|e| e.weight().position == 1 && sets_graph[e.source()].game == 0)
                    .is_none()
                {
                    sets_graph[i].placeholders.0.to_owned()
                } else {
                    sets_graph[i].placeholders.1.to_owned()
                };
                sets_graph.clone().edges(i).for_each(|e| {
                    if e.weight().outcome == "loser" {
                        sets_graph[e.target()].game = 0;
                    } else {
                        if e.weight().position == 1 {
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
                        }
                    }
                });
            }
        }
    });

    // * COMPLETED STEP 1. (f)
    let mut game_number_offsets: Vec<u64> = Vec::new();
    let mut game_numbers: Vec<Vec<u64>> = Vec::new();
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
                sets_graph.node_indices().filter(|&n| sets_graph[n].bracket == sets_graph[i].bracket && sets_graph[n].round == sets_graph[i].round).for_each(|n| game_numbers.push(sets_graph.edges_directed(n, Incoming).map(|e| sets_graph[e.source()].game).collect::<Vec<u64>>()));
                game_number_offsets = order_normalize(&game_numbers);
                game_numbers = Vec::new();
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
                                        .source()].round}, sets_graph[i].bracket, sets_graph[i].round, sets_graph[i].position, sets_graph[i].game, game_number_offsets[sets_graph[i].position as usize - 1], game_number_offsets,
                );

                if sets_graph[i].game != 0 {
                    sets_graph[i].game = sets_graph[i].game
                        + game_number_offsets[sets_graph[i].position as usize - 1]
                        - sets_graph[i].position;
                } else {
                    sets_graph
                    .node_indices()
                    .filter(|&g| sets_graph[g].game != 0 && if sets_graph[g].bracket == sets_graph[i].bracket && sets_graph[g].round == sets_graph[i].round {game_number_offsets[sets_graph[g].position as usize - 1] > game_number_offsets[sets_graph[i].position as usize - 1]} else { g > i })
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

    // * COMPLETED STEP 1. (h)
    sets_graph.clone().node_indices().for_each(|i| {
        if sets_graph
            .edges_directed(i, Incoming)
            .find(|e| e.weight().position == 1)
            .is_some()
        {
            sets_graph[i].placeholders.0 = sets_graph
                .edges_directed(i, Incoming)
                .filter(|e| e.weight().position == 1)
                .fold("".to_owned(), |mut acc, e| {
                    acc.insert_str(
                        0,
                        &(if e.weight().outcome == "winner" {
                            "W"
                        } else {
                            "L"
                        }
                        .to_owned()
                            + &sets_graph[e.source()].game.to_string()
                            + if acc != "" { "/" } else { "" }),
                    );
                    acc
                });
        }
        if sets_graph
            .edges_directed(i, Incoming)
            .find(|e| e.weight().position == 2)
            .is_some()
        {
            sets_graph[i].placeholders.1 = sets_graph
                .edges_directed(i, Incoming)
                .filter(|e| e.weight().position == 2)
                .fold("".to_owned(), |mut acc, e| {
                    acc.insert_str(
                        0,
                        &(if e.weight().outcome == "winner" {
                            "W"
                        } else {
                            "L"
                        }
                        .to_owned()
                            + &sets_graph[e.source()].game.to_string()
                            + if acc != "" { "/" } else { "" }),
                    );
                    acc
                });
        }
    });

    // ! COMPLETED STEP 1. (i)
    sets_graph.clone().node_indices().for_each(|n| {
        let mut position: u64 = 1;
        let mut offset: u64 = 0;
        let mut k = n;
        for i in 1..=sets_graph[sets_graph
            .node_indices()
            .rfind(|&g| sets_graph[g].bracket == sets_graph[n].bracket)
            .unwrap()]
        .round
            - sets_graph[n].round
        {
            println!(
                "bracket: {}, round: {}, {:?}, position: {}, offset: {}",
                sets_graph[n].bracket, i, k, position, offset
            );
            if sets_graph
                .edges(k)
                .find(|e| e.weight().outcome == "winner" && e.weight().position == 1)
                .is_some()
            {
                offset += position;
            }
            position *= 2;
            k = sets_graph
                .edges(k)
                .find(|e| e.weight().outcome == "winner")
                .unwrap()
                .target();
        }
        position -= offset;
        sets_graph[n].position = position;
    });

    // let clean_graph = sets_graph.map(|_, n| (n.bracket, n.game), |_, e| e);
    println!("{:?}", Dot::new(&sets_graph));
    sets_graph
}
