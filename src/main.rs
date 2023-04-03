use device_query::{DeviceQuery, DeviceState, Keycode};
use std::{time::{Instant}};

use crate::num_parser::{print_time, get_time};


mod num_parser;

fn main() {
    print!("\x1b[?1049h\x1b[J");
    let device_state = DeviceState::new();

    let mut con = true;
    let decimals = 1.0;
    let mut last = -decimals;
    
    let start = Instant::now();
    while con {
        let keys: Vec<Keycode> = device_state.get_keys();
        for key in keys.iter() {
            if key == &Keycode::Space {
                print_time(get_time(start.elapsed().as_secs_f64(), decimals as usize));
                con = false;
            }
        }
        
        let time = (start.elapsed().as_secs_f64() * decimals * 10.0).round();
        if time - decimals < last {
            continue;
        }

        last = time;
        print_time(get_time(start.elapsed().as_secs_f64(), decimals as usize));
    }
    print!("\x1b[?1049l");
}
