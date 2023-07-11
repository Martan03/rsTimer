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
}
