pub mod double_elim;
pub mod single_elim;
pub mod single_elim_copy;

pub enum Result {
    Sets(Vec<String>),
    Res(u64),
    BSize(u64),
    Byes(u64),
    SetsR1(u64),
    SetsArray(Vec<String>),
    SetsArray2(Vec<String>),
    JsonTournament(Tournament),
}

#[derive(Debug)]
pub struct Tournament {
    pub name: String,
    pub description: String,
    pub game: String,
    pub tuple: u64,
    pub style: String,
    pub players: Vec<String>,
    pub sets: Vec<Set>,
}

#[derive(Clone, Debug)]
pub struct Set {
    pub bracket: u64,
    pub round: u64,
    pub position: u64,
    pub players: Vec<String>,
    pub scores: Vec<u64>,
    pub winners: Vec<String>,
    pub losers: Vec<String>,
    pub outcomes: Vec<String>,
    pub status: String,
}

pub struct Form {
    pub name: String,
    pub description: String,
    pub game: String,
    pub tuple: u64,
    pub players: Vec<String>,
    pub style: String,
}

pub fn triangle(n: u64, k: u64) -> u64 {
    // A208569
    if n == 1 && k == 1 {
        1
    } else if (k % 2) == 1 {
        triangle(n - 1, (k + 1) / 2)
    } else {
        u64::pow(2, n as u32 - 1) + 1 - triangle(n - 1, k / 2)
    }
}

fn fractal(n: u64, k: u64) -> u64 {
    //A088208
    if n == 1 && k == 1 {
        1
    } else if (k % 4) == 0 {
        fractal(n - 1, k / 2)
    } else if (k % 4) == 1 {
        fractal(n - 1, (k - 1) / 2 + 1)
    } else if (k % 4) == 2 {
        fractal(n, k - 1) + u64::pow(2, n as u32 - 2)
    } else {
        fractal(n, k + 1) + u64::pow(2, n as u32 - 2)
    }
}

fn reverse_fractal(n: u64, k: u64) -> u64 {
    //A088208 (reverse)
    if n == 1 && k == 1 {
        1
    } else if n == 2 && k == 1 {
        2
    } else if n == 2 && k == 2 {
        1
    } else if (k % 4) <= 1 {
        reverse_fractal(n - 1, (k - (k % 4)) / 2 + (k % 4))
    } else {
        reverse_fractal(n, k + 2 * (k % 4) - 5) + u64::pow(2, n as u32 - 2)
    }
}

fn losers_fractal(_n: u64, _k: u64) {
    panic!();
}

fn losers_expand(n: u64, k: u64) -> u64 {
    // NONE
    if n == 1 && k == 1 {
        1
    } else if n % 2 == 1 && k % 2 == 0 {
        2 * losers_expand(n - 1, k / 2)
    } else if n % 2 == 1 && k % 2 == 1 {
        2 * losers_expand(n - 1, (k + 1) / 2) - 1
    } else {
        2 * losers_expand(n - 1, k)
    }
}

fn fractal_old(n: u64, k: u64) -> u64 {
    //A088208
    if n == 1 && k == 1 {
        1
    } else if (k % 4) <= 1 {
        fractal(n - 1, (k - (k % 4)) / 2 + (k % 4))
    } else {
        fractal(n, k + 2 * (k % 4) - 5) + u64::pow(2, n as u32 - 2)
    }
}

fn losers_rounds(n: u64) -> u64 {
    // A126236
    ((n as f64).log(2.0).floor() + (2.0 * n as f64 / 3.0).log(2.0).floor()) as u64
}

fn losers_first_round_matches_helper(n: u64) -> u64 {
    // A072376
    if n < 2 {
        n
    } else {
        u64::pow(2, (n as f64 / 2.0).log(2.0).floor() as u32)
    }
}

fn losers_first_round_matches(n: u64) -> u64 {
    // A062050
    let n = n - losers_first_round_matches_helper(n + 2);
    1 + n - u64::pow(2, (n as f64).log(2.0).floor() as u32)
}

fn losers_filled_round_matches(n: u64) -> u64 {
    // A016116
    f64::exp2((n as f64 / 2.0).floor()) as u64
}

fn losers_first_round_pairs(n: u64, k: u64) -> u64 {
    // A304624 (fake)
    if k == 1 {
        u64::pow(2, n as u32 - 1)
    } else if (k % 4) <= 1 {
        losers_first_round_pairs(n - 1, (k - (k % 4)) / 2 + (k % 4))
    } else {
        losers_first_round_pairs(n, k + 2 * (k % 4) - 5) + u64::pow(2, n as u32 - 2)
    }
}

fn losers_first_round_pairs_old(n: u64, k: u64) -> u64 {
    // NONE
    if k == 1 {
        u64::pow(2, n as u32 - 1)
    } else if (k % 4) == 0 {
        losers_first_round_pairs(n, k - 1) + u64::pow(2, n as u32 - 2)
    } else if (k % 4) == 1 {
        losers_first_round_pairs(n, k + 1) + u64::pow(2, n as u32 - 2)
    } else if (k % 4) == 2 {
        losers_first_round_pairs(n - 1, k / 2)
    } else {
        losers_first_round_pairs(n - 1, (k + 1) / 2)
    }
}

fn losers_filled_first_round_matches(n: u64) -> u64 {
    // 2^(A126236(n) - 1)
    u64::pow(
        2,
        ((n as f64).log(2.0).floor() + (2.0 * n as f64 / 3.0).log(2.0).floor()) as u32 - 1,
    )
}

fn losers_semifilled_first_round_matches(n: u64) -> u64 {
    // A306390
    u64::pow(
        2,
        (1.0 + (((n as f64 - 1.0) / 3.0).log(2.0)).floor()) as u32,
    )
}
