use super::*;

fn outcome_set(
    mut array: Vec<Set>,
    bracket_p1: u64,
    bracket_p2: u64,
    round_p1: u64,
    round_p2: u64,
    pos_p1: u64,
    pos_p2: u64,
    outcomes: &str,
) {
    if outcomes == "ww" {
        let array_index = array
            .iter()
            .position(|obj| {
                obj.round == round_p1 && obj.bracket == bracket_p1 && obj.position == pos_p1
            })
            .unwrap();
        array[array_index].outcomes[0] = (array.len() - 1).to_string() + "p1";
        if !array[array
            .iter()
            .position(|obj| {
                obj.round == round_p1 && obj.bracket == bracket_p1 && obj.position == pos_p1
            })
            .unwrap()]
        .winners[0]
            .is_empty()
        {
            let array_index = array.len() - 1;
            let array_index2 = array
                .iter()
                .position(|obj| {
                    obj.round == round_p1 && obj.bracket == bracket_p1 && obj.position == pos_p1
                })
                .unwrap();
            array[array_index].players[0] = array[array_index2].winners[0].clone();
        } else {
            let array_index = array.len() - 1;
            array[array_index].players[0] = "W".to_string()
                + &array
                    .iter()
                    .position(|obj| {
                        obj.round == round_p1 && obj.bracket == bracket_p1 && obj.position == pos_p1
                    })
                    .unwrap()
                    .to_string();
        }
        let array_index = array
            .iter()
            .position(|obj| {
                obj.round == round_p2 && obj.bracket == bracket_p2 && obj.position == pos_p2
            })
            .unwrap();
        array[array_index].outcomes[0] = (array.len() - 1).to_string() + "p2";
        if !array[array
            .iter()
            .position(|obj| {
                obj.round == round_p2 && obj.bracket == bracket_p2 && obj.position == pos_p2
            })
            .unwrap()]
        .winners[0]
            .is_empty()
        {
            let array_index = array.len() - 1;
            let array_index2 = array
                .iter()
                .position(|obj| {
                    obj.round == round_p2 && obj.bracket == bracket_p2 && obj.position == pos_p2
                })
                .unwrap();
            array[array_index].players[1] = array[array_index2].winners[0].clone();
        } else {
            let array_index = array.len() - 1;
            array[array_index].players[1] = "W".to_string()
                + &array
                    .iter()
                    .position(|obj| {
                        obj.round == round_p2 && obj.bracket == bracket_p2 && obj.position == pos_p2
                    })
                    .unwrap()
                    .to_string();
        }
    }
    if outcomes == "wl" {
        let array_index = array
            .iter()
            .position(|obj| {
                obj.round == round_p1 && obj.bracket == bracket_p1 && obj.position == pos_p1
            })
            .unwrap();
        array[array_index].outcomes[0] = (array.len() - 1).to_string() + "p1";
        if !array[array
            .iter()
            .position(|obj| {
                obj.round == round_p1 && obj.bracket == bracket_p1 && obj.position == pos_p1
            })
            .unwrap()]
        .winners[0]
            .is_empty()
        {
            let array_index = array.len() - 1;
            let array_index2 = array
                .iter()
                .position(|obj| {
                    obj.round == round_p1 && obj.bracket == bracket_p1 && obj.position == pos_p1
                })
                .unwrap();
            array[array_index].players[0] = array[array_index2].winners[0].clone();
        } else {
            let array_index = array.len() - 1;
            array[array_index].players[0] = "W".to_string()
                + &array
                    .iter()
                    .position(|obj| {
                        obj.round == round_p1 && obj.bracket == bracket_p1 && obj.position == pos_p1
                    })
                    .unwrap()
                    .to_string();
        }
        let array_index = array
            .iter()
            .position(|obj| {
                obj.round == round_p2 && obj.bracket == bracket_p2 && obj.position == pos_p2
            })
            .unwrap();
        array[array_index].outcomes[1] = (array.len() - 1).to_string() + "p2";
        if !array[array
            .iter()
            .position(|obj| {
                obj.round == round_p2 && obj.bracket == bracket_p2 && obj.position == pos_p2
            })
            .unwrap()]
        .losers[0]
            .is_empty()
        {
            let array_index = array.len() - 1;
            let array_index2 = array
                .iter()
                .position(|obj| {
                    obj.round == round_p2 && obj.bracket == bracket_p2 && obj.position == pos_p2
                })
                .unwrap();
            array[array_index].players[1] = array[array_index2].losers[0].clone();
        } else {
            let array_index = array.len() - 1;
            array[array_index].players[1] = "L".to_string()
                + &array
                    .iter()
                    .position(|obj| {
                        obj.round == round_p2 && obj.bracket == bracket_p2 && obj.position == pos_p2
                    })
                    .unwrap()
                    .to_string();
        }
    }
    if outcomes == "lw" {
        let array_index = array
            .iter()
            .position(|obj| {
                obj.round == round_p1 && obj.bracket == bracket_p1 && obj.position == pos_p1
            })
            .unwrap();
        array[array_index].outcomes[1] = (array.len() - 1).to_string() + "p1";
        if !array[array
            .iter()
            .position(|obj| {
                obj.round == round_p1 && obj.bracket == bracket_p1 && obj.position == pos_p1
            })
            .unwrap()]
        .losers[0]
            .is_empty()
        {
            let array_index = array.len() - 1;
            let array_index2 = array
                .iter()
                .position(|obj| {
                    obj.round == round_p1 && obj.bracket == bracket_p1 && obj.position == pos_p1
                })
                .unwrap();
            array[array_index].players[0] = array[array_index2].losers[0].clone();
        } else {
            let array_index = array.len() - 1;
            array[array_index].players[0] = "L".to_string()
                + &array
                    .iter()
                    .position(|obj| {
                        obj.round == round_p1 && obj.bracket == bracket_p1 && obj.position == pos_p1
                    })
                    .unwrap()
                    .to_string();
        }
        let array_index = array
            .iter()
            .position(|obj| {
                obj.round == round_p2 && obj.bracket == bracket_p2 && obj.position == pos_p2
            })
            .unwrap();
        array[array_index].outcomes[0] = (array.len() - 1).to_string() + "p2";
        if !array[array
            .iter()
            .position(|obj| {
                obj.round == round_p2 && obj.bracket == bracket_p2 && obj.position == pos_p2
            })
            .unwrap()]
        .winners[0]
            .is_empty()
        {
            let array_index = array.len() - 1;
            let array_index2 = array
                .iter()
                .position(|obj| {
                    obj.round == round_p2 && obj.bracket == bracket_p2 && obj.position == pos_p2
                })
                .unwrap();
            array[array_index].players[1] = array[array_index2].winners[0].clone();
        } else {
            let array_index = array.len() - 1;
            array[array_index].players[1] = "W".to_string()
                + &array
                    .iter()
                    .position(|obj| {
                        obj.round == round_p2 && obj.bracket == bracket_p2 && obj.position == pos_p2
                    })
                    .unwrap()
                    .to_string();
        }
    }
    if outcomes == "ll" {
        let array_index = array
            .iter()
            .position(|obj| {
                obj.round == round_p1 && obj.bracket == bracket_p1 && obj.position == pos_p1
            })
            .unwrap();
        array[array_index].outcomes[1] = (array.len() - 1).to_string() + "p1";
        if !array[array
            .iter()
            .position(|obj| {
                obj.round == round_p1 && obj.bracket == bracket_p1 && obj.position == pos_p1
            })
            .unwrap()]
        .losers[0]
            .is_empty()
        {
            let array_index = array.len() - 1;
            let array_index2 = array
                .iter()
                .position(|obj| {
                    obj.round == round_p1 && obj.bracket == bracket_p1 && obj.position == pos_p1
                })
                .unwrap();
            array[array_index].players[0] = array[array_index2].losers[0].clone();
        } else {
            let array_index = array.len() - 1;
            array[array_index].players[0] = "L".to_string()
                + &array
                    .iter()
                    .position(|obj| {
                        obj.round == round_p1 && obj.bracket == bracket_p1 && obj.position == pos_p1
                    })
                    .unwrap()
                    .to_string();
        }
        let array_index = array
            .iter()
            .position(|obj| {
                obj.round == round_p2 && obj.bracket == bracket_p2 && obj.position == pos_p2
            })
            .unwrap();
        array[array_index].outcomes[1] = (array.len() - 1).to_string() + "p2";
        if !array[array
            .iter()
            .position(|obj| {
                obj.round == round_p2 && obj.bracket == bracket_p2 && obj.position == pos_p2
            })
            .unwrap()]
        .losers[0]
            .is_empty()
        {
            let array_index = array.len() - 1;
            let array_index2 = array
                .iter()
                .position(|obj| {
                    obj.round == round_p2 && obj.bracket == bracket_p2 && obj.position == pos_p2
                })
                .unwrap();
            array[array_index].players[1] = array[array_index2].losers[0].clone();
        } else {
            let array_index = array.len() - 1;
            array[array_index].players[1] = "L".to_string()
                + &array
                    .iter()
                    .position(|obj| {
                        obj.round == round_p2 && obj.bracket == bracket_p2 && obj.position == pos_p2
                    })
                    .unwrap()
                    .to_string();
        }
    }
}
/*
fn doubleEliminationOld(jsonForm, result) {
  const sets = jsonForm.players.split("\n");

  if (result == 'sets') {
    return sets;
  }

  let pCount = sets.len();
  let res = 0;
  while (pCount > 1) {
    res++;
    pCount = pCount / 2;
  }

  if (result == 'res') {
    return res;
  }

  const b_size = 1 << (res);

  if (result == 'b_size') {
    return b_size;
  }

  const byes = b_size - sets.len();

  if (result == 'byes') {
    return byes;
  }

  const setsR1 = sets.len() - (b_size >> 1);

  if (result == 'setsR1') {
    return setsR1;
  }

  let order_array = [];
  let order_array2 = [];
  let setsArray = [];
  let setsArray2 = [];

  for (let i = 0; i < b_size; i++) {
    if (i < byes) {
      setsArray[i] = "\n";
    } else if (i < sets.len()) {
      setsArray[i] = sets[i];
    } else {
      setsArray[i] = "\n";
    }
  }
  for (let i = 0; i < (b_size >> 1); i++) {
    if (i < byes) {
      setsArray2[i] = sets[i];
    } else {
      setsArray2[i] = "\n";
    }
  }

  if (result == 'setsArray') {
    return setsArray;
  }

  if (result == 'setsArray2') {
    return setsArray2;
  }

  for (let i = 0; i < (b_size >> 1); i++) {
    order_array[i] = setsArray[triangle(res + 1, 2 * i + 1) - 1] + "\n" + setsArray[triangle(res + 1, 2 * i + 2) - 1];
  }
  for (let i = 0; i < (b_size >> 2); i++) {
    order_array2[i] = setsArray2[triangle(res, 2 * i + 1) - 1] + "\n" + setsArray2[triangle(res, 2 * i + 2) - 1];
  }

  let players = [];
  sets.forEach(name =>
    players.push(
      {
        "name": name
      }
    )
  );
  let sets_players = [];
  order_array.forEach((set, index) => {
    if (set.split("\n")[0] !== "") {
      sets_players.push(
        {
          "bracket": 0,
          "round": 0,
          "position": index,
          "players": {
            "p1": set.split("\n")[0],
            "p2": set.split("\n")[1]
          },
          "scores": {
            "p1": "",
            "p2": ""
          },
          "winners": {
            "w1": ""
          },
          "losers": {
            "l1": ""
          },
          "outcomes": {
            "winners": "",
            "losers": "eliminated"
          },
          "status": "preview"
        }
      )
    }
  }
  )

  let special = 1;
  if ((b_size >> 1) + (b_size >> 2) < players.len() && players.len() <= b_size) {
    special = 0;
    for (let i = 0; i < players.len() - (b_size >> 1) - (b_size >> 2); i++) {
      sets_players.push(
        {
          "bracket": 1,
          "round": 0,
          "position": reverse_fractal(res - 1, i + 1),
          "players": {
            "p1": "",
            "p2": ""
          },
          "scores": {
            "p1": "",
            "p2": ""
          },
          "winners": {
            "w1": ""
          },
          "losers": {
            "l1": ""
          },
          "outcomes": {
            "winners": "",
            "losers": "eliminated"
          },
          "status": "preview"
        }
      )
    }
  }

  order_array2.forEach((set, index) => {
    if (set.split("\n")[1]) {
      sets_players.push(
        {
          "bracket": 0,
          "round": 1,
          "position": index,
          "players": {
            "p1": set.split("\n")[0],
            "p2": set.split("\n")[1]
          },
          "scores": {
            "p1": "",
            "p2": ""
          },
          "winners": {
            "w1": ""
          },
          "losers": {
            "l1": ""
          },
          "outcomes": {
            "winners": "",
            "losers": "eliminated"
          },
          "status": "preview"
        }
      )
    }
  }
  )

  let bye_array = [];
  order_array2.forEach((set, index) => {
    if (!set.split("\n")[1]) {
      sets_players.push(
        {
          "bracket": 0,
          "round": 1,
          "position": index,
          "players": {
            "p1": set.split("\n")[0],
            "p2": set.split("\n")[1]
          },
          "scores": {
            "p1": "",
            "p2": ""
          },
          "winners": {
            "w1": ""
          },
          "losers": {
            "l1": ""
          },
          "outcomes": {
            "winners": "",
            "losers": "eliminated"
          },
          "status": "preview"
        }
      )
      if (index % 2 == 0) {
        bye_array.push(index / 2);
      }
      if ((sets_players.findIndex(obj => obj.round == 0 && obj.position == index * 2)) !== -1) {
        sets_players[sets_players.findIndex(obj => obj.round == 0 && obj.position == index * 2)].outcomes[0] = sets_players.len() - 1;
      }
      if ((sets_players.findIndex(obj => obj.round == 0 && obj.position == index * 2 + 1)) !== -1) {
        sets_players[sets_players.findIndex(obj => obj.round == 0 && obj.position == index * 2 + 1)].outcomes[0] = sets_players.len() - 1;
      }
    }
  }
  )

  if ((b_size >> 1) + (b_size >> 2) < players.len() && players.len() <= b_size) {
    for (let i = 0; i < losers_filled_round_matches(res); i++) {
      sets_players.push(
        {
          "bracket": 1,
          "round": 1,
          "position": losers_filled_round_matches(res) - i - 1,
          "players": {
            "p1": "",
            "p2": ""
          },
          "scores": {
            "p1": "",
            "p2": ""
          },
          "winners": {
            "w1": ""
          },
          "losers": {
            "l1": ""
          },
          "outcomes": {
            "winners": "",
            "losers": "eliminated"
          },
          "status": "preview"
        }
      )
    }
    for (let i = 0; i < losers_filled_round_matches(res - 1); i++) {
      sets_players.push(
        {
          "bracket": 1,
          "round": 2,
          "position": losers_filled_round_matches(res - 1) - i - 1,
          "players": {
            "p1": "",
            "p2": ""
          },
          "scores": {
            "p1": "",
            "p2": ""
          },
          "winners": {
            "w1": ""
          },
          "losers": {
            "l1": ""
          },
          "outcomes": {
            "winners": "",
            "losers": "eliminated"
          },
          "status": "preview"
        }
      )
    }
  }
  else {
    for (let i = 0; i < players.len() - (b_size >> 1); i++) {
      sets_players.push(
        {
          "bracket": 1,
          "round": 0,
          "position": reverse_fractal(res - 1, i + 1),
          "players": {
            "p1": "",
            "p2": ""
          },
          "scores": {
            "p1": "",
            "p2": ""
          },
          "winners": {
            "w1": ""
          },
          "losers": {
            "l1": ""
          },
          "outcomes": {
            "winners": "",
            "losers": "eliminated"
          },
          "status": "preview"
        }
      )
    }
    for (let i = 0; i < losers_filled_round_matches(res - 1); i++) {
      sets_players.push(
        {
          "bracket": 1,
          "round": 1,
          "position": losers_filled_round_matches(res - 1) - i - 1,
          "players": {
            "p1": "",
            "p2": ""
          },
          "scores": {
            "p1": "",
            "p2": ""
          },
          "winners": {
            "w1": ""
          },
          "losers": {
            "l1": ""
          },
          "outcomes": {
            "winners": "",
            "losers": "eliminated"
          },
          "status": "preview"
        }
      )
    }
  }

  for (let i = 2; i < res; i++) {
    for (let j = 0; j < (b_size >> (i + 1)); j++) {
      if (!bye_array.includes(j)) {
        sets_players.push(
          {
            "bracket": 0,
            "round": i,
            "position": j,
            "players": {
              "p1": "",
              "p2": ""
            },
            "scores": {
              "p1": "",
              "p2": ""
            },
            "winners": {
              "w1": ""
            },
            "losers": {
              "l1": ""
            },
            "outcomes": {
              "winners": "",
              "losers": "eliminated"
            },
            "status": "preview"
          }
        )
        if ((sets_players.findIndex(obj => obj.round == i - 1 && obj.position == j * 2)) !== -1) {
          sets_players[sets_players.findIndex(obj => obj.round == i - 1 && obj.position == j * 2)].outcomes[0] = sets_players.len() - 1;
        }
        if ((sets_players.findIndex(obj => obj.round == i - 1 && obj.position == j * 2 + 1)) !== -1) {
          sets_players[sets_players.findIndex(obj => obj.round == i - 1 && obj.position == j * 2 + 1)].outcomes[0] = sets_players.len() - 1;
        }
      }
    }
    for (let j = 0; j < (b_size >> (i + 1)); j++) {
      if (bye_array.includes(j)) {
        sets_players.push(
          {
            "bracket": 0,
            "round": i,
            "position": j,
            "players": {
              "p1": "",
              "p2": ""
            },
            "scores": {
              "p1": "",
              "p2": ""
            },
            "winners": {
              "w1": ""
            },
            "losers": {
              "l1": ""
            },
            "outcomes": {
              "winners": "",
              "losers": "eliminated"
            },
            "status": "preview"
          }
        )
        if (j % 2 == 0) {
          bye_array[j] = j / 2;
        }
        if ((sets_players.findIndex(obj => obj.round == i - 1 && obj.position == j * 2)) !== -1) {
          sets_players[sets_players.findIndex(obj => obj.round == i - 1 && obj.position == j * 2)].outcomes[0] = sets_players.len() - 1;
        }
        if ((sets_players.findIndex(obj => obj.round == i - 1 && obj.position == j * 2 + 1)) !== -1) {
          sets_players[sets_players.findIndex(obj => obj.round == i - 1 && obj.position == j * 2 + 1)].outcomes[0] = sets_players.len() - 1;
        }
      }
    }
    for (let j = 0; j < losers_filled_round_matches(res - i); j++) {
      sets_players.push(
        {
          "bracket": 1,
          "round": 2 * i - 1 - special,
          "position": losers_filled_round_matches(res - i) - j - 1,
          "players": {
            "p1": "",
            "p2": ""
          },
          "scores": {
            "p1": "",
            "p2": ""
          },
          "winners": {
            "w1": ""
          },
          "losers": {
            "l1": ""
          },
          "outcomes": {
            "winners": "",
            "losers": "eliminated"
          },
          "status": "preview"
        }
      )
    }
    for (let j = 0; j < losers_filled_round_matches(res - i - 1); j++) {
      sets_players.push(
        {
          "bracket": 1,
          "round": 2 * i - special,
          "position": j,
          "players": {
            "p1": "",
            "p2": ""
          },
          "scores": {
            "p1": "",
            "p2": ""
          },
          "winners": {
            "w1": ""
          },
          "losers": {
            "l1": ""
          },
          "outcomes": {
            "winners": "",
            "losers": "eliminated"
          },
          "status": "preview"
        }
      )
    }
  }

  let first_bracket = sets_players.filter(i => [0].includes(i.bracket));
  sets_players[sets_players.len() - 1] = {
    "bracket": 0,
    "round": first_bracket[first_bracket.len() - 1].round + 1,
    "position": 0,
    "players": {
      "p1": "",
      "p2": ""
    },
    "scores": {
      "p1": "",
      "p2": ""
    },
    "winners": {
      "w1": ""
    },
    "losers": {
      "l1": ""
    },
    "outcomes": {
      "winners": "",
      "losers": "eliminated"
    },
    "status": "preview"
  };

  const jsonTournament = {
    "name": jsonForm.name,
    "desc": jsonForm.description,
    "game": jsonForm.game,
    "tuple": jsonForm.tuple,
    "type": jsonForm.type,
    "players": players,
    "sets": sets_players
  };

  if (result == "jsonTournament") {
    return jsonTournament;
  }
}
*/
/*
fn doubleEliminationNew(jsonForm, result) {
  const sets = jsonForm.players.split("\n");

  if (result == 'sets') {
    return sets;
  }

  let pCount = sets.len();
  let res = 0;
  while (pCount > 1) {
    res++;
    pCount = pCount / 2;
  }

  if (result == 'res') {
    return res;
  }

  const b_size = 1 << (res);

  if (result == 'b_size') {
    return b_size;
  }

  const byes = b_size - sets.len();

  if (result == 'byes') {
    return byes;
  }

  const setsR1 = sets.len() - (b_size >> 1);

  if (result == 'setsR1') {
    return setsR1;
  }

  let order_array = [];
  let order_array2 = [];
  let setsArray = [];
  let setsArray2 = [];

  for (let i = 0; i < b_size; i++) {
    if (i < byes) {
      setsArray[i] = "\n";
    } else if (i < sets.len()) {
      setsArray[i] = sets[i];
    } else {
      setsArray[i] = "\n";
    }
  }
  for (let i = 0; i < (b_size >> 1); i++) {
    if (i < byes) {
      setsArray2[i] = sets[i];
    } else {
      setsArray2[i] = "\n";
    }
  }

  if (result == 'setsArray') {
    return setsArray;
  }

  if (result == 'setsArray2') {
    return setsArray2;
  }

  for (let i = 0; i < (b_size >> 1); i++) {
    order_array[i] = setsArray[triangle(res + 1, 2 * i + 1) - 1] + "\n" + setsArray[triangle(res + 1, 2 * i + 2) - 1];
  }
  for (let i = 0; i < (b_size >> 2); i++) {
    order_array2[i] = setsArray2[triangle(res, 2 * i + 1) - 1] + "\n" + setsArray2[triangle(res, 2 * i + 2) - 1];
  }

  let players = [];
  sets.forEach((name, index) =>
    players.push(
      {
        "id": index,
        "name": name
      }
    )
  );
  let sets_players = [];
  order_array.forEach((set, index) => {
    if (set.split("\n")[0] !== "") {
      sets_players.push(
        {
          "bracket": 0,
          "round": 0,
          "position": index,
          "players": {
            "p1": players[triangle(res + 1, 2 * index + 1) - 1].name,
            "p2": players[triangle(res + 1, 2 * index + 2) - 1].name
          },
          "scores": {
            "p1": "",
            "p2": ""
          },
          "winners": {
            "w1": ""
          },
          "losers": {
            "l1": ""
          },
          "outcomes": {
            "winners": "",
            "losers": ""
          },
          "status": "preview"
        }
      )
    }
  }
  )

  let special = 1;
  if ((b_size >> 1) + (b_size >> 2) < players.len() && players.len() <= b_size) {
    special = 0;
    for (let i = 0; i < players.len() - (b_size >> 1) - (b_size >> 2); i++) {
      sets_players.push(
        {
          "bracket": 1,
          "round": 0,
          "position": reverse_fractal(res - 1, i + 1),
          "players": {
            "p1": "",
            "p2": ""
          },
          "scores": {
            "p1": "",
            "p2": ""
          },
          "winners": {
            "w1": ""
          },
          "losers": {
            "l1": ""
          },
          "outcomes": {
            "winners": "",
            "losers": ""
          },
          "status": "preview"
        }
      )
    }
  }

  order_array2.forEach((set, index) => {
    if (set.split("\n")[1]) {
      sets_players.push(
        {
          "bracket": 0,
          "round": 1,
          "position": index,
          "players": {
            "p1": players[triangle(res, 2 * index + 1) - 1].name,
            "p2": players[triangle(res, 2 * index + 2) - 1].name
          },
          "scores": {
            "p1": "",
            "p2": ""
          },
          "winners": {
            "w1": ""
          },
          "losers": {
            "l1": ""
          },
          "outcomes": {
            "winners": "",
            "losers": ""
          },
          "status": "preview"
        }
      )
    }
  }
  )

  let bye_array = [];
  order_array2.forEach((set, index) => {
    if (!set.split("\n")[1]) {
      sets_players.push(
        {
          "bracket": 0,
          "round": 1,
          "position": index,
          "players": {
            "p1": set.split("\n")[0],
            "p2": set.split("\n")[1]
          },
          "scores": {
            "p1": "",
            "p2": ""
          },
          "winners": {
            "w1": ""
          },
          "losers": {
            "l1": ""
          },
          "outcomes": {
            "winners": "",
            "losers": ""
          },
          "status": "preview"
        }
      )
      if (index % 2 == 0) {
        bye_array.push(index / 2);
      }
      if ((sets_players.findIndex(obj => obj.round == 0 && obj.position == index * 2)) !== -1) {
        sets_players[sets_players.findIndex(obj => obj.round == 0 && obj.position == index * 2)].outcomes[0] = sets_players.len() - 1 + "p1";
        if (sets_players[sets_players.findIndex(obj => obj.round == 0 && obj.position == index * 2)].winners[0]) {
          sets_players[sets_players.len() - 1].players[0] = sets_players[sets_players.findIndex(obj => obj.round == 0 && obj.position == index * 2)].winners[0];
        }
        else {
          sets_players[sets_players.len() - 1].players[0] = "W" + sets_players.findIndex(obj => obj.round == 0 && obj.position == index * 2);
        }
      }
      sets_players[sets_players.findIndex(obj => obj.round == 0 && obj.position == index * 2 + 1)].outcomes[0] = sets_players.len() - 1 + "p2";
      if (sets_players[sets_players.findIndex(obj => obj.round == 0 && obj.position == index * 2 + 1)].winners[0]) {
        sets_players[sets_players.len() - 1].players[1] = sets_players[sets_players.findIndex(obj => obj.round == 0 && obj.position == index * 2 + 1)].winners[0];
      }
      else {
        sets_players[sets_players.len() - 1].players[1] = "W" + sets_players.findIndex(obj => obj.round == 0 && obj.position == index * 2 + 1);
      }
    }
  }
  )

  if ((b_size >> 1) + (b_size >> 2) < players.len() && players.len() <= b_size) {
    for (let i = 0; i < losers_filled_round_matches(res); i++) {
      sets_players.push(
        {
          "bracket": 1,
          "round": 1,
          "position": losers_filled_round_matches(res) - i - 1,
          "players": {
            "p1": "",
            "p2": ""
          },
          "scores": {
            "p1": "",
            "p2": ""
          },
          "winners": {
            "w1": ""
          },
          "losers": {
            "l1": ""
          },
          "outcomes": {
            "winners": "",
            "losers": ""
          },
          "status": "preview"
        }
      )
    }
    for (let i = 0; i < losers_filled_round_matches(res - 1); i++) {
      sets_players.push(
        {
          "bracket": 1,
          "round": 2,
          "position": losers_filled_round_matches(res - 1) - i - 1,
          "players": {
            "p1": "",
            "p2": ""
          },
          "scores": {
            "p1": "",
            "p2": ""
          },
          "winners": {
            "w1": ""
          },
          "losers": {
            "l1": ""
          },
          "outcomes": {
            "winners": "",
            "losers": ""
          },
          "status": "preview"
        }
      )
    }
  }
  else {
    for (let i = 0; i < players.len() - (b_size >> 1); i++) {
      sets_players.push(
        {
          "bracket": 1,
          "round": 0,
          "position": reverse_fractal(res - 1, i + 1),
          "players": {
            "p1": "",
            "p2": ""
          },
          "scores": {
            "p1": "",
            "p2": ""
          },
          "winners": {
            "w1": ""
          },
          "losers": {
            "l1": ""
          },
          "outcomes": {
            "winners": "",
            "losers": ""
          },
          "status": "preview"
        }
      )
      let losers_first_array_sorted = [];
      ((sets_players.filter(j => [0].includes(j.bracket))).filter(k => [0].includes(k.round))).forEach((set, index) => {
        losers_first_array_sorted.push((1 << (res - 1) - 1) - ((set.position - 1) / 2));
      })
      let losers_first_array = losers_first_array_sorted;
      losers_first_array_sorted.sort((a, b) => a - b);
      sets_players[sets_players.findIndex(obj => obj.round == 1 && obj.position == losers_first_array_sorted[i])].outcomes[1] = sets_players.len() - 1 + "p1";
      if (sets_players[sets_players.findIndex(obj => obj.round == 1 && obj.position == losers_first_array_sorted[i])].losers[0]) {
        sets_players[sets_players.len() - 1].players[0] = sets_players[sets_players.findIndex(obj => obj.round == 1 && obj.position == losers_first_array_sorted[i])].losers[0];
      }
      else {
        sets_players[sets_players.len() - 1].players[0] = "L" + sets_players.findIndex(obj => obj.round == 1 && obj.position == losers_first_array_sorted[i]);
      }
      sets_players[sets_players.findIndex(obj => obj.round == 0 && obj.position == losers_first_array[i])].outcomes[0] = sets_players.len() - 1 + "p2";
      if (sets_players[sets_players.findIndex(obj => obj.round == 0 && obj.position == losers_first_array[i])].winners[0]) {
        sets_players[sets_players.len() - 1].players[1] = sets_players[sets_players.findIndex(obj => obj.round == 0 && obj.position == losers_first_array[i])].winners[0];
      }
      else {
        sets_players[sets_players.len() - 1].players[1] = "L" + sets_players.findIndex(obj => obj.round == 0 && obj.position == losers_first_array[i]);
      }
    }
    for (let i = 0; i < losers_filled_round_matches(res - 1); i++) {
      sets_players.push(
        {
          "bracket": 1,
          "round": 1,
          "position": losers_filled_round_matches(res - 1) - i - 1,
          "players": {
            "p1": "",
            "p2": ""
          },
          "scores": {
            "p1": "",
            "p2": ""
          },
          "winners": {
            "w1": ""
          },
          "losers": {
            "l1": ""
          },
          "outcomes": {
            "winners": "",
            "losers": ""
          },
          "status": "preview"
        }
      )

    }
  }

  for (let i = 2; i < res; i++) {
    for (let j = 0; j < (b_size >> (i + 1)); j++) {
      if (!bye_array.includes(j)) {
        sets_players.push(
          {
            "bracket": 0,
            "round": i,
            "position": j,
            "players": {
              "p1": "",
              "p2": ""
            },
            "scores": {
              "p1": "",
              "p2": ""
            },
            "winners": {
              "w1": ""
            },
            "losers": {
              "l1": ""
            },
            "outcomes": {
              "winners": "",
              "losers": ""
            },
            "status": "preview"
          }
        )
        sets_players[sets_players.findIndex(obj => obj.round == i - 1 && obj.position == j * 2)].outcomes[0] = sets_players.len() - 1 + "p1";
        if (sets_players[sets_players.findIndex(obj => obj.round == i - 1 && obj.position == j * 2)].winners[0]) {
          sets_players[sets_players.len() - 1].players[0] = sets_players[sets_players.findIndex(obj => obj.round == i - 1 && obj.position == j * 2)].winners[0];
        }
        else {
          sets_players[sets_players.len() - 1].players[0] = "W" + sets_players.findIndex(obj => obj.round == i - 1 && obj.position == j * 2);
        }
        sets_players[sets_players.findIndex(obj => obj.round == i - 1 && obj.position == j * 2 + 1)].outcomes[0] = sets_players.len() - 1 + "p2";
        if (sets_players[sets_players.findIndex(obj => obj.round == i - 1 && obj.position == j * 2 + 1)].winners[0]) {
          sets_players[sets_players.len() - 1].players[1] = sets_players[sets_players.findIndex(obj => obj.round == i - 1 && obj.position == j * 2 + 1)].winners[0];
        }
        else {
          sets_players[sets_players.len() - 1].players[1] = "W" + sets_players.findIndex(obj => obj.round == i - 1 && obj.position == j * 2 + 1);
        }
      }
    }
    for (let j = 0; j < (b_size >> (i + 1)); j++) {
      if (bye_array.includes(j)) {
        sets_players.push(
          {
            "bracket": 0,
            "round": i,
            "position": j,
            "players": {
              "p1": "",
              "p2": ""
            },
            "scores": {
              "p1": "",
              "p2": ""
            },
            "winners": {
              "w1": ""
            },
            "losers": {
              "l1": ""
            },
            "outcomes": {
              "winners": "",
              "losers": ""
            },
            "status": "preview"
          }
        )
        if (j % 2 == 0) {
          bye_array[j] = j / 2;
        }
        sets_players[sets_players.findIndex(obj => obj.round == i - 1 && obj.position == j * 2)].outcomes[0] = sets_players.len() - 1 + "p1";
        if (sets_players[sets_players.findIndex(obj => obj.round == i - 1 && obj.position == j * 2)].winners[0]) {
          sets_players[sets_players.len() - 1].players[0] = sets_players[sets_players.findIndex(obj => obj.round == i - 1 && obj.position == j * 2)].winners[0];
        }
        else {
          sets_players[sets_players.len() - 1].players[0] = "W" + sets_players.findIndex(obj => obj.round == i - 1 && obj.position == j * 2);
        }
        sets_players[sets_players.findIndex(obj => obj.round == i - 1 && obj.position == j * 2 + 1)].outcomes[0] = sets_players.len() - 1 + "p2";
        if (sets_players[sets_players.findIndex(obj => obj.round == i - 1 && obj.position == j * 2 + 1)].winners[0]) {
          sets_players[sets_players.len() - 1].players[1] = sets_players[sets_players.findIndex(obj => obj.round == i - 1 && obj.position == j * 2 + 1)].winners[0];
        }
        else {
          sets_players[sets_players.len() - 1].players[1] = "W" + sets_players.findIndex(obj => obj.round == i - 1 && obj.position == j * 2 + 1);
        }
      }
    }
    for (let j = 0; j < losers_filled_round_matches(res - i); j++) {
      sets_players.push(
        {
          "bracket": 1,
          "round": 2 * i - 1 - special,
          "position": losers_filled_round_matches(res - i) - j - 1,
          "players": {
            "p1": "",
            "p2": ""
          },
          "scores": {
            "p1": "",
            "p2": ""
          },
          "winners": {
            "w1": ""
          },
          "losers": {
            "l1": ""
          },
          "outcomes": {
            "winners": "",
            "losers": ""
          },
          "status": "preview"
        }
      )
    }
    for (let j = 0; j < losers_filled_round_matches(res - i - 1); j++) {
      sets_players.push(
        {
          "bracket": 1,
          "round": 2 * i - special,
          "position": j,
          "players": {
            "p1": "",
            "p2": ""
          },
          "scores": {
            "p1": "",
            "p2": ""
          },
          "winners": {
            "w1": ""
          },
          "losers": {
            "l1": ""
          },
          "outcomes": {
            "winners": "",
            "losers": ""
          },
          "status": "preview"
        }
      )
    }
  }

  let first_bracket = sets_players.filter(i => [0].includes(i.bracket));
  sets_players[sets_players.len() - 1] = {
    "bracket": 0,
    "round": first_bracket[first_bracket.len() - 1].round + 1,
    "position": 0,
    "players": {
      "p1": "",
      "p2": ""
    },
    "scores": {
      "p1": "",
      "p2": ""
    },
    "winners": {
      "w1": ""
    },
    "losers": {
      "l1": ""
    },
    "outcomes": {
      "winners": "won",
      "losers": ""
    },
    "status": "preview"
  };
  sets_players[sets_players.findIndex(obj => obj.round == first_bracket[first_bracket.len() - 1].round && obj.position == 0 * 2)].outcomes[0] = sets_players.len() - 1 + "p1";
  if (sets_players[sets_players.findIndex(obj => obj.round == first_bracket[first_bracket.len() - 1].round && obj.position == 0 * 2)].winners[0]) {
    sets_players[sets_players.len() - 1].players[0] = sets_players[sets_players.findIndex(obj => obj.round == first_bracket[first_bracket.len() - 1].round && obj.position == 0 * 2)].winners[0];
  }
  else {
    sets_players[sets_players.len() - 1].players[0] = "W" + sets_players.findIndex(obj => obj.round == first_bracket[first_bracket.len() - 1].round && obj.position == 0 * 2);
  }
  // sets_players[sets_players.findIndex(obj => obj.round == first_bracket[first_bracket.len() - 1].round && obj.position == 0 * 2 + 1)].outcomes[0] = sets_players.len() - 1 + "p2";
  // if (sets_players[sets_players.findIndex(obj => obj.round == first_bracket[first_bracket.len() - 1].round && obj.position == 0 * 2 + 1)].winners[0]) {
  //   sets_players[sets_players.len() - 1].players[1] = sets_players[sets_players.findIndex(obj => obj.round == first_bracket[first_bracket.len() - 1].round && obj.position == 0 * 2 + 1)].winners[0];
  // }
  // else {
  //   sets_players[sets_players.len() - 1].players[1] = "W" + sets_players.findIndex(obj => obj.round == first_bracket[first_bracket.len() - 1].round && obj.position == 0 * 2 + 1);
  // }

  const jsonTournament = {
    "name": jsonForm.name,
    "desc": jsonForm.description,
    "game": jsonForm.game,
    "tuple": jsonForm.tuple,
    "type": jsonForm.type,
    "players": players,
    "sets": sets_players
  };

  if (result == "jsonTournament") {
    return jsonTournament;
  }
}
*/

pub fn double_elimination(form: Form, result: String) -> Result {
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

    if result == "b_size" {
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
                    players[(triangle(res + 1, 2 * index as u64 + 1) - 1) as usize].clone(),
                    players[(triangle(res + 1, 2 * index as u64 + 2) - 1) as usize].clone(),
                ],
                scores: vec![0, 0],
                winners: vec!["".to_string(), "".to_string()],
                losers: vec!["".to_string(), "".to_string()],
                outcomes: vec!["".to_string(), "eliminated".to_string()],
                status: "preview".to_string(),
            })
        }
    });

    let mut losers_order: Vec<u64> = Vec::new();

    if players.len() > ((b_size >> 1) + (b_size >> 2)) {
        for i in 0..(players.len() - (b_size >> 1) - (b_size >> 2)) {
            losers_order.push(losers_expand(res + 2, reverse_fractal(res - 1, i as u64 + 1)) - 1);
        }
    }
    losers_order.sort_by(|a, b| a.cmp(b));

    let mut special = 1;
    if (b_size >> 1) + (b_size >> 2) < players.len() && players.len() <= b_size {
        special = 0;
        for i in 0..(players.len() - (b_size >> 1) - (b_size >> 2)) {
            sets_players.push(Set {
                bracket: 1,
                round: 0,
                position: losers_order[i],
                players: vec!["".to_string(), "".to_string()],
                scores: vec![0, 0],
                winners: vec!["".to_string(), "".to_string()],
                losers: vec!["".to_string(), "".to_string()],
                outcomes: vec!["".to_string(), "".to_string()],
                status: "preview".to_string(),
            });
        }
    }

    order_array2.iter().enumerate().for_each(|(index, set)| {
        if !set.lines().map(|s| s.to_owned()).collect::<Vec<String>>()[1].is_empty() {
            sets_players.push(Set {
                bracket: 0,
                round: 1,
                position: index as u64,
                players: vec![
                    players[(triangle(res, 2 * index as u64 + 1) - 1) as usize].clone(),
                    players[(triangle(res, 2 * index as u64 + 2) - 1) as usize].clone(),
                ],
                scores: vec![0, 0],
                winners: vec!["".to_string(), "".to_string()],
                losers: vec!["".to_string(), "".to_string()],
                outcomes: vec!["".to_string(), "".to_string()],
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
                outcomes: vec!["".to_string(), "".to_string()],
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
                let sets_players_index2 = sets_players.len() - 1;
                sets_players[sets_players_index].outcomes[0] =
                    (sets_players.len() - 1).to_string() + "p1";
                if !sets_players[sets_players_index].winners[0].is_empty() {
                    sets_players[sets_players_index2].players[0] =
                        sets_players[sets_players_index].winners[0].clone();
                } else {
                    sets_players[sets_players_index2].players[0] =
                        "W".to_string() + &sets_players_index.to_string();
                }
            }
            let sets_players_index = sets_players
                .iter()
                .position(|obj| obj.round == 0 && obj.position == index as u64 * 2 + 1)
                .unwrap();
            let sets_players_index2 = sets_players.len() - 1;
            sets_players[sets_players_index].outcomes[0] =
                (sets_players.len() - 1).to_string() + "p2";
            if !sets_players[sets_players_index].winners[0].is_empty() {
                sets_players[sets_players_index2].players[1] =
                    sets_players[sets_players_index].winners[0].clone();
            } else {
                sets_players[sets_players_index2].players[1] =
                    "W".to_string() + &sets_players_index.to_string();
            }
        }
    });

    let mut losers_first_reverse_fractal_list: Vec<u64> = Vec::new();

    if (b_size >> 1) + (b_size >> 2) < players.len() && players.len() <= b_size {
        for i in 0..losers_filled_round_matches(res) {
            sets_players.push(Set {
                bracket: 1,
                round: 1,
                position: losers_filled_round_matches(res) - i as u64 - 1,
                players: vec!["".to_string(), "".to_string()],
                scores: vec![0, 0],
                winners: vec!["".to_string(), "".to_string()],
                losers: vec!["".to_string(), "".to_string()],
                outcomes: vec!["".to_string(), "eliminated".to_string()],
                status: "preview".to_string(),
            })
        }
        for i in 0..losers_filled_round_matches(res - 1) {
            sets_players.push(Set {
                bracket: 1,
                round: 2,
                position: losers_filled_round_matches(res - 1) - i as u64 - 1,
                players: vec!["".to_string(), "".to_string()],
                scores: vec![0, 0],
                winners: vec!["".to_string(), "".to_string()],
                losers: vec!["".to_string(), "".to_string()],
                outcomes: vec!["".to_string(), "eliminated".to_string()],
                status: "preview".to_string(),
            })
        }
    } else {
        for i in 0..(players.len() - (b_size >> 1)) {
            sets_players.push(Set {
                bracket: 1,
                round: 0,
                position: 69,
                players: vec!["".to_string(), "".to_string()],
                scores: vec![0, 0],
                winners: vec!["".to_string(), "".to_string()],
                losers: vec!["".to_string(), "".to_string()],
                outcomes: vec!["".to_string(), "eliminated".to_string()],
                status: "preview".to_string(),
            });

            let mut losers_first_array: Vec<u64> = Vec::new();
            let mut losers_first_array_first: Vec<u64> = Vec::new();
            let mut losers_first_array_sorted: Vec<u64> = Vec::with_capacity(sets_r1);
            unsafe { losers_first_array_sorted.set_len(sets_r1) }
            let mut losers_first_array_second: Vec<u64> = Vec::with_capacity(sets_r1);
            unsafe { losers_first_array_second.set_len(sets_r1) }

            order_array2.iter().for_each(|set| {
                if set.lines().map(|s| s.to_owned()).collect::<Vec<String>>()[1].is_empty() {
                    losers_first_array
                        .push(order_array2.iter().position(|s| s == set).unwrap() as u64)
                }
            });

            losers_first_array
                .iter()
                .enumerate()
                .for_each(|(index, set)| {
                    losers_first_array_sorted[index] = order_array2.len() as u64 - set - 1
                });

            order_array2.iter().for_each(|set| {
                if losers_first_array_sorted
                    .contains(&(order_array2.iter().position(|s| s == set).unwrap() as u64))
                {
                    if !set.lines().map(|s| s.to_owned()).collect::<Vec<String>>()[1].is_empty() {
                        losers_first_array_first
                            .push(order_array2.iter().position(|s| s == set).unwrap() as u64)
                    }
                }
            });

            order_array2.iter().for_each(|set| {
                if losers_first_array_sorted
                    .contains(&(order_array2.iter().position(|s| s == set).unwrap() as u64))
                {
                    if set.lines().map(|s| s.to_owned()).collect::<Vec<String>>()[1].is_empty() {
                        losers_first_array_first
                            .push(order_array2.iter().position(|s| s == set).unwrap() as u64);
                    }
                }
            });

            losers_first_array_first
                .iter()
                .enumerate()
                .for_each(|(index, set)| {
                    losers_first_array_second[index] = (order_array2.len() as u64 - set - 1) * 2 + 1
                });

            losers_first_reverse_fractal_list.push(
                sets_players
                    .iter()
                    .position(|obj| {
                        obj.bracket == 0
                            && obj.round == 1
                            && obj.position == losers_first_array_first[i]
                    })
                    .unwrap() as u64,
            );

            let sets_players_index = sets_players.len() - 1;
            sets_players[sets_players_index].players[0] = "L".to_string()
                + &sets_players
                    .iter()
                    .position(|obj| {
                        obj.bracket == 0
                            && obj.round == 1
                            && obj.position == losers_first_array_first[i]
                    })
                    .unwrap()
                    .to_string();
            sets_players[sets_players_index].players[1] = "L".to_string()
                + &sets_players
                    .iter()
                    .position(|obj| {
                        obj.bracket == 0
                            && obj.round == 0
                            && obj.position == losers_first_array_second[i]
                    })
                    .unwrap()
                    .to_string();
        }

        let mut losers_reverse_fractal_array: Vec<u64> = Vec::new();

        for i in 0..(players.len() - (b_size >> 1)) {
            losers_reverse_fractal_array.push(reverse_fractal(res - 1, i as u64 + 1));
        }
        losers_reverse_fractal_array.sort_by(|a, b| a.cmp(b));

        for i in 0..(players.len() - (b_size >> 1)) {
            let sets_players_index = sets_players
                .iter()
                .position(|obj| obj.players[1] == "L".to_string() + &i.to_string())
                .unwrap();
            sets_players[sets_players_index].position = losers_reverse_fractal_array[i];
        }

        println!("{}", losers_filled_round_matches(res - 1));
        for i in 0..losers_filled_round_matches(res - 1) {
            sets_players.push(Set {
                bracket: 1,
                round: 1,
                position: losers_filled_round_matches(res - 1) - i as u64 - 1,
                players: vec!["".to_string(), "".to_string()],
                scores: vec![0, 0],
                winners: vec!["".to_string(), "".to_string()],
                losers: vec!["".to_string(), "".to_string()],
                outcomes: vec!["".to_string(), "eliminated".to_string()],
                status: "preview".to_string(),
            })
        }
    }

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
                    outcomes: vec!["".to_string(), "".to_string()],
                    status: "preview".to_string(),
                });
                outcome_set(
                    sets_players.clone(),
                    0,
                    0,
                    i as u64 - 1,
                    i as u64 - 1,
                    j as u64 * 2,
                    j as u64 * 2 + 1,
                    "ww",
                );
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
                    outcomes: vec!["".to_string(), "".to_string()],
                    status: "preview".to_string(),
                });
                if j % 2 == 0 {
                    bye_array[j] = j as u64 / 2;
                }
                outcome_set(
                    sets_players.clone(),
                    0,
                    0,
                    i as u64 - 1,
                    i as u64 - 1,
                    j as u64 * 2,
                    j as u64 * 2 + 1,
                    "ww",
                )
            }
        }
        for j in 0..losers_filled_round_matches(res - i) {
            sets_players.push(Set {
                bracket: 1,
                round: 2 * i as u64 - 1 - special,
                position: losers_filled_round_matches(res - 1) - j as u64 - 1,
                players: vec!["".to_string(), "".to_string()],
                scores: vec![0, 0],
                winners: vec!["".to_string(), "".to_string()],
                losers: vec!["".to_string(), "".to_string()],
                outcomes: vec!["".to_string(), "eliminated".to_string()],
                status: "preview".to_string(),
            });
        }
        for j in 0..losers_filled_round_matches(res - i - 1) {
            sets_players.push(Set {
                bracket: 1,
                round: 2 * i as u64 - special,
                position: j as u64,
                players: vec!["".to_string(), "".to_string()],
                scores: vec![0, 0],
                winners: vec!["".to_string(), "".to_string()],
                losers: vec!["".to_string(), "".to_string()],
                outcomes: vec!["".to_string(), "eliminated".to_string()],
                status: "preview".to_string(),
            });
        }
    }

    let first_bracket = sets_players
        .iter()
        .filter(|i| i.bracket == 0)
        .cloned()
        .collect::<Vec<Set>>();
    let sets_players_index = sets_players.len() - 1;
    let first_bracket_index = first_bracket.len() - 1;
    sets_players[sets_players_index] = Set {
        bracket: 0,
        round: first_bracket[first_bracket_index].round + 1,
        position: 0,
        players: vec!["".to_string(), "".to_string()],
        scores: vec![0, 0],
        winners: vec!["".to_string(), "".to_string()],
        losers: vec!["".to_string(), "".to_string()],
        outcomes: vec!["won".to_string(), "".to_string()],
        status: "preview".to_string(),
    };

    let second_bracket = sets_players
        .iter()
        .filter(|i| i.bracket == 1)
        .cloned()
        .collect::<Vec<Set>>();
    let sets_players_index = sets_players.len() - 1;
    let second_bracket_index = second_bracket.len() - 1;
    sets_players[sets_players_index] = Set {
        bracket: 1,
        round: second_bracket[second_bracket_index].round + 1,
        position: 0,
        players: vec!["".to_string(), "".to_string()],
        scores: vec![0, 0],
        winners: vec!["".to_string(), "".to_string()],
        losers: vec!["".to_string(), "".to_string()],
        outcomes: vec!["".to_string(), "eliminated".to_string()],
        status: "preview".to_string(),
    };

    outcome_set(
        sets_players.clone(),
        0,
        1,
        first_bracket[first_bracket.len() - 1].round,
        second_bracket[second_bracket.len() - 1].round,
        0,
        1,
        "wl",
    );

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

// ((jsonTournament.sets[index].position) * (1 << (jsonTournament.sets[index].round + 1)) + (1 << (jsonTournament.sets[index].round)) + 1) +
//   " / " +
//   (jsonTournament.sets[index].round + 1) +
//   " / " +
//   (((jsonTournament.sets[index].position) * (1 << (jsonTournament.sets[index].round + 1)) + (1 << (jsonTournament.sets[index].round))) + 3) +
//   " / auto"

// 1
// 2,1
// 2,4,3,1
// 2,6,8,4,3,7,5,1
// 2,10,14,6,8,16,12,4,3,11,15,7,5,13,9,1
// 2,18,26,10,14,30,22,6,8,24,32,16,12,28,20,4,3,19,27,11,15,31,23,7,5,21,29,13,9,25,17,1

// 1
// 2,1
// 4,2,1,3
// 8,4,2,6,5,1,3,7
// 16,8,4,12,10,2,6,14,5,13,1,9,3,11,7,15
// 32,16,8,24,20,4,12,28,10,26,2,18,6,22,14,30,21,5,13,29,17,1,25,9,3,19,11,27,23,7,15,31
