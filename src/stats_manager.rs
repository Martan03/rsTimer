use std::time::Duration;

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
                stats: stats,
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

    pub fn display_stats(&self) {
        println!("\x1b[0m\x1b[2J\x1b[HStats:");
        println!("\x1b[0Gtest");
        for i in 0..self.stats.sessions[&self.session].stats.len() {
            println!("\x1b[0G{}", self.stats.sessions[&self.session].stats[i].time.as_secs_f64());
        }
    }

    pub fn display_sessions(&self) {
        println!("\x1b[2J\x1b[92mSessions:");

        for (key, value) in self.stats.sessions.iter() {
            println!(
                "  \x1b[93m{key}\x1b[0m (scramble type: {})",
                value.scramble_type
            );
        }
    }
}
