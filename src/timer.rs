use std::time::{Duration, Instant};

use crossterm::event::{poll, read, Event, KeyCode, KeyEvent};
use termint::{
    enums::Color,
    geometry::{Constraint, TextAlign},
    widgets::{Layout, Spacer, StrSpanExtension},
};

use crate::{
    app::{App, Screen},
    asci::time_layout,
    error::Error,
    stats::stat::Stat,
    widgets::raw_span::RawSpan,
};

/// Idle and running timer implementation
impl App {
    /// Renders timer screen
    pub fn render_timer(&mut self) -> Result<(), Error> {
        let scramble = match &self.scramble {
            Some(s) => s.get().to_owned(),
            None => String::new(),
        };
        self._render_timer(self.time.as_secs_f64(), &scramble)
    }

    /// Listens to pressed keys while showing Timer screen
    pub fn listen_timer(&mut self, code: KeyCode) -> Result<(), Error> {
        match code {
            KeyCode::Right | KeyCode::Char('l') | KeyCode::Char('L') => {
                self.config.set_font(self.config.font.next())?;
            }
            KeyCode::Left | KeyCode::Char('h') | KeyCode::Char('H') => {
                self.config.set_font(self.config.font.prev())?;
            }
            KeyCode::Char('s') | KeyCode::Char('S') => {
                self.screen = Screen::Sessions
            }
            KeyCode::Tab => self.screen = Screen::Stats,
            KeyCode::Char(' ') => self.start_timer()?,
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
                return Err(Error::Exit)
            }
            _ => {}
        }
        self.render()
    }

    /// Helper function for rendering timer
    fn _render_timer(
        &mut self,
        time: f64,
        scramble: &str,
    ) -> Result<(), Error> {
        let mut layout = Layout::vertical();
        layout
            .add_child(scramble.align(TextAlign::Center), Constraint::Min(1));
        layout.add_child(Spacer::new(), Constraint::Fill);

        let (time, height) = time_layout(time, 3, &self.config.font);
        layout.add_child(time, Constraint::Length(height));

        layout.add_child(Spacer::new(), Constraint::Fill);
        layout.add_child(self.timer_help(), Constraint::Length(1));

        self.term.render(layout)?;
        Ok(())
    }

    /// Stats the running timer loop
    fn start_timer(&mut self) -> Result<(), Error> {
        let start = Instant::now();

        let wait_time = Duration::from_secs_f64(0.001);
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
        if let Some(scramble) = &mut self.scramble {
            self.stats.add(
                Stat::new(self.time, scramble.get().to_owned(), String::new()),
                &self.session.as_ref().unwrap(),
            )?;
            self.stats.save()?;
            scramble.generate();
        }
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

    fn timer_help(&self) -> Layout {
        let mut layout = Layout::horizontal();
        layout.add_child(
            RawSpan::new("[Space]Start ").fg(Color::Gray),
            Constraint::Min(0),
        );
        layout.add_child(
            RawSpan::new("[Tab]Stats ").fg(Color::Gray),
            Constraint::Min(0),
        );
        layout.add_child(
            RawSpan::new("[s]Sessions ").fg(Color::Gray),
            Constraint::Min(0),
        );
        layout.add_child(
            RawSpan::new("[→|l]Next font ").fg(Color::Gray),
            Constraint::Min(0),
        );
        layout.add_child(
            RawSpan::new("[←|h]Prev. font ").fg(Color::Gray),
            Constraint::Min(0),
        );
        layout.add_child(
            RawSpan::new("[Esc|q]Quit ").fg(Color::Gray),
            Constraint::Min(0),
        );
        layout
    }
}
