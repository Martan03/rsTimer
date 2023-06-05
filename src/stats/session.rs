use serde::{Serialize, Deserialize};

use crate::stats::stat::Stat;
use crate::stats::stats::Stats;

#[derive(Serialize, Deserialize)]
/// Session struct
pub struct Session {
    name: String,
    scramble_type: String,
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
    pub fn new(name: String, scramble_type: String) -> Session {
        Session {
            name,
            scramble_type,
            stats: Vec::new(),
        }
    }
}
