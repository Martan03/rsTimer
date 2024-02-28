use std::time::Duration;

use crate::{asci::time_layout, stats_manager::StatsManager, timer::Timer};

use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use eyre::{Report, Result};
use termint::{
    geometry::constrain::Constrain,
    term::Term,
    widgets::{
        block::Block, border::BorderType, layout::Layout,
        span::StrSpanExtension,
    },
};

pub struct Game {
    timer: Timer,
    started: bool,
    stats_manager: StatsManager,
}

impl Game {
    /// Constructs a new [`Game`]
    ///
    /// **Parameters:**
    /// * `stats_manager` - struct for stats managing
    ///
    /// **Returns:**
    /// * Constructed [`Game`] in Result
    pub fn new(stats_manager: StatsManager) -> Self {
        Self {
            timer: Timer::new(3),
            started: false,
            stats_manager,
        }
    }

    /// Starts main game loop
    ///
    /// **Returns:**
    /// * Ok() on success, else Err with corresponding error message
    pub fn start_game(&mut self) -> Result<()> {
        enable_raw_mode()?;

        // Generates scramble
        self.stats_manager.scramble.generate();
        self.render()?;

        // Game loop
        self.started = true;
        while self.started {
            if poll(Duration::from_millis(100))? {
                self.key_listener()?;
            }
        }

        Ok(disable_raw_mode()?)
    }
}

// Private methods implementations
impl Game {
    /// Listens to key presses
    fn key_listener(&mut self) -> Result<()> {
        let Event::Key(KeyEvent { code, .. }) = read()? else {
            return Ok(());
        };

        match code {
            // Starts timer when Space pressed
            KeyCode::Char(' ') => {
                self.timer.start_timer(&self.stats_manager.session)?;
                self.stats_manager.add_time(self.timer.get_time())?;

                self.stats_manager.scramble.generate();
                self.render()?;
            }
            // Opens session list
            KeyCode::Char('s') => self.stats_manager.open_session_list(),
            // Displays sesssion stats
            KeyCode::Tab => {
                if self.stats_manager.open_stats()? {
                    self.started = false;
                    self.stats_manager.stats.save()?;
                }
                self.render()?;
            }
            // Closes the game
            KeyCode::Esc => {
                self.stats_manager.stats.save()?;
                self.started = false;
            }
            _ => {}
        }
        Ok(())
    }

    /// Renders the idle timer screen
    fn render(&mut self) -> Result<()> {
        print!("\x1b[H\x1b[J");
        let mut block = Block::new()
            .title(self.stats_manager.session.as_str())
            .border_type(BorderType::Thicker);

        block.add_child(self.scramble_layout(), Constrain::Length(1));
        block.add_child("".to_span(), Constrain::Fill);
        block.add_child(
            time_layout(self.timer.get_time().as_secs_f64(), 3),
            Constrain::Length(5),
        );
        block.add_child("".to_span(), Constrain::Fill);

        let term = Term::new();
        term.render(block).map_err(|e| Report::msg(e))
    }

    /// Gets scramble layout
    fn scramble_layout(&mut self) -> Layout {
        let mut layout = Layout::horizontal();
        layout.add_child("".to_span(), Constrain::Fill);
        layout.add_child(
            self.stats_manager.scramble.get().to_span(),
            Constrain::Min(0),
        );
        layout.add_child("".to_span(), Constrain::Fill);
        layout
    }
}
