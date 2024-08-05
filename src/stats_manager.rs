use std::{
    io::{stdout, Write},
    time::Duration,
};

use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use termint::{
    geometry::Constraint,
    term::Term,
    widgets::{Block, BorderType},
};

use crate::{
    error::Error,
    scramble::{get_scramble, Scramble},
    stats::{stat::Stat, stats::Stats},
};

/// Stores session stats, session name and current scramble
pub struct StatsManager {
    pub stats: Stats,
    pub session: String,
    pub scramble: Scramble,
}

//  ██████   ██ ██████  ██████  ██   ██ ███████  ██████  ███████  █████   █████
// ██  ████ ███      ██      ██ ██   ██ ██      ██            ██ ██   ██ ██   ██
// ██ ██ ██  ██  █████   █████  ███████ ███████ ███████      ██   █████   ██████
// ████  ██  ██ ██           ██      ██      ██ ██    ██    ██   ██   ██      ██
//  ██████   ██ ███████ ██████       ██ ███████  ██████     ██    █████   █████  ██

impl StatsManager {
    /// Loads stats and opens given session
    ///
    /// **Parameters:**
    /// * `name` - name of the session to be opened
    ///
    /// **Returns:**
    /// * Ok() on success, else Err()
    pub fn open(name: &str) -> Result<StatsManager, Error> {
        let stats = Stats::load();

        let Some(session) = stats.get_session(name).cloned() else {
            return Err(Error::Msg(format!(
                "Session '{}' doesn't exist.",
                name
            )));
        };

        Ok(StatsManager {
            stats,
            session: name.to_owned(),
            scramble: get_scramble(&session.scramble_type),
        })
    }

    /// Displays session picker and open selected session
    ///
    /// **Returns:**
    /// * [`StatsManager`] in Result, Err with error message when occures
    pub fn picker() -> Result<StatsManager, Error> {
        let stats = Stats::load();
        let mut cur = stats.get_sessions().first().map(|_| 0_usize);

        let mut mngr = Self {
            stats,
            session: "".to_owned(),
            scramble: Scramble::new(0, vec![]),
        };

        enable_raw_mode()?;

        mngr.session_pick_render(cur)?;
        let mut con = true;
        while con {
            if poll(Duration::from_millis(100))? {
                if let Err(e) = mngr.session_pick_listen(&mut con, &mut cur) {
                    disable_raw_mode()?;
                    return Err(e);
                };
            }
        }

        disable_raw_mode()?;
        let current = cur.ok_or(Error::Msg("No item selected".to_string()))?;
        mngr.session = mngr
            .stats
            .get_sessions()
            .get(current)
            .ok_or(Error::Msg("Getting session".to_string()))?
            .to_owned();
        let Some(session) = mngr.stats.sessions.get(&mngr.session) else {
            return Err(Error::Msg("Session doesn't exist".to_string()));
        };
        mngr.scramble = get_scramble(&session.scramble_type);
        Ok(mngr)
    }

    /// Opens stats window
    ///
    /// **Returns:**
    /// Ok(bool) on success - true to exit app - else Err()
    pub fn open_stats(&mut self, exit: &mut bool) -> Result<(), Error> {
        let mut cur = Some(0_usize);
        let mut con = true;

        self.stats_render(cur, None)?;

        while con {
            if poll(Duration::from_millis(100))? {
                self.stats_listen(&mut cur, &mut con, exit)?;
            }
        }

        Ok(())
    }

    /// Adds time to active session
    ///
    /// **Parameters:**
    /// * `time` - time it took to solve the cube
    ///
    /// **Returns:**
    /// * Ok() on success, else Err()
    pub fn add_time(&mut self, time: Duration) -> Result<(), Error> {
        self.stats.add(
            Stat::new(time, self.scramble.get().to_owned(), "".to_owned()),
            &self.session,
        )?;
        Ok(())
    }

    /// Opens sessions list
    pub fn open_session_list(&self) {
        self.display_sessions();
    }
}

// Private method implementations
impl StatsManager {
    /// Session picker key listener
    fn session_pick_listen(
        &self,
        con: &mut bool,
        cur: &mut Option<usize>,
    ) -> Result<(), Error> {
        let Event::Key(KeyEvent { code, .. }) = read()? else {
            return Ok(());
        };

        match code {
            // Quits session picker
            KeyCode::Esc => {
                return Err(Error::Msg("User quit selection".to_string()))
            }
            // Moves list selection one item up
            KeyCode::Up => {
                if let Some(val) = cur {
                    *cur = Some(val.saturating_sub(1));
                    self.session_pick_render(*cur)?;
                }
            }
            // Moves list selection one item down
            KeyCode::Down => {
                if let Some(val) = cur {
                    if *val + 1 < self.stats.get_sessions().len() {
                        *cur = Some(*val + 1);
                        self.session_pick_render(*cur)?;
                    }
                }
            }
            // Select current session
            KeyCode::Enter => *con = false,
            _ => {}
        }

        Ok(())
    }

    /// Renders session picker
    fn session_pick_render(&self, _cur: Option<usize>) -> Result<(), Error> {
        let mut block = Block::vertical()
            .title("Sessions")
            .border_type(BorderType::Thicker);

        let keys = self.stats.get_sessions();
        if keys.is_empty() {
            block.add_child("No sessions...", Constraint::Fill);
        } else {
            // let list = List::new(keys).selected(cur);
            // block.add_child(list, Constraint::Fill);
        }

        let mut term = Term::new();
        print!("\x1b[H\x1b[J");
        _ = term.render(block);
        Ok(stdout().flush()?)
    }

    /// Displays sessions
    fn display_sessions(&self) {
        println!("\x1b[2J\x1b[H\x1b[92mSessions:\x1b[0m");

        for (key, value) in self.stats.sessions.iter() {
            println!(
                "\x1b[0G  \x1b[93m{key}\x1b[0m (scramble type: {})",
                value.scramble_type
            );
        }
    }

    /// Listens to key presses and reacts to it while stats window is active
    fn stats_listen(
        &mut self,
        cur: &mut Option<usize>,
        con: &mut bool,
        exit: &mut bool,
    ) -> Result<(), Error> {
        let Event::Key(KeyEvent { code, .. }) = read()? else {
            return Ok(());
        };

        match code {
            KeyCode::Up => {
                if let Some(val) = *cur {
                    *cur = Some(val.saturating_sub(1));
                    self.stats_render(*cur, Some(val))?;
                }
            }
            KeyCode::Down => {
                if let Some(val) = *cur {
                    if val + 1 < self.stats.sessions[&self.session].stats.len()
                    {
                        *cur = Some(val + 1);
                        self.stats_render(*cur, Some(val))?;
                    }
                }
            }
            KeyCode::Delete => {
                if let Some(val) = *cur {
                    self.stats.remove(val, &self.session);
                    self.stats_render(*cur, Some(val))?;
                }
            }
            KeyCode::Tab => *con = false,
            KeyCode::Esc => {
                *exit = true;
                *con = false;
            }
            _ => {}
        }
        Ok(())
    }

    /// Renders sessions stats
    fn stats_render(
        &self,
        _cur: Option<usize>,
        _prev: Option<usize>,
    ) -> Result<(), Error> {
        // let mut block =
        //     Block::new().title("Stats").border_type(BorderType::Thicker);

        // let stats: Vec<String> = self.stats.sessions[&self.session]
        //     .stats
        //     .iter()
        //     .map(|i| format!("{:.3}", i.time.as_secs_f64()))
        //     .collect();

        // if stats.is_empty() {
        //     block.add_child("Not times set yet...", Constrain::Fill);
        // } else {
        //     let prev = prev.unwrap_or(0);
        //     let list = List::new(stats).selected(cur).to_current(prev);
        //     block.add_child(list, Constrain::Fill);
        // }

        // let term = Term::new();
        // print!("\x1b[H\x1b[J");
        // term.render(block).map_err(Report::msg)?;
        Ok(stdout().flush()?)
    }
}
