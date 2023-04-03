use crossterm::{
    event::{poll, read, Event, KeyCode},
    Result,
};
use std::{
    ops::Sub,
    time::{Duration, Instant},
};

use crate::num_parser::{get_time, print_time};

#[path = "num_parser.rs"]
mod num_parser;

pub struct Timer {
    decimals: usize,
    time: Duration,
    con: bool,
}

impl Timer {
    pub fn new(decimals: usize) -> Self {
        Timer {
            decimals,
            time: Duration::new(0, 0),
            con: false,
        }
    }

    pub fn start_timer(&mut self) -> Result<()> {
        let mut last = Duration::new(0, 0);
        let start = Instant::now();
        self.con = true;

        while self.con {
            if poll(Duration::from_millis(100))? {
                self.key_listener()?;
            }
            let time = start.elapsed();
            if (time.sub(last).as_millis() as i128 - self.decimals as i128) < 0
            {
                continue;
            }

            last = time;
            print_time(get_time(time.as_secs_f64(), self.decimals));
        }

        self.time = start.elapsed();
        print_time(get_time(start.elapsed().as_secs_f64(), self.decimals));

        Ok(())
    }

    fn key_listener(&mut self) -> Result<()> {
        let event = read()?;

        if event == Event::Key(KeyCode::Char(' ').into()) {
            self.con = false;
        }

        Ok(())
    }
}
