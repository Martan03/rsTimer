use std::time::Duration;

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

/// Stat struct containing information about solve
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Stat {
    pub time: Duration,
    scramble: String,
    comment: String,
    pub datetime: DateTime<Local>,
}

impl Stat {
    /// Creates new [`Stat`]
    ///
    /// **Parameters:**
    /// * `time` - duration it took to solve the cube
    /// * `scramble` - cube scramble
    /// * `comment` - comment to be added to this solve in stats
    ///
    /// **Returns:**
    /// * Created [`Stat`]
    pub fn new(time: Duration, scramble: String, comment: String) -> Stat {
        Stat {
            time,
            scramble,
            comment,
            datetime: Local::now(),
        }
    }
}
