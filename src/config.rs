use std::{
    fs::{create_dir_all, read_to_string, File},
    io::Write,
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

use crate::{asci::digit_type::DigitType, error::Error};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub font: DigitType,
}

impl Config {
    /// Loads config, uses default config when not found
    pub fn load() -> Config {
        let mut dir = Config::get_path();
        dir.push("config.json");

        match serde_json::from_str::<Self>(
            &read_to_string(dir).unwrap_or(String::new()),
        ) {
            Ok(conf) => conf,
            Err(_) => Self::default(),
        }
    }

    /// Saves config to the JSON file
    pub fn save(&self) -> Result<(), Error> {
        let mut dir = Config::get_path();
        create_dir_all(&dir)?;

        dir.push("config.json");
        let mut file = File::create(&dir)?;

        let json_string = serde_json::to_string_pretty(self)?;
        file.write_all(json_string.as_bytes())?;
        Ok(())
    }

    /// Sets font to the given value and saves the config
    pub fn set_font(&mut self, font: DigitType) -> Result<(), Error> {
        self.font = font;
        self.save()
    }

    /// Gets path of the config folder
    fn get_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or(PathBuf::from("."));
        path.push("rstimer");
        path
    }
}
