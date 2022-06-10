use super::*;
use petgraph::stable_graph::StableGraph;
use std::fmt;

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

pub fn new_elim(players: Vec<String>, tuple: u64) -> () {
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
    println!("{:#?}", sets_graph);
}
