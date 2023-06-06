use std::collections::HashMap;

//use chrono::{offset, DateTime, Local};
//use dirs::config_dir;
use eyre::{Report, Result};
use serde::{Deserialize, Serialize};

use crate::stats::session::Session;
use crate::stats::stat::Stat;

#[derive(Serialize, Deserialize)]
pub struct Stats {
    sessions: HashMap<String, Session>,
}

impl Stats {
    /// Adds given stat to the stats of given session
    ///
    /// **Parameters:**
    /// * `stat` - stat to be added
    /// * `session` - session name
    pub fn add(&mut self, stat: Stat, session: &str) -> Result<()> {
        if let Some(session) = self.sessions.get_mut(session) {
            session.add(stat);
            Ok(())
        } else {
            Err(Report::msg("Error: non existing session"))
        }
    }

    pub fn add_session(
        &mut self,
        session: &str,
        scramble_type: &str,
    ) -> Result<()> {
        if self.sessions.contains_key(session) {
            Err(Report::msg("Error: session with this name already exists"))
        } else {
            self.sessions
                .insert(session.to_owned(), Session::new(scramble_type));
            Ok(())
        }
    }

    /// Gets session by given name
    ///
    /// **Parameters:**
    /// * `session` - session name
    ///
    /// **Returns:**
    /// * Stats vector of the session with session name
    /*
    pub fn get_session(&self, session: &str) -> Result<Vec<Stat>> {
        match self.sessions.get(session) {
            Some(stats) => Ok(stats.clone()),
            None => Err(Report::msg("Error getting given session")),
        }
    }
    */

    /// Loads stats from JSON file
    ///
    /// **Returns:**
    /// * Loaded stats in Result
    pub fn load() -> Result<Stats> {
        let stats = match std::fs::read_to_string(Stats::get_stats_dir()?) {
            Err(_) => Stats {
                sessions: HashMap::new(),
            },
            Ok(s) => serde_json::from_str::<Stats>(&s)?,
        };
        Ok(stats)
    }

    /// Saves stats to json file
    ///
    /// **Returns:**
    /// * Ok() on success, else Err with Report message
    pub fn save(&self) -> Result<()> {
        let filename = Stats::get_stats_dir()?;
        let path = std::path::Path::new(&filename);
        let prefix = path
            .parent()
            .ok_or(Report::msg("Error creating stats directory"))?;
        std::fs::create_dir_all(&prefix)?;
        std::fs::File::create(&path)?;

        let text = serde_json::to_string_pretty::<Stats>(self)?;
        std::fs::write(&path, text)?;

        Ok(())
    }

    /// Gets the directory to save stats in
    ///
    /// **Returns:**
    /// * Ok(directory path string) on success, else Err
    fn get_stats_dir() -> Result<String> {
        Ok("./stats".to_owned())
        /* For debugging purposes stats directory will be in code directory
        let config =
        config_dir().ok_or(Report::msg("Can't get stats directory"))?;

        Ok(config
            .to_str()
            .ok_or(Report::msg("Invalid path to stats"))?
            .to_owned()
            + "/rstimer/stats")
        */
    }
}
