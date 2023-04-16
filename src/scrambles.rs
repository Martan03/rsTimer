use std::vec;

pub fn get_scramble(scramble_type: &str) -> Vec<Vec<&'static str>> {
    match scramble_type {
        "3x3" => get_3x3(),
        _ => vec![vec![]],
    }
}

fn get_3x3() -> Vec<Vec<&'static str>> {
    vec![
        vec!["R", "R'", "R2"],
        vec!["L", "L'", "L2"],
        vec!["U", "U'", "U2"],
        vec!["D", "D'", "D2"],
        vec!["F", "F'", "F2"],
        vec!["B", "B'", "B2"],
    ]
}
