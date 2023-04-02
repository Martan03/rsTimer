/*use device_query::{DeviceQuery, DeviceState, Keycode};
use std::time::{Instant};

fn main() {
    let device_state = DeviceState::new();
    let start = Instant::now();

    let mut con = true;

    while con {
        let keys: Vec<Keycode> = device_state.get_keys();
        for key in keys.iter() {
            if key == &Keycode::Space {
                println!("\x1b[H\x1b[J{:.2}", start.elapsed().as_secs_f32());
                con = false;
            }
        }
        println!("\x1b[H\x1b[J{:.3}", start.elapsed().as_secs_f64());
    }
}*/

mod digits;
use digits::get_digits;

fn print_digits(digits: &[u8]) {
    let digit_lines = get_digits();

    for line in 0..5 {
        for digit in digits.iter() {
            print!("{}", digit_lines[*digit as usize][line]);
        }
        println!();
    }
}

fn main() {
    print_digits(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]); // prints digits 0, 1, and 2
}

