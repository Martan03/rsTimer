use std::time::Duration;

use crossterm::event::{poll, read, Event, KeyCode};
use eyre::{Report, Result};

use crate::{
    scramble::Scramble,
    scrambles::get_scramble,
    stats::{stat::Stat, stats::Stats},
};

pub struct StatsManager {
    pub stats: Stats,
    pub session: String,
    pub scramble: Scramble,
}

impl StatsManager {
    /// Loads stats and opens given session
    ///
    /// **Parameters:**
    /// * `session` - name of the session to be opened
    ///
    /// **Returns:**
    /// * Ok() on success, else Err()
    pub fn open_session(name: &str) -> Result<StatsManager> {
        let stats = Stats::load()?;

        if !stats.exists(name) {
            return Err(Report::msg("Error: given session doesn't exist"));
        }

        if let Some(session) = stats.sessions.get(name) {
            let (scramble_length, scramble_moves) =
                get_scramble(&session.scramble_type);

            return Ok(StatsManager {
                stats,
                session: name.to_owned(),
                scramble: Scramble::new(scramble_length, scramble_moves),
            });
        }

        Err(Report::msg("Error: scramble type not found"))
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

    /// Displays stats of active session
    ///
    /// **Parameters:**
    /// * `active_stat` - number of active stat to be highlighted
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

    pub fn open_session_list(&self) {
        self.display_sessions();
    }

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
    ///
    /// **Parameters:**
    /// * `active_stat` - number of active stat to be highlighted
    /// * `exit` - when true, app will be exited
    ///
    /// **Returns:**
    /// Ok(bool) on success - false to close stats - else Err()
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
