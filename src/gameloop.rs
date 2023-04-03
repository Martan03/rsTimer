use crate::timer::Timer;
use device_query::{DeviceQuery, DeviceState, Keycode};

pub struct Gamedata {
    device_state: DeviceState,
    prev_keys: Vec<Keycode>,
    timer: Timer,
    con: bool,
}

impl Gamedata {
    pub fn new() -> Gamedata {
        Gamedata {
            device_state: DeviceState::new(),
            prev_keys: Vec::new(),
            timer: Timer::new(3),
            con: false,
        }
    }

    pub fn start_game(&mut self) {
        self.con = true;
        while self.con {
            self.key_listener();
        }
    }

    fn key_listener(&mut self) {
        let keys = self.device_state.get_keys();
        for key in keys.iter() {
            if !keys.contains(&Keycode::Space)
                && self.prev_keys.contains(&Keycode::Space)
            {
                self.timer.start_timer();
            }
            if key == &Keycode::Escape {
                self.con = false;
            }
        }
        self.prev_keys = keys;
    }
}
