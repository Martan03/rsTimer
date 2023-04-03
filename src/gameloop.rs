use std::time::Duration;

use crate::timer::Timer;

use crossterm::{
    event::{poll, read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
};

use self::num_parser::{get_time, print_time};

#[path = "num_parser.rs"]
mod num_parser;

pub struct Gamedata {
    timer: Timer,
    con: bool,
}

impl Gamedata {
    pub fn new() -> Gamedata {
        Gamedata {
            timer: Timer::new(3),
            con: false,
        }
    }

    pub fn start_game(&mut self) -> Result<()> {
        enable_raw_mode()?;

        self.con = true;
        print_time(get_time(0.0, 3));

        while self.con {
            if poll(Duration::from_millis(100))? {
                self.key_listener()?;
            }
        }

        disable_raw_mode()
    }

    fn key_listener(&mut self) -> Result<()> {
        let event = read()?;

        if event == Event::Key(KeyCode::Char(' ').into()) {
            self.timer.start_timer()?;
        }

        if event == Event::Key(KeyCode::Esc.into()) {
            self.con = false;
        }
        Ok(())
    }
}
