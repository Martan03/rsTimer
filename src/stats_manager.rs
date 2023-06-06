use eyre::{Report, Result};

use crate::stats::stats::Stats;

pub struct StatsManager {
    stats: Stats,
    session: String,
}

impl StatsManager {
    /// Loads stats from JSON
    /// 
    /// **Returns:**
    /// * Ok() on success, else Err
    pub fn load(&mut self) -> Result<()> {
        self.stats = Stats::load()?;
        Ok(())
    }

    /// Loads stats and opens given session
    /// 
    /// **Parameters:**
    /// * `session` - name of the session to be opened
    /// 
    /// **Returns:**
    /// * Ok() on success, else Err
    pub fn open_session(&mut self, session: &str) -> Result<()> {
        self.load()?;

        if self.stats.exists(session) {
            self.session = session.to_owned();
            Ok(())
        } else {
            Err(Report::msg("Error: given session doesn't exist"))
        }
    }
}
