use device_query::{DeviceQuery, DeviceState, Keycode};
use std::{
    ops::Sub,
    time::{Duration, Instant},
};

use crate::num_parser::{get_time, print_time};

#[path = "num_parser.rs"]
mod num_parser;

pub struct Timer {
    device_state: DeviceState,
    prev_keys: Vec<Keycode>,
    decimals: usize,
    time: Duration,
    con: bool,
}

impl Timer {
    pub fn new(decimals: usize) -> Self {
        Timer {
            device_state: DeviceState::new(),
            prev_keys: Vec::new(),
            decimals,
            time: Duration::new(0, 0),
            con: false,
        }
    }

    pub fn start_timer(&mut self) {
        let mut last = Duration::new(0, 0);
        let start = Instant::now();
        while self.con {
            self.key_listener();

            let time = start.elapsed();
            if (time.sub(last).as_millis() as i128 - self.decimals as i128) < 0
            {
                continue;
            }

            last = time;
            print_time(get_time(time.as_secs_f64(), self.decimals));
        }

        self.time = start.elapsed();
        print_time(get_time(start.elapsed().as_secs_f64(), self.decimals));
    }

    fn key_listener(&mut self) {
        let keys = self.device_state.get_keys();
        self.prev_keys = keys;
        for key in self.prev_keys.iter() {
            if key == &Keycode::Space {
                self.con = false;
            }
        }
    }
}
