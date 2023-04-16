use crossterm::{
    event::{poll, read, Event, KeyCode},
    Result,
};
use std::{
    time::{Duration, Instant},
};

use crate::num_parser::{get_time, print_time};

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
        print!("\x1b[2J");
        print_time(get_time(0.0, self.decimals));

        let start = Instant::now();
        self.con = true;

        while self.con {
            self.key_listener()?;
            
            print_time(get_time(start.elapsed().as_secs_f64(), self.decimals));
        }

        self.time = start.elapsed();
        print_time(get_time(start.elapsed().as_secs_f64(), self.decimals));

        Ok(())
    }

    fn key_listener(&mut self) -> Result<()> {
        if poll(Duration::from_millis(1))? {
            let event = read()?;
    
            if event == Event::Key(KeyCode::Char(' ').into()) {
                self.con = false;
            }
        }

        Ok(())
    }
}
