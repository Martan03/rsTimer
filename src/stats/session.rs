use serde::{Deserialize, Serialize};

use crate::stats::stat::Stat;

#[derive(Debug, Serialize, Deserialize, Clone)]
/// Session struct to store its stats and scramble type
pub struct Session {
    pub scramble_type: String,
    pub stats: Vec<Stat>,
}

impl Session {
    /// Constructs new Session
    ///
    /// **Parameters:**
    /// * `scramble_type` - scramble type
    ///
    /// **Returns:**
    /// * Created [`Session`]
    pub fn new(scramble_type: &str) -> Session {
        Session {
            scramble_type: scramble_type.to_owned(),
            stats: Vec::new(),
        }
    }

    /// Adds new [`Stat`] to the [`Session`]
    ///
    /// **Parameters:**
    /// * `stat` - [`Stat`] to be stored in [`Session`]
    pub fn add(&mut self, stat: Stat) {
        self.stats.push(stat);
    }

    /// Removes [`Stat`] from [`Session`]
    ///
    /// **Parameters:**
    /// * `index` - index of [`Stat`] to be removed from [`Session`]
    pub fn remove(&mut self, index: usize) {
        if index < self.stats.len() {
            self.stats.remove(index);
        }
    }
}
