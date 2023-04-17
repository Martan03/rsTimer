use std::env;

use eyre::Result;

use crate::{gameloop::Game, scrambles::get_scramble};

mod digits;
mod gameloop;
mod num_parser;
mod scramble;
mod scrambles;
mod stats;
mod timer;

fn main() -> Result<()> {
    // Parse arguments
    let mut scramble_type = "".to_owned();

    for arg in env::args().skip(1) {
        match arg.as_str() {
            "-h" => help(),
            _ => {
                // Invalid usage if scramble type already specified
                if scramble_type != "".to_owned() {
                    invalid_usage("multiple scramble types");
                    std::process::exit(1);
                }
                scramble_type = arg;
            }
        }
    }
    // Sets default scramble type 3x3x3
    if scramble_type == "".to_owned() {
        scramble_type = "3x3x3".to_owned();
    }

    // Checks if scramble type exists
    let (len, moves) = get_scramble(&scramble_type);
    if len == 0 {
        invalid_usage("non-existing scramble type");
        std::process::exit(1);
    }

    // Saves screen, clears screen and hides cursor
    print!("\x1b[?1049h\x1b[2J\x1b[?25l");

    // Start the app
    let mut game = Game::new(len, moves)?;
    game.start_game()?;

    // Restores screen
    print!("\x1b[?1049l\x1b[?25h");

    Ok(())
}

/// Displays help
fn help() {
    println!("Help not implemented yet");
}

/// Prints invalid usage message
/// * 'msg' - message
fn invalid_usage(msg: &str) {
    eprint!("\x1b[91mInvalid usage:\x1b[0m {msg}. ");
    eprintln!("Type \x1b[93mtimer -h\x1b[0m to display help");
}
