use crossterm::event::KeyCode;
use termint::{
    enums::Color,
    geometry::{Constraint, Unit},
    widgets::{
        Block, Grid, Layout, Paragraph, Spacer, StrSpanExtension, Widget,
    },
};

use crate::{
    app::{App, Screen},
    error::Error,
};

impl App {
    /// Renders the solve screen
    pub fn render_solve(&mut self) -> Result<(), Error> {
        let mut layout = Layout::horizontal();
        layout.add_child(self.timer_stats(), Constraint::Length(17));
        layout.add_child(self.solve_details(), Constraint::Fill);

        let mut main = Layout::vertical();
        main.add_child(layout, Constraint::Fill);
        main.add_child(App::solve_help(), Constraint::Min(0));

        self.term.render(main)?;
        Ok(())
    }

    /// Listens to pressed keys while showing Solve screen
    pub fn listen_solve(&mut self, code: KeyCode) -> Result<(), Error> {
        match code {
            KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('K') => {
                {
                    let mut state = self.stats_state.borrow_mut();
                    if let Some(sel) = state.selected {
                        state.selected = Some(sel.saturating_sub(1));
                    }
                }
                self.render_solve()?;
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
                self.render_solve()?;
            }
            KeyCode::Tab => {
                self.screen = Screen::Timer;
                self.render()?;
            }
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
                return Err(Error::Exit)
            }
            _ => {}
        }
        Ok(())
    }
}

impl App {
    fn solve_details(&self) -> Block {
        let mut details = Grid::new(
            vec![Unit::Length(6), Unit::Fill(1)],
            vec![Unit::Length(1); 3],
        );

        let stat = &self.stats.sessions[self.session.as_ref().unwrap()].stats
            [self.stats_state.borrow().selected.unwrap()];

        details.add_child("ID:".fg(Color::Indexed(244)), 0, 0);
        details.add_child(
            (self.stats.sessions[self.session.as_ref().unwrap()]
                .stats
                .len()
                - self.stats_state.borrow().selected.unwrap())
            .to_string(),
            1,
            0,
        );

        details.add_child("Time:".fg(Color::Indexed(244)), 0, 1);
        details.add_child(
            format!("{:.3}", stat.time.as_secs_f64()).fg(Color::Cyan),
            1,
            1,
        );

        details.add_child("Date:".fg(Color::Indexed(244)), 0, 2);
        details.add_child(
            stat.datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
            1,
            2,
        );

        let mut block = Block::vertical();
        block.add_child(
            Box::new(details) as Box<dyn Widget>,
            Constraint::Min(0),
        );
        block.add_child(Spacer::new(), Constraint::Length(1));
        block.add_child(stat.scramble.fg(Color::White), Constraint::Min(0));

        block
    }

    /// Renders solve help
    fn solve_help() -> Paragraph {
        Paragraph::new(vec![
            "[[↑|k]Next stat".fg(Color::Gray).into(),
            "[↓/j]Prev. stat".fg(Color::Gray).into(),
            "[Tab]Close stats".fg(Color::Gray).into(),
            "[Esc|q]Quit".fg(Color::Gray).into(),
        ])
        .separator(" ")
    }
}
