use rand::{thread_rng, Rng};

/// Scramble struct containing valid moves, current scramble and length
/// scramble should have
#[derive(Debug)]
pub struct Scramble {
    moves: Vec<Vec<&'static str>>,
    scramble: String,
    length: usize,
}

impl Scramble {
    /// Creates new [`Scramble`] based on the given type.
    /// Returns scramble with zero length and no valid moves
    pub fn new(scramble_type: &str) -> Scramble {
        match scramble_type {
            "2x2x2" => Scramble::custom(9, Scramble::get_2x2x2()),
            "3x3x3" => Scramble::custom(21, Scramble::get_3x3x3()),
            "4x4x4" => Scramble::custom(43, Scramble::get_4x4x4()),
            _ => Scramble::custom(0, vec![]),
        }
    }

    /// Creates new custom [`Scramble`] with given length and moves
    pub fn custom(len: usize, moves: Vec<Vec<&'static str>>) -> Scramble {
        Scramble {
            moves,
            scramble: "".to_owned(),
            length: len,
        }
    }

    /// Generates new scramble
    pub fn generate(&mut self) {
        self.scramble = "".to_owned();
        let mut last: usize = 0;

        for i in 0..self.length {
            let mut r = thread_rng().gen_range(0..self.moves.len());
            while i > 0 && r == last {
                r = thread_rng().gen_range(0..self.moves.len());
            }
            last = r;

            let c = thread_rng().gen_range(0..self.moves[r].len());

            self.scramble.push_str(self.moves[r][c]);
            self.scramble.push(' ');
        }
        self.scramble = self.scramble.trim().to_owned();
    }

    /// Gets scramble
    pub fn get(&self) -> &str {
        &self.scramble
    }
}

impl Scramble {
    /// Gets move groups for 2x2x2 scramble
    fn get_2x2x2() -> Vec<Vec<&'static str>> {
        vec![
            vec!["R", "R'", "R2"],
            vec!["U", "U'", "U2"],
            vec!["F", "F'", "F2"],
        ]
    }

    /// Gets move groups for 3x3x3 scramble
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
}
