use serde::{Serialize, Deserialize};

use crate::stats::stat::Stat;

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
/// Session struct
pub struct Session {
    pub scramble_type: String,
    pub stats: Vec<Stat>,
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
    pub fn new(scramble_type: String) -> Session {
        Session {
            scramble_type,
            stats: Vec::new(),
        }
    }

    pub fn add(&mut self, stat: Stat) {
        self.stats.push(stat);
    }
}
