use tournament::generation::*;

fn main() {
    // single_elim::new_rr(
    //     "1, 2, 3, 4, 5, 6, 7, 8"
    //         .split(",")
    //         .map(|p| p.to_owned())
    //         .collect::<Vec<String>>(),
    //     3,
    // );
    elimination::new_elim(
        "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27"
            .split(",")
            .map(|p| p.to_owned())
            .collect::<Vec<String>>(),
        4,
    );
}
