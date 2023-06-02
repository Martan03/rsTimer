use crossterm::{
    event::{poll, read, Event, KeyCode},
    Result,
};
use std::time::{Duration, Instant};

use crate::num_parser::{get_time, print_time};

pub struct Timer {
    decimals: usize,
    time: Duration,
    con: bool,
}

impl Timer {
    /// Constructs a new Timer
    /// 
    /// **Parameters:**
    /// * `decimals` - how many decimals will be printed
    /// 
    /// **Returns:**
    /// * Created Timer struct
    pub fn new(decimals: usize) -> Self {
        Timer {
            decimals,
            time: Duration::new(0, 0),
            con: false,
        }
    }

    /// Starts timer, prints it periodically, stops when space pressed
    /// 
    /// **Result:**
    /// * Ok() on success, else Err
    pub fn start_timer(&mut self) -> Result<()> {
        // Clears screen and prints
        print!("\x1b[2J");

        print_time(get_time(0.0, self.decimals));

        let start = Instant::now();
        self.con = true;

        // Timer loop
        while self.con {
            self.key_listener()?;

            print_time(get_time(start.elapsed().as_secs_f64(), self.decimals));
        }

        self.time = start.elapsed();
        print_time(get_time(start.elapsed().as_secs_f64(), self.decimals));

        Ok(())
    }

    /// Listens to key presses and reacts to it
    /// 
    /// **Returns:**
    /// * Ok() on success, else Err
    fn key_listener(&mut self) -> Result<()> {
        if poll(Duration::from_millis(1))? {
            let event = read()?;

            if event == Event::Key(KeyCode::Char(' ').into()) {
                self.con = false;
            }
        }

        Ok(())
    }

    pub fn get_time(&self) -> Duration {
        self.time
    }
}
