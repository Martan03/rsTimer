use std::{
    env,
    io::{stdin, stdout, Write},
    time::Duration,
};

use eyre::Result;

use crate::{
    gameloop::Game,
    scrambles::get_scramble,
    stats::{stat::Stat, stats::Stats},
};

mod digits;
mod gameloop;
mod num_parser;
mod scramble;
mod scrambles;
mod stats_manager;
mod stats {
    pub mod session;
    pub mod stat;
    pub mod stats;
}
mod timer;

fn main() -> Result<()> {
    // Parse arguments
    let mut scramble_type = "".to_owned();

    for arg in env::args().skip(1) {
        match arg.as_str() {
            "-a" => add_session()?,
            "-l" => {
                list_sessions()?;
                return Ok(());
            }
            "-h" => {
                help();
                return Ok(());
            }
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

/// Add session prompt
fn add_session() -> Result<()> {
    println!("Adding session. Please fill out the prompt.");
    print!("Session name: ");
    stdout().flush()?;
    let mut name = String::new();
    stdin().read_line(&mut name)?;

    print!("Scramble type: ");
    stdout().flush()?;
    let mut scramble_type = String::new();
    stdin().read_line(&mut scramble_type)?;

    let mut stats = Stats::load()?;
    stats.add_session(&name.trim(), &scramble_type.trim())?;

    stats.save()?;

    Ok(())
}

/// Lists all sessions
fn list_sessions() -> Result<()> {
    let stats = Stats::load()?;
    stats.print_sessions();

    Ok(())
}

/// Displays help
fn help() {
    print!("Welcome to help for \x1b[92mrsTimer\x1b[0m by ");
    // Prints name with color gradient
    let name = "Martan03";
    let r = 0;
    let g = 220;
    for i in 0..name.len() {
        print!(
            "\x1b[38;2;{};{};255m{}",
            r + i * 25,
            g - i * 20,
            name.chars().nth(i).unwrap()
        );
    }
    println!("\n\n\x1b[92mUsage: \x1b[97mrstimer \x1b[90m[flags]\n");
    println!("\x1b[92mFlags:\n\x1b[0m");
    println!("\x1b[93m  -h --help");
    println!("\x1b[0m    Displays help\n");
    println!("\x1b[93m  -l");
    println!("\x1b[0m    Lists all sessions\n");
    println!("\x1b[93m  -a");
    println!("\x1b[0m    Opens dialog to add session\n");
}

/// Prints invalid usage message
/// * 'msg' - message
fn invalid_usage(msg: &str) {
    eprint!("\x1b[91mInvalid usage:\x1b[0m {msg}. ");
    eprintln!("Type \x1b[93mtimer -h\x1b[0m to display help");
}
