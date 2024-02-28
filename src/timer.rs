use crossterm::event::{poll, read, Event, KeyCode};
use eyre::{Report, Result};
use std::time::{Duration, Instant};
use termint::{
    geometry::constrain::Constrain,
    term::Term,
    widgets::{block::Block, border::BorderType, span::StrSpanExtension},
};

use crate::asci::time_layout;

pub struct Timer {
    decimals: usize,
    time: Duration,
    running: bool,
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
            running: false,
        }
    }

    /// Starts timer, prints it periodically, stops when space pressed
    ///
    /// **Result:**
    /// * Ok() on success, else Err with corresponding error message
    pub fn start_timer(&mut self, title: &str) -> Result<()> {
        self.render(title, 0.0)?;

        let wait_time = Duration::from_secs_f64(
            1. / 10_usize.pow(self.decimals as u32) as f64,
        );
        let start = Instant::now();

        let mut last = start;
        self.running = true;
        while self.running {
            self.key_listener()?;
            if last.elapsed() >= wait_time {
                self.render(title, start.elapsed().as_secs_f64())?;
                last = Instant::now();
            }
        }

        self.time = start.elapsed();
        self.render(title, start.elapsed().as_secs_f64())?;

        Ok(())
    }

    pub fn get_time(&self) -> Duration {
        self.time
    }
}

// Private methods implementaions
impl Timer {
    /// Renders timer when running
    fn render(&self, title: &str, time: f64) -> Result<()> {
        print!("\x1b[2J");

        let mut block =
            Block::new().title(title).border_type(BorderType::Thicker);
        block.add_child("".to_span(), Constrain::Length(1));
        block.add_child("".to_span(), Constrain::Fill);
        block
            .add_child(time_layout(time, self.decimals), Constrain::Length(5));
        block.add_child("".to_span(), Constrain::Fill);

        let term = Term::new();
        term.render(block).map_err(Report::msg)
    }

    /// Listens to key presses and reacts to it
    fn key_listener(&mut self) -> Result<()> {
        if poll(Duration::from_millis(1))? {
            let event = read()?;

            if event == Event::Key(KeyCode::Char(' ').into()) {
                self.running = false;
            }
        }

        Ok(())
    }
}
