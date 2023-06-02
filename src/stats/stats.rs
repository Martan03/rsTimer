use std::{io::Write, time::Duration, collections::HashMap};

use chrono::{offset, DateTime, Local};
use dirs::config_dir;
use eyre::{Report, Result};
use serde::{Deserialize, Serialize};

use crossterm::event::{poll, read, Event, KeyCode};

use crate::stats::stat::Stat;

#[derive(Serialize, Deserialize)]
pub struct Stats {
    sessions: HashMap<String, Vec<Stat>>,
}

impl Stats {
    /// Adds given stat to the stats of given session
    /// 
    /// **Parameters:**
    /// * `stat` - stat to be added
    /// * `session` - session name
    pub fn add(&self, stat: Stat, session: String) {
        self.sessions.entry(session).or_insert(Vec::new()).push(stat);
    }

    /// Gets session by given name
    /// 
    /// **Parameters:**
    /// * `session` - session name
    /// 
    /// **Returns:**
    /// * Stats vector of the session with session name
    pub fn get_session(&self, session: String) -> Vec<Stat> {
        self.sessions[session]
    }

    /// Loads stats from JSON file
    /// 
    /// **Returns:**
    /// * Loaded stats in Result
    pub fn load() -> Result<Stats> {
        let stats = match std::fs::read_to_string("./stats".to_owned()) {
            Err(_) => Stats {
                sessions: HashMap::new(),
            },
            Ok(s) => serde_json::from_str::<Session>(&s)?,
        };
        Ok(stats)
    }

    /// Saves stats to json file
    /// 
    /// **Returns:**
    /// * Ok() on success, else Err with Report message
    pub fn save(&self) -> Result<()> {
        let filename = "./stats".to_owned();
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
}

#[derive(Serialize, Deserialize)]
pub struct Session {
    pub name: String,
    stats: Vec<Stat>,
}

impl Session {
    /// Loads stats from json file
    pub fn load() -> Result<Session> {
        let stats = match std::fs::read_to_string(Session::get_stats_dir()?) {
            Err(_) => Session {
                name: "1".to_owned(),
                stats: Vec::new(),
            },
            Ok(s) => serde_json::from_str::<Session>(&s)?,
        };
        Ok(stats)
    }

    /// Saves stats to json file
    pub fn save(&self) -> Result<()> {
        let filename = Session::get_stats_dir()?;
        let path = std::path::Path::new(&filename);
        let prefix = path
            .parent()
            .ok_or(Report::msg("Error creating stats directory"))?;
        std::fs::create_dir_all(&prefix)?;
        std::fs::File::create(&path)?;

        let text = serde_json::to_string_pretty::<Session>(self)?;
        std::fs::write(&path, text)?;

        Ok(())
    }

    /// Displays statistics
    pub fn display(&mut self) -> Result<()> {
        print!("\x1b[H\x1b[J");

        self.print_border();

        self.print_stats(0, 10);

        let mut con = true;
        while con {
            if poll(Duration::from_millis(100))? {
                con = self.key_listener()?;
            }
        }

        Ok(())
    }

    /// Listens for key presses
    fn key_listener(&mut self) -> Result<bool> {
        let event = read()?;

        // Ends statistics loop when ESC pressed
        if event == Event::Key(KeyCode::Esc.into())
            || event == Event::Key(KeyCode::Tab.into())
        {
            return Ok(false);
        }
        Ok(true)
    }

    /// Adds stat to stats
    pub fn add_stat(&mut self, stat: Stat) {
        self.stats.push(stat);
    }

    fn print_stats(&mut self, from: usize, mut to: usize) {
        if to > self.stats.len() {
            to = self.stats.len();
        }

        print!("\x1b[H");
        for stat in self.stats[from..to].iter() {
            let hours = stat.time.as_secs() / 3600;
            let mins = (stat.time.as_secs() / 60) % 60;
            let secs_mils = stat.time.as_secs_f32() % 60.;

            print!("\x1b[1E\x1b[3G");
            if hours != 0 {
                print!("{hours}:{:06}:{:06.3}", mins, secs_mils);
            } else if mins != 0 {
                print!("{mins}:{:06.3}", secs_mils);
            } else {
                print!("{:.3}", secs_mils);
            }
        }
        _ = std::io::stdout().flush();
    }

    // Gets the directory to save stats in
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

    /// Prints border around window
    fn print_border(&mut self) {
        let (w, h) = termion::terminal_size().unwrap();

        print!("\x1b[H\x1b[0m{:▄<1$}", "", w as usize);
        for _i in 1..h - 1 {
            print!("\x1b[1E█\x1b[{w}G█")
        }
        print!("\x1b[1E{:▀<1$}", "", w as usize);
        _ = std::io::stdout().flush();
    }
}

#[derive(Serialize, Deserialize)]
pub struct Stats {
    stats: Vec<Session>,
}

impl Stats {
    /// Loads all sessions from json file
    pub fn load_all() -> Result<Stats> {
        let stats = match std::fs::read_to_string(Session::get_stats_dir()?) {
            Err(_) => Stats { stats: Vec::new() },
            Ok(s) => serde_json::from_str::<Stats>(&s)?,
        };
        Ok(stats)
    }
}
