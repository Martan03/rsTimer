use std::time::Duration;

use chrono::{offset, DateTime, Local};
use dirs::config_dir;
use eyre::{Report, Result};
use serde::{Deserialize, Serialize};

use crossterm::event::{poll, read, Event, KeyCode};

#[derive(Serialize, Deserialize)]
pub struct Stat {
    time: Duration,
    scramble: String,
    comment: String,
    #[serde(with = "my_date_format")]
    datetime: DateTime<Local>,
}

mod my_date_format {
    use chrono::{DateTime, Local, TimeZone};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    pub fn serialize<S>(
        date: &DateTime<Local>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<DateTime<Local>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Local
            .datetime_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
    }
}

impl Stat {
    pub fn new(time: Duration, scramble: String, comment: String) -> Stat {
        Stat {
            time,
            scramble,
            comment,
            datetime: offset::Local::now(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Stats {
    stats: Vec<Stat>,
}

impl Stats {
    /// Loads stats from json file
    pub fn load() -> Result<Stats> {
        let stats = match std::fs::read_to_string(Stats::get_stats_dir()?) {
            Err(_) => Stats { stats: Vec::new() },
            Ok(s) => serde_json::from_str::<Stats>(&s)?,
        };
        Ok(stats)
    }

    /// Saves stats to json file
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

    /// Displays statistics
    pub fn display(&mut self) -> Result<()> {
        print!("\x1b[H\x1b[J");

        for stat in self.stats.iter() {
            println!("{}", stat.time.as_secs_f64())
        }

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

    /// Gets the directory to save stats in
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
