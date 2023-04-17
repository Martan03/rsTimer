use std::time::Duration;

use crate::{
    scramble::Scramble,
    stats::{Stat, Stats},
    timer::Timer,
};

use crossterm::{
    event::{poll, read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use eyre::Result;

use self::num_parser::{get_time, print_time};

#[path = "num_parser.rs"]
mod num_parser;

pub struct Game {
    timer: Timer,
    con: bool,
    scramble: Scramble,
    stats: Stats,
}

impl Game {
    /// Constructs a new Game
    /// * 'len' - length of the scramble
    /// * 'moves' - moves to make scramble off
    pub fn new(len: usize, moves: Vec<Vec<&'static str>>) -> Result<Game> {
        Ok(Game {
            timer: Timer::new(3),
            con: false,
            scramble: Scramble::new(len, moves),
            stats: Stats::load()?,
        })
    }

    /// Starts main game loop
    pub fn start_game(&mut self) -> Result<()> {
        enable_raw_mode()?;

        self.con = true;

        // Generates scramble
        self.scramble.generate();
        self.print_scramble();

        print_time(get_time(0.0, 3));

        // Game loop
        while self.con {
            if poll(Duration::from_millis(100))? {
                self.key_listener()?;
            }
        }

        Ok(disable_raw_mode()?)
    }

    /// Listens to key presses
    fn key_listener(&mut self) -> Result<()> {
        let event = read()?;

        // Starts timer when Space pressed
        if event == Event::Key(KeyCode::Char(' ').into()) {
            self.timer.start_timer()?;
            self.stats.add_stat(Stat::new(
                self.timer.get_time(),
                self.scramble.get().to_owned(),
            ));

            self.scramble.generate();
            self.print_scramble();
        }
        // Ends game loop when ESC pressed
        if event == Event::Key(KeyCode::Esc.into()) {
            self.stats.save()?;
            self.con = false;
        }
        Ok(())
    }

    /// Prints scramble
    fn print_scramble(&mut self) {
        let (w, _) = termion::terminal_size().unwrap();
        let px = (w as usize - self.scramble.get().len()) / 2;

        println!("\x1b[1;{px}H\x1b[0m{}", self.scramble.get());
    }
}
