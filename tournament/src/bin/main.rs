use tournament::generation::*;

fn main() {
    single_elim::new_elim(
        "1, 2, 3, 4, 5, 6, 7, 8, 9, 10"
            .split(",")
            .map(|p| p.to_owned())
            .collect::<Vec<String>>(),
        3,
    );
}
