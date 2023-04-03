use device_query::{DeviceQuery, DeviceState, Keycode};
use std::time::{Instant};

use num_parser::print_number;
mod num_parser;

fn main() {
    let device_state = DeviceState::new();
    let start = Instant::now();

    let mut con = true;
    let mut last= -5.0;

    while con {
        let keys: Vec<Keycode> = device_state.get_keys();
        for key in keys.iter() {
            if key == &Keycode::Space {
                println!("\x1b[H\x1b[J{:.2}", start.elapsed().as_secs_f32());
                con = false;
            }
        }
        let time = (start.elapsed().as_secs_f64() * 1000.0).round();
        if time - 1.0 <= last {
            continue;
        }
        last = time;
        println!("\x1b[H\x1b[J");
        print_number(start.elapsed().as_secs_f64());
    }
}
