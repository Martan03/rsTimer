use rand::Rng;

use crate::scrambles::get_scramble;

pub struct Scramble {
    moves: Vec<Vec<&'static str>>,
    scramble: String,
    length: i64,
}

impl Scramble {
    pub fn new(scramble_type: String) -> Scramble {
        Scramble {
            moves: get_scramble(&scramble_type),
            scramble: "".to_owned(),
            length: 21,
        }
    }

    pub fn generate(&mut self) {
        self.scramble = "".to_owned();
        let mut last: usize = 0;
        for i in 0..self.length {
            let mut r = rand::thread_rng().gen_range(0..self.moves.len());
            while i > 0 && r == last {
                r = rand::thread_rng().gen_range(0..self.moves.len());
            }
            last = r;
    
            let c = rand::thread_rng().gen_range(0..self.moves[r].len());

            self.scramble.push_str(&self.moves[r][c]);
            self.scramble.push(' ');
        }
        self.scramble = self.scramble.trim().to_owned();
    }

    pub fn get(&mut self) -> &str {
        return &self.scramble;
    }
}
