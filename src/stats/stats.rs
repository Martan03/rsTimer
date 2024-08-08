use std::collections::HashMap;
use std::time::Duration;

use dirs::config_dir;
//use chrono::{offset, DateTime, Local};
//use dirs::config_dir;
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::stats::session::Session;
use crate::stats::stat::Stat;

#[derive(Debug, Serialize, Deserialize)]
pub struct Stats {
    pub sessions: HashMap<String, Session>,
}

impl Stats {
    /// Loads stats from JSON file
    pub fn load() -> Stats {
        match std::fs::read_to_string(
            Stats::get_stats_dir().unwrap_or(".".to_string()),
        ) {
            Err(_) => Stats {
                sessions: HashMap::new(),
            },
            Ok(s) => serde_json::from_str::<Stats>(&s).unwrap_or(Stats {
                sessions: HashMap::new(),
            }),
        }
    }

    /// Saves stats to json file
    pub fn save(&self) -> Result<(), Error> {
        let filename = Stats::get_stats_dir()?;
        let path = std::path::Path::new(&filename);
        let prefix = path
            .parent()
            .ok_or(Error::Msg("Error creating stats directory".to_string()))?;
        std::fs::create_dir_all(prefix)?;
        std::fs::File::create(path)?;

        let text = serde_json::to_string_pretty::<Stats>(self)?;
        std::fs::write(path, text)?;

        Ok(())
    }

    /// Adds given stat to the stats of given session
    pub fn add(&mut self, stat: Stat, session: &str) -> Result<(), Error> {
        if let Some(session) = self.sessions.get_mut(session) {
            session.add(stat);
            Ok(())
        } else {
            Err(Error::Msg("non existing session".to_string()))
        }
    }

    /// Removes [`Stat`] from given [`Session`]
    pub fn remove(&mut self, index: usize, session: &str) {
        if let Some(session) = self.sessions.get_mut(session) {
            session.remove(index);
        }
    }

    /// Adds given session to the stats
    pub fn add_session(
        &mut self,
        session: &str,
        scramble_type: &str,
    ) -> Result<(), Error> {
        if self.exists(session) {
            Err(Error::Msg(
                "session with this name already exists".to_string(),
            ))
        } else {
            self.sessions
                .insert(session.to_owned(), Session::new(scramble_type));
            Ok(())
        }
    }

    pub fn get_session(&self, name: &str) -> Option<&Session> {
        self.sessions.get(name)
    }

    /// Gets all session names
    pub fn get_sessions(&self) -> Vec<String> {
        self.sessions.keys().map(|v| v.to_string()).collect()
    }

    /// Checks whether session exists
    pub fn exists(&self, session: &str) -> bool {
        self.sessions.contains_key(session)
    }

    /// Gets average of the given session
    pub fn avg(&self, session: &str) -> Option<Duration> {
        let session = self.sessions.get(session)?;
        let total: Duration = session.stats.iter().map(|s| s.time).sum();

        if session.stats.is_empty() {
            None
        } else {
            Some(total / session.stats.len() as u32)
        }
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
}

impl Stats {
    /// Gets the directory to save stats in
    fn get_stats_dir() -> Result<String, Error> {
        let config = config_dir()
            .ok_or(Error::Msg("Can't get stats directory".to_string()))?;

        Ok(config
            .to_str()
            .ok_or(Error::Msg("Invalid path to stats".to_string()))?
            .to_owned()
            + "/rstimer/stats")
    }
}
