use std::time::Duration;

use crate::{stats_manager::StatsManager, timer::Timer};

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
    stats_manager: StatsManager,
}

impl Game {
    /// Constructs a new Game
    ///
    /// **Parameters:**
    /// * `stats_manager` - struct for stats managing
    ///
    /// **Returns:**
    /// * Constructed Game in Result
    pub fn new(stats_manager: StatsManager) -> Result<Game> {
        Ok(Game {
            timer: Timer::new(3),
            con: false,
            stats_manager,
        })
    }

    /// Starts main game loop
    ///
    /// **Returns:**
    /// * Ok() on success, else Err
    pub fn start_game(&mut self) -> Result<()> {
        enable_raw_mode()?;

        self.con = true;

        // Generates scramble
        self.stats_manager.scramble.generate();
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
    ///
    /// **Returns:**
    /// * Ok() on success, else Err
    fn key_listener(&mut self) -> Result<()> {
        let event = read()?;

        // Starts timer when Space pressed
        if event == Event::Key(KeyCode::Char(' ').into()) {
            self.timer.start_timer()?;
            self.stats_manager.add_time(self.timer.get_time())?;

            self.stats_manager.scramble.generate();
            self.print_scramble();
        }
        if event == Event::Key(KeyCode::Char('s').into()) {
            self.stats_manager.display_sessions();
        }
        // Opens statistics
        if event == Event::Key(KeyCode::Tab.into()) {
            self.stats_manager.display_stats();
        }
        // Ends game loop when ESC pressed
        if event == Event::Key(KeyCode::Esc.into()) {
            self.stats_manager.stats.save()?;
            self.con = false;
        }
        Ok(())
    }

    /// Prints screen (scramble, time)
    fn print_screen(&mut self) {
        print!("\x1b[H\x1b[J");
        self.print_scramble();
        print_time(get_time(self.timer.get_time().as_secs_f64(), 3));
    }

    /// Prints scramble
    fn print_scramble(&mut self) {
        let (w, _) = termion::terminal_size().unwrap();
        let px = (w as usize - self.stats_manager.scramble.get().len()) / 2;

        println!("\x1b[1;{px}H\x1b[0m{}", self.stats_manager.scramble.get());
    }
}
