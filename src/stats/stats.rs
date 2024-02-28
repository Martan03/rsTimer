use std::collections::HashMap;

//use chrono::{offset, DateTime, Local};
//use dirs::config_dir;
use eyre::{Report, Result};
use serde::{Deserialize, Serialize};

use crate::stats::session::Session;
use crate::stats::stat::Stat;

#[derive(Debug, Serialize, Deserialize)]
pub struct Stats {
    pub sessions: HashMap<String, Session>,
}

impl Stats {
    /// Loads stats from JSON file
    ///
    /// **Returns:**
    /// * Loaded stats in Result - contains error message on error
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
    /// * Ok() on success, else Err with corresponding error message
    pub fn save(&self) -> Result<()> {
        let filename = Stats::get_stats_dir()?;
        let path = std::path::Path::new(&filename);
        let prefix = path
            .parent()
            .ok_or(Report::msg("Error creating stats directory"))?;
        std::fs::create_dir_all(prefix)?;
        std::fs::File::create(path)?;

        let text = serde_json::to_string_pretty::<Stats>(self)?;
        std::fs::write(path, text)?;

        Ok(())
    }

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

    /// Adds given session to the stats
    ///
    /// **Parameters:**
    /// * `session` - name of the session
    /// * `scramble_type` - type of the scramble
    ///
    /// **Returns:**
    /// * Ok on success, else Err with corresponding error message
    pub fn add_session(
        &mut self,
        session: &str,
        scramble_type: &str,
    ) -> Result<()> {
        if self.exists(session) {
            Err(Report::msg("Error: session with this name already exists"))
        } else {
            self.sessions
                .insert(session.to_owned(), Session::new(scramble_type));
            Ok(())
        }
    }

    /// Gets all session names
    ///
    /// **Returns:**
    /// * Vector containing all session names
    pub fn get_sessions(&self) -> Vec<String> {
        self.sessions.keys().map(|v| v.to_string()).collect()
    }

    /// Checks whether session exists
    ///
    /// **Returns:**
    /// * True when exists, else falses
    pub fn exists(&self, session: &str) -> bool {
        self.sessions.contains_key(session)
    }

    /// This might be removed (doesn't really make sense to be here)
    /// TODO
    pub fn print_sessions(&self) {
        println!("\x1b[92mSessions:");

        let keys: Vec<_> = self.sessions.keys().cloned().collect();
        for key in keys {
            print!("  \x1b[93m{key}\x1b[0m (scramble type: ");
            match self.sessions.get(&key) {
                Some(session) => println!("{})", session.scramble_type),
                None => println!("Unknown)"),
            }
        }
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
