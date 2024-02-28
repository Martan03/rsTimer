use crossterm::{
    event::{poll, read, Event, KeyCode},
    Result,
};
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
    /// * Ok() on success, else Err
    pub fn start_timer(&mut self, title: &str) -> Result<()> {
        self.render(title, 0.0);

        let start = Instant::now();
        self.running = true;

        // Timer loop
        while self.running {
            self.key_listener()?;
            self.render(title, start.elapsed().as_secs_f64());
        }

        self.time = start.elapsed();
        self.render(title, start.elapsed().as_secs_f64());

        Ok(())
    }

    fn render(&self, title: &str, time: f64) {
        print!("\x1b[2J");

        let mut block =
            Block::new().title(title).border_type(BorderType::Thicker);
        block.add_child("".to_span(), Constrain::Length(1));
        block.add_child("".to_span(), Constrain::Fill);
        block
            .add_child(time_layout(time, self.decimals), Constrain::Length(5));
        block.add_child("".to_span(), Constrain::Fill);

        let term = Term::new();
        _ = term.render(block);
    }

    /// Listens to key presses and reacts to it
    ///
    /// **Returns:**
    /// * Ok() on success, else Err
    fn key_listener(&mut self) -> Result<()> {
        if poll(Duration::from_millis(1))? {
            let event = read()?;

            if event == Event::Key(KeyCode::Char(' ').into()) {
                self.running = false;
            }
        }

        Ok(())
    }

    pub fn get_time(&self) -> Duration {
        self.time
    }
}
