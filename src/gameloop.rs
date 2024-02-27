use std::time::Duration;

use crate::{
    num_parser::get_time_length, stats_manager::StatsManager, timer::Timer,
};

use crossterm::{
    event::{poll, read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use eyre::Result;
use termint::{
    enums::wrap::Wrap,
    geometry::{constrain::Constrain, direction::Direction},
    term::Term,
    widgets::{
        block::Block, border::BorderType, grad::Grad, layout::Layout,
        span::StrSpanExtension,
    },
};

use crate::num_parser::get_time;

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
        self.print_screen();

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
            //self.print_scramble();
        }
        if event == Event::Key(KeyCode::Char('s').into()) {
            self.stats_manager.open_session_list();
        }
        // Opens statistics
        if event == Event::Key(KeyCode::Tab.into()) {
            if self.stats_manager.open_stats()? {
                self.con = false;
                self.stats_manager.stats.save()?;
            }
            self.print_screen();
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
        let mut block = Block::new()
            .title(self.stats_manager.session.as_str())
            .border_type(BorderType::Thicker);

        block.add_child(self.scramble_layout(), Constrain::Length(1));
        block.add_child("".to_span(), Constrain::Fill);
        let time = get_time(self.timer.get_time().as_secs_f64(), 3);
        block
            .add_child(self.time_layout(&time), Constrain::Length(time.len()));
        block.add_child("".to_span(), Constrain::Fill);

        let term = Term::new();
        _ = term.render(block);
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

    fn time_layout(&mut self, time: &[String]) -> Layout {
        let time_len = get_time_length(time);
        let time_str = time.join("");

        let mut layout = Layout::horizontal();
        layout.add_child("".to_span(), Constrain::Fill);
        layout.add_child(
            Grad::new(time_str, (0, 220, 255), (160, 100, 255))
                .direction(Direction::Vertical)
                .wrap(Wrap::Letter)
                .ellipsis(""),
            Constrain::Length(time_len),
        );
        layout.add_child("".to_span(), Constrain::Fill);
        layout
    }
}
