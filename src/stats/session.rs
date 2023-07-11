use serde::{Deserialize, Serialize};

use crate::stats::stat::Stat;

#[derive(Serialize, Deserialize, Clone)]
/// Session struct
pub struct Session {
    pub scramble_type: String,
    stats: Vec<Stat>,
}

impl Session {
    /// Constructs new Session
    ///
    /// **Parameters:**
    /// * `name` - session name
    /// * `scramble_type` - scramble type
    ///
    /// **Returns:**
    /// * New Session
    pub fn new(scramble_type: &str) -> Session {
        Session {
            scramble_type: scramble_type.to_owned(),
            stats: Vec::new(),
        }
    }

    pub fn add(&mut self, stat: Stat) {
        self.stats.push(stat);
    }
}
