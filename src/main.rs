use device_query::{DeviceQuery, DeviceState, Keycode};
use std::{time::{Instant}};

use num_parser::print_number;
mod num_parser;

fn main() {
    let device_state = DeviceState::new();
    let start = Instant::now();

    let mut con = true;
    let digits = 1.0;
    let mut last = -digits;

    while con {
        let keys: Vec<Keycode> = device_state.get_keys();
        for key in keys.iter() {
            if key == &Keycode::Space {
                println!("\x1b[H\x1b[J");
                print_number(start.elapsed().as_secs_f64());
                con = false;
            }
        }
        
        let time = (start.elapsed().as_secs_f64() * digits * 10.0).round();
        if time - digits <= last {
            continue;
        }
        last = time;
        println!("\x1b[H\x1b[J");
        print_number(start.elapsed().as_secs_f64());
    }
}
