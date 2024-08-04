use std::time::{Duration, Instant};

use crossterm::event::{poll, read, Event, KeyCode, KeyEvent};
use termint::{
    geometry::{Constraint, TextAlign},
    widgets::{Block, BorderType, Spacer, StrSpanExtension},
};

use crate::{
    app::{App, Screen},
    asci::time_layout,
    error::Error,
};

/// Idle and running timer implementation
impl App {
    /// Renders timer screen
    pub fn render_timer(&mut self) -> Result<(), Error> {
        self._render_timer(self.time.as_secs_f64(), "Scramble")
    }

    /// Listens to pressed keys while showing Timer screen
    pub fn listen_timer(&mut self, code: KeyCode) -> Result<(), Error> {
        match code {
            KeyCode::Tab => self.screen = Screen::Stats,
            KeyCode::Char(' ') => self.start_timer()?,
            KeyCode::Esc | KeyCode::Char('q') => return Err(Error::Exit),
            _ => {}
        }
        self.render()
    }

    fn _render_timer(
        &mut self,
        time: f64,
        scramble: &str,
    ) -> Result<(), Error> {
        let name = self.session.clone().unwrap_or("".to_string());
        let mut block = Block::vertical()
            .title(name.as_str())
            .border_type(BorderType::Thicker);

        block.add_child(scramble.align(TextAlign::Center), Constraint::Min(1));
        block.add_child(Spacer::new(), Constraint::Fill);
        block.add_child(time_layout(time, 3), Constraint::Length(5));
        block.add_child(Spacer::new(), Constraint::Fill);

        self.term.render(block)?;
        Ok(())
    }

    fn start_timer(&mut self) -> Result<(), Error> {
        let start = Instant::now();

        let wait_time = Duration::from_secs_f64(0.1);
        let mut last = start;

        let mut running = true;
        while running {
            if last.elapsed() >= wait_time {
                self._render_timer(start.elapsed().as_secs_f64(), "")?;
                last = Instant::now();
            }
            if poll(Duration::from_millis(1))? {
                running = self.listen_run_timer()?;
            }
        }

        self.time = start.elapsed();
        Ok(())
    }

    fn listen_run_timer(&mut self) -> Result<bool, Error> {
        let Event::Key(KeyEvent { code, .. }) = read()? else {
            return Ok(true);
        };

        Ok(match code {
            KeyCode::Char(' ') => false,
            _ => true,
        })
    }
}
