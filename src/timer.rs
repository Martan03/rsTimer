use std::time::{Duration, Instant};

use crossterm::event::{poll, read, Event, KeyCode, KeyEvent};
use termint::{
    enums::Color,
    geometry::Constraint,
    style::Style,
    widgets::{Block, Border, Layout, List, Spacer},
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
            KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('K') => {
                {
                    let mut state = self.stats_state.borrow_mut();
                    if let Some(sel) = state.selected {
                        state.selected = Some(sel.saturating_sub(1));
                    }
                }
                self.term.rerender()?;
                return Ok(());
            }
            KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('J') => {
                {
                    let mut state = self.stats_state.borrow_mut();
                    if let Some(sel) = state.selected {
                        if sel + 1
                            < self
                                .stats
                                .get_session(self.session.as_ref().unwrap())
                                .unwrap()
                                .stats
                                .len()
                        {
                            state.selected = Some(sel + 1);
                        }
                    }
                }
                self.term.rerender()?;
                return Ok(());
            }
            KeyCode::Delete => {
                if let Some(sel) = self.stats_state.borrow().selected {
                    self.stats.remove(sel, self.session.as_ref().unwrap());
                }
                return self.render_timer();
            }
            KeyCode::Char('s') | KeyCode::Char('S') => {
                self.screen = Screen::Sessions
            }
            KeyCode::Char('n') | KeyCode::Char('N') => {
                if let Some(scramble) = &mut self.scramble {
                    scramble.generate();
                }
            }
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
        let mut slayout = Layout::horizontal().center();
        slayout.add_child(scramble, Constraint::Min(0));

        let mut timer = Layout::vertical();
        timer.add_child(slayout, Constraint::Min(1));
        timer.add_child(Spacer::new(), Constraint::Fill);
        let (time, height) = time_layout(time, 3, &self.config.font);
        timer.add_child(time, Constraint::Length(height));
        timer.add_child(Spacer::new(), Constraint::Fill);

        let mut layout = Layout::horizontal();
        layout.add_child(self.timer_stats(), Constraint::Length(17));
        layout.add_child(timer, Constraint::Fill);

        let mut main = Layout::vertical();
        main.add_child(layout, Constraint::Fill);
        main.add_child(self.timer_help(), Constraint::Length(1));

        self.term.render(main)?;
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
                self.session.as_ref().unwrap(),
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

        Ok(!matches!(code, KeyCode::Char(' ')))
    }

    fn timer_stats(&mut self) -> Block {
        let name = self.session.clone().unwrap_or("".to_string());
        let mut block = Block::vertical().title(name.as_str());

        let stats: Vec<String> = self.stats.sessions
            [self.session.as_ref().unwrap()]
        .stats
        .iter()
        .map(|i| format!("{:.3}", i.time.as_secs_f64()))
        .collect();

        if stats.is_empty() {
            block.add_child("No times set yet...", Constraint::Fill);
        } else {
            block.add_child(
                format!("Solves: {}", stats.len()),
                Constraint::Min(0),
            );
            block.add_child(
                format!(
                    "Mean: {:.3}",
                    self.stats
                        .avg(self.session.as_ref().unwrap())
                        .unwrap_or(Duration::from_secs(0))
                        .as_secs_f64()
                ),
                Constraint::Min(0),
            );
            block.add_child(
                Block::vertical().borders(Border::BOTTOM),
                Constraint::Length(1),
            );
            block.add_child(
                List::new(stats, self.stats_state.clone())
                    .selected_style(Style::new().fg(Color::Cyan))
                    .auto_scroll(),
                Constraint::Fill,
            );
        }
        block
    }

    fn timer_help(&self) -> Layout {
        let mut layout = Layout::horizontal();
        layout.add_child(
            RawSpan::new("[Space]Start ").fg(Color::Gray),
            Constraint::Min(0),
        );
        layout.add_child(
            RawSpan::new("[s]Sessions ").fg(Color::Gray),
            Constraint::Min(0),
        );
        layout.add_child(
            RawSpan::new("[↑|k]Next stat ").fg(Color::Gray),
            Constraint::Min(0),
        );
        layout.add_child(
            RawSpan::new("[↓/j]Prev. stat ").fg(Color::Gray),
            Constraint::Min(0),
        );
        layout.add_child(
            RawSpan::new("[Del]Delete stat ").fg(Color::Gray),
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
