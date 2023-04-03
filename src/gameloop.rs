use std::time::{Instant};

use crate::timer::Timer;
use device_query::{DeviceQuery, DeviceState, Keycode};

use self::num_parser::{print_time, get_time};

#[path = "num_parser.rs"]
mod num_parser;

pub struct Gamedata {
    device_state: DeviceState,
    prev_keys: Vec<Keycode>,
    space_time: Instant,
    timer: Timer,
    con: bool,
}

impl Gamedata {
    pub fn new() -> Gamedata {
        Gamedata {
            device_state: DeviceState::new(),
            prev_keys: Vec::new(),
            space_time: Instant::now(),
            timer: Timer::new(3),
            con: false,
        }
    }

    pub fn start_game(&mut self) {
        self.con = true;
        print_time(get_time(0.0, 3));
        while self.con {
            self.key_listener();
        }
    }

    fn key_listener(&mut self) {
        let keys = self.device_state.get_keys();

        let prev_space = self.prev_keys.contains(&Keycode::Space);
        let space = keys.contains(&Keycode::Space);
        if space && !prev_space
        {
            self.space_time = Instant::now();
        }
        else if !space && prev_space
            && self.space_time.elapsed().as_secs() >= 1
        {
            self.timer.start_timer();
            self.space_time = Instant::now();
        }

        for key in keys.iter() {
            if key == &Keycode::Escape {
                self.con = false;
            }
        }
        self.prev_keys = keys;
    }
}
