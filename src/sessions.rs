use crossterm::event::KeyCode;
use termint::{
    enums::Color,
    geometry::Constraint,
    style::Style,
    widgets::{Layout, List, Spacer},
};

use crate::{
    app::{App, Screen},
    error::Error,
    scramble::get_scramble,
    widgets::raw_span::RawSpan,
};

impl App {
    /// Renders sessions list
    pub fn render_sessions(&mut self) -> Result<(), Error> {
        let mut layout = Layout::vertical();
        layout.add_child(Spacer::new(), Constraint::Fill);
        layout.add_child(self.title(), Constraint::Length(6));
        layout.add_child(Spacer::new(), Constraint::Length(1));
        layout.add_child(self.list(), Constraint::Fill);
        layout.add_child(self.sessions_help(), Constraint::Length(1));

        self.term.render(layout)?;
        Ok(())
    }

    /// Listens to pressed keys while showing Sessions screen
    pub fn listen_sessions(&mut self, code: KeyCode) -> Result<(), Error> {
        match code {
            KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('K') => {
                {
                    let mut state = self.sessions_state.borrow_mut();
                    if let Some(sel) = state.selected {
                        state.selected = Some(sel.saturating_sub(1));
                    }
                }
                self.term.rerender()?;
                Ok(())
            }
            KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('J') => {
                {
                    let mut state = self.sessions_state.borrow_mut();
                    if let Some(sel) = state.selected {
                        if sel + 1 < self.stats.sessions.len() {
                            state.selected = Some(sel + 1);
                        }
                    }
                }
                self.term.rerender()?;
                Ok(())
            }
            KeyCode::Enter => {
                let Some(sel) = self.sessions_state.borrow().selected else {
                    return Ok(());
                };

                let session = self
                    .stats
                    .get_sessions()
                    .get(sel)
                    .ok_or(Error::Msg("getting session".to_string()))?
                    .to_string();

                let Some(session_info) = self.stats.get_session(&session)
                else {
                    return Ok(());
                };

                let mut scramble = get_scramble(&session_info.scramble_type);
                scramble.generate();
                self.scramble = Some(scramble);

                self.session = Some(session);
                self.screen = Screen::Timer;

                self.render_timer()
            }
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
                Err(Error::Exit)
            }
            _ => Ok(()),
        }
    }

    /// Inserts title to the given block
    fn title(&self) -> Layout {
        let title = format!(
            "{}\n{}\n{}\n{}\n{}\n{}",
            "██████╗ ███████╗████████╗██╗███╗   ███╗███████╗██████╗ ",
            "██╔══██╗██╔════╝╚══██╔══╝██║████╗ ████║██╔════╝██╔══██╗",
            "██████╔╝███████╗   ██║   ██║██╔████╔██║█████╗  ██████╔╝",
            "██╔══██╗╚════██║   ██║   ██║██║╚██╔╝██║██╔══╝  ██╔══██╗",
            "██║  ██║███████║   ██║   ██║██║ ╚═╝ ██║███████╗██║  ██║",
            "╚═╝  ╚═╝╚══════╝   ╚═╝   ╚═╝╚═╝     ╚═╝╚══════╝╚═╝  ╚═╝",
        );

        let mut layout = Layout::horizontal().center();
        layout.add_child(
            RawSpan::new(title).fg(Color::DarkCyan),
            Constraint::Length(56),
        );
        layout
    }

    fn list(&self) -> Layout {
        let mut layout = Layout::horizontal().center();
        let keys = self.stats.get_sessions();
        if keys.is_empty() {
            layout.add_child("No sessions...", Constraint::Min(0));
        } else {
            layout.add_child(
                List::new(keys, self.sessions_state.clone())
                    .selected_style(Style::new().fg(Color::DarkCyan))
                    .auto_scroll(),
                Constraint::Length(5),
            );
        }
        layout
    }

    /// Gets sessions help layout
    fn sessions_help(&self) -> Layout {
        let mut layout = Layout::horizontal();
        layout.add_child(
            RawSpan::new("[↑|k]Move up ").fg(Color::Gray),
            Constraint::Min(0),
        );
        layout.add_child(
            RawSpan::new("[↓/j]Move down ").fg(Color::Gray),
            Constraint::Min(0),
        );
        layout.add_child(
            RawSpan::new("[Enter]Select session ").fg(Color::Gray),
            Constraint::Min(0),
        );
        layout.add_child(
            RawSpan::new("[Esc|q]Quit ").fg(Color::Gray),
            Constraint::Min(0),
        );
        layout
    }
}
