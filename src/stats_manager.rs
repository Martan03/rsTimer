use eyre::{Report, Result};

use crate::stats::{stats::Stats, stat::Stat};

pub struct StatsManager {
    stats: Stats,
    session: String,
}

impl StatsManager {
    /// Loads stats and opens given session
    /// 
    /// **Parameters:**
    /// * `session` - name of the session to be opened
    /// 
    /// **Returns:**
    /// * Ok() on success, else Err
    pub fn open_session(&mut self, session: &str) -> Result<()> {
        self.stats = Stats::load()?;

        if self.stats.exists(session) {
            self.session = session.to_owned();
            Ok(())
        } else {
            Err(Report::msg("Error: given session doesn't exist"))
        }
    }

    pub fn add_session(&mut self, session: &str) {
        // todo
    }
}
