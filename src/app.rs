use std::{
    cell::RefCell,
    io::{stdout, Write},
    rc::Rc,
    time::Duration,
};

use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use termint::{
    enums::{Color, Modifier},
    geometry::{Constraint, TextAlign},
    style::Style,
    term::Term,
    widgets::{Block, BorderType, Layout, List, ListState, StrSpanExtension},
};

use crate::{
    config::Config,
    error::Error,
    scramble::{get_scramble, Scramble},
    stats::stats::Stats,
};

#[derive(Debug, Clone, Default)]
pub enum Screen {
    Timer,
    Stats,
    #[default]
    Sessions,
}

/// App struct containing the main loop, key listeners and rendering
#[derive(Debug)]
pub struct App {
    pub term: Term,
    pub config: Config,
    pub screen: Screen,
    pub session: Option<String>,
    pub scramble: Option<Scramble>,
    pub stats: Stats,
    pub time: Duration,
    pub sessions_state: Rc<RefCell<ListState>>,
    pub stats_state: Rc<RefCell<ListState>>,
}

impl App {
    /// Creates new [`App`]
    pub fn new() -> Self {
        Self {
            term: Term::new().small_screen(App::small_screen()),
            ..Default::default()
        }
    }

    /// Creates new [`App`] and opens given session
    pub fn open(session: String) -> Self {
        let stats = Stats::load();
        let Some(session_info) = stats.get_session(&session) else {
            return Self::default();
        };

        let mut scramble = get_scramble(&session_info.scramble_type);
        scramble.generate();

        Self {
            session: Some(session),
            screen: Screen::Timer,
            scramble: Some(scramble),
            stats,
            term: Term::new().small_screen(App::small_screen()),
            ..Default::default()
        }
    }

    /// Runs the [`App`]
    pub fn run(&mut self) -> Result<(), Error> {
        // Saves screen, clears screen and hides cursor
        print!("\x1b[?1049h\x1b[2J\x1b[?25l");
        _ = stdout().flush();
        enable_raw_mode()?;

        let res = self.main_loop();

        disable_raw_mode()?;
        // Restores screen
        print!("\x1b[?1049l\x1b[?25h");
        _ = stdout().flush();

        match res {
            Err(Error::Exit) => Ok(()),
            _ => res,
        }
    }

    /// Main loop of the [`App`]
    fn main_loop(&mut self) -> Result<(), Error> {
        self.render()?;
        loop {
            if poll(Duration::from_millis(100))? {
                self.key_listener()?;
            }
        }
    }

    /// Renders current screen of the [`App`]
    pub fn render(&mut self) -> Result<(), Error> {
        match self.screen {
            Screen::Timer => self.render_timer(),
            Screen::Stats => self.render_stats(),
            Screen::Sessions => self.render_sessions(),
        }
    }

    /// Handles key listening
    fn key_listener(&mut self) -> Result<(), Error> {
        let Event::Key(KeyEvent { code, .. }) = read()? else {
            return Ok(());
        };

        match self.screen {
            Screen::Timer => self.listen_timer(code),
            Screen::Stats => self.listen_stats(code),
            Screen::Sessions => self.listen_sessions(code),
        }
    }

    fn small_screen() -> Layout {
        let mut layout = Layout::vertical().center();
        layout.add_child(
            "Terminal too small!"
                .modifier(Modifier::BOLD)
                .align(TextAlign::Center),
            Constraint::Min(0),
        );
        layout.add_child(
            "You have to increase terminal size".align(TextAlign::Center),
            Constraint::Min(0),
        );
        layout
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            term: Default::default(),
            config: Config::load(),
            screen: Default::default(),
            session: None,
            scramble: None,
            stats: Stats::load(),
            time: Duration::new(0, 0),
            sessions_state: Rc::new(RefCell::new(ListState::selected(0, 0))),
            stats_state: Rc::new(RefCell::new(ListState::selected(0, 0))),
        }
    }
}

impl App {
    /// Renders Stats screen
    fn render_stats(&mut self) -> Result<(), Error> {
        let name = self.session.clone().unwrap_or("".to_string());
        let mut block = Block::vertical()
            .title(format!("{} stats", name).as_str())
            .border_type(BorderType::Thicker);

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
                List::new(stats, self.stats_state.clone())
                    .selected_style(Style::new().fg(Color::Cyan))
                    .auto_scroll(),
                Constraint::Fill,
            );
        }

        self.term.render(block)?;
        Ok(())
    }

    /// Listens to pressed keys while showing Stats screen
    fn listen_stats(&mut self, code: KeyCode) -> Result<(), Error> {
        match code {
            KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('K') => {
                {
                    let mut state = self.stats_state.borrow_mut();
                    if let Some(sel) = state.selected {
                        state.selected = Some(sel.saturating_sub(1));
                    }
                }
                self.term.rerender()?;
                Ok(())
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
                Ok(())
            }
            KeyCode::Delete => {
                if let Some(sel) = self.stats_state.borrow().selected {
                    self.stats.remove(sel, self.session.as_ref().unwrap());
                }
                self.render_stats()
            }
            KeyCode::Tab => {
                self.screen = Screen::Timer;
                self.render_timer()
            }
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
                Err(Error::Exit)
            }
            _ => Ok(()),
        }
    }
}
