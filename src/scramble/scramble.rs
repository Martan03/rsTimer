use rand::{thread_rng, Rng};

/// Scramble struct containing valid moves, current scramble and length
/// scramble should have
pub struct Scramble {
    moves: Vec<Vec<&'static str>>,
    scramble: String,
    length: usize,
}

impl Scramble {
    /// Constructs a new [`Scramble`]
    ///
    /// **Parameters:**
    /// * `len` - length of the scramble
    /// * `moves` - move groups for scrambler
    ///
    /// **Returns:**
    /// * Created Scramble struct
    pub fn new(len: usize, moves: Vec<Vec<&'static str>>) -> Scramble {
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
    pub fn get(&mut self) -> &str {
        &self.scramble
    }
}
