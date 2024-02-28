use std::{
    io::{stdout, Write},
    mem,
    time::Duration,
};

use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use eyre::{Report, Result};
use termint::{
    geometry::constrain::Constrain,
    term::Term,
    widgets::{block::Block, border::BorderType, list::List},
};

use crate::{
    scramble::{get_scramble, Scramble},
    stats::{stat::Stat, stats::Stats},
};

/// Stores session stats, session name and current scramble
pub struct StatsManager {
    pub stats: Stats,
    pub session: String,
    pub scramble: Scramble,
}

impl StatsManager {
    /// Loads stats and opens given session
    ///
    /// **Parameters:**
    /// * `name` - name of the session to be opened
    ///
    /// **Returns:**
    /// * Ok() on success, else Err()
    pub fn open(name: &str) -> Result<StatsManager> {
        let stats = Stats::load()?;

        let Some(session) = stats.get_session(name).cloned() else {
            return Err(Report::msg(format!(
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
    pub fn picker() -> Result<StatsManager> {
        let stats = Stats::load()?;
        let mut cur = stats.get_sessions().first().map(|_| 0_usize);

        let mut mngr = Self {
            stats,
            session: "".to_owned(),
            scramble: Scramble::new(0, vec![]),
        };

        enable_raw_mode()?;

        mngr.render_session_pick(cur);
        let mut con = true;
        while con {
            if poll(Duration::from_millis(100))? {
                if let Err(e) = mngr.session_pick_listen(&mut con, &mut cur) {
                    disable_raw_mode()?;
                    return Err(Report::msg(e));
                };
            }
        }

        disable_raw_mode()?;
        let current = cur.ok_or(Report::msg("No item selected"))?;
        mngr.session = mngr
            .stats
            .get_sessions()
            .get(current)
            .ok_or(Report::msg("Getting session"))?
            .to_owned();
        let Some(session) = mngr.stats.sessions.get(&mngr.session) else {
            return Err(Report::msg("Session doesn't exist"));
        };
        mngr.scramble = get_scramble(&session.scramble_type);
        Ok(mngr)
    }

    /// Adds time to active session
    ///
    /// **Parameters:**
    /// * `time` - time it took to solve the cube
    ///
    /// **Returns:**
    /// * Ok() on success, else Err()
    pub fn add_time(&mut self, time: Duration) -> Result<()> {
        self.stats.add(
            Stat::new(time, self.scramble.get().to_owned(), "".to_owned()),
            &self.session,
        )?;
        Ok(())
    }

    /// Opens stats window
    ///
    /// **Returns:**
    /// Ok(bool) on success - true to exit app - else Err()
    pub fn open_stats(&self) -> Result<bool> {
        let mut active_stat: usize = 0;
        let mut exit = false;

        self.display_stats(active_stat);

        while self.stats_key_listener(
            &mut active_stat,
            self.stats.sessions[&self.session].stats.len(),
            &mut exit,
        )? {
            // Empty loop body
        }

        Ok(exit)
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
    ) -> Result<()> {
        let Event::Key(KeyEvent { code, .. }) = read()? else {
            return Ok(());
        };

        match code {
            // Quits session picker
            KeyCode::Esc => return Err(Report::msg("User quit selection")),
            // Moves list selection one item up
            KeyCode::Up => {
                if let Some(val) = cur {
                    *cur = Some(val.saturating_sub(1));
                    self.render_session_pick(*cur);
                }
            }
            // Moves list selection one item down
            KeyCode::Down => {
                if let Some(val) = cur {
                    if *val + 1 < self.stats.get_sessions().len() {
                        *cur = Some(*val + 1);
                        self.render_session_pick(*cur);
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
    fn render_session_pick(&self, cur: Option<usize>) {
        let mut block = Block::new()
            .title("Sessions")
            .border_type(BorderType::Thicker);

        let keys = self.stats.get_sessions();
        let sessions: Vec<&str> = keys.iter().map(|v| v.as_str()).collect();
        let static_sessions = unsafe { mem::transmute(sessions) };

        let list = List::new(static_sessions).current(cur);
        block.add_child(list, Constrain::Fill);

        let term = Term::new();
        print!("\x1b[H\x1b[J");
        _ = term.render(block);
        _ = stdout().flush();
    }

    /// Displays stats of active session
    fn display_stats(&self, active_stat: usize) {
        println!("\x1b[2J\x1b[H\x1b[92mStats:\x1b[0m");

        for (i, stat) in
            self.stats.sessions[&self.session].stats.iter().enumerate()
        {
            if active_stat == i {
                print!("\x1b[093m");
            }
            println!("\x1b[0G{:.3}\x1b[0m", stat.time.as_secs_f64());
        }
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
    fn stats_key_listener(
        &self,
        active: &mut usize,
        active_max: usize,
        exit: &mut bool,
    ) -> Result<bool> {
        if poll(Duration::from_millis(100))? {
            let event = read()?;

            if event == Event::Key(KeyCode::Tab.into()) {
                return Ok(false);
            }
            if event == Event::Key(KeyCode::Down.into())
                && *active < active_max - 1
            {
                *active += 1;
                self.display_stats(*active);
            }
            if event == Event::Key(KeyCode::Up.into()) && *active > 0 {
                *active -= 1;
                self.display_stats(*active);
            }
            if event == Event::Key(KeyCode::Esc.into()) {
                *exit = true;
                return Ok(false);
            }
        }
        Ok(true)
    }
}
