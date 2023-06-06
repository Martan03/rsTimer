use std::time::Duration;

use chrono::{offset, DateTime, Local};
use serde::{Deserialize, Serialize};

// Statistic struct
#[derive(Serialize, Deserialize, Clone)]
pub struct Stat {
    time: Duration,
    scramble: String,
    comment: String,
    #[serde(with = "stat_date_format")]
    datetime: DateTime<Local>,
}

/// Custom date format to be able to serialize the date
mod stat_date_format {
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
    /// Stat constructor
    ///
    /// **Parameters:**
    /// * `time` - duration it took to solve the cube
    /// * `scramble` - cube scramble
    /// * `comment` - comment to be added to this solve in stats
    ///
    /// **Returns:**
    /// * Created stat
    pub fn new(time: Duration, scramble: String, comment: String) -> Stat {
        Stat {
            time,
            scramble,
            comment,
            datetime: offset::Local::now(),
        }
    }
}
