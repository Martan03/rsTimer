#[allow(clippy::module_inception)]
mod scramble;

pub use scramble::Scramble;

/// Gets scramble length and scramble moves
///
/// **Parameters:**
/// * `scramble_type` - type of scramble (eg. 3x3x3)
///
/// **Returns:**
/// * [`Scramble`] struct
pub fn get_scramble(scramble_type: &str) -> Scramble {
    match scramble_type {
        "2x2x2" => Scramble::new(9, get_2x2x2()),
        "3x3x3" => Scramble::new(21, get_3x3x3()),
        "4x4x4" => Scramble::new(43, get_4x4x4()),
        _ => Scramble::new(0, vec![vec![]]),
    }
}

/// Gets move groups for 2x2x2 scramble
///
/// **Returns:**
/// * Move groups vector
fn get_2x2x2() -> Vec<Vec<&'static str>> {
    vec![
        vec!["R", "R'", "R2"],
        vec!["U", "U'", "U2"],
        vec!["F", "F'", "F2"],
    ]
}

/// Gets move grouops for 3x3x3 scramble
///
/// **Returns:**
/// * Move groups vector
fn get_3x3x3() -> Vec<Vec<&'static str>> {
    vec![
        vec!["R", "R'", "R2"],
        vec!["L", "L'", "L2"],
        vec!["U", "U'", "U2"],
        vec!["D", "D'", "D2"],
        vec!["F", "F'", "F2"],
        vec!["B", "B'", "B2"],
    ]
}

/// Gets move groups for 4x4x4 scramble
///
/// **Returns:**
/// * Move groups vector
fn get_4x4x4() -> Vec<Vec<&'static str>> {
    vec![
        vec!["R", "R'", "R2", "Rw", "Rw'", "Rw2"],
        vec!["L", "L'", "L2"],
        vec!["U", "U'", "U2", "Uw", "Uw'", "Uw2"],
        vec!["D", "D'", "D2"],
        vec!["F", "F'", "F2", "Fw", "Fw'", "Fw2"],
        vec!["B", "B'", "B2"],
    ]
}
