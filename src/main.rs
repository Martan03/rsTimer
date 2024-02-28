use std::{
    env,
    io::{stdin, stdout, Write},
};

use eyre::Result;
use termint::{
    enums::fg::Fg,
    help,
    widgets::{grad::Grad, span::StrSpanExtension},
};

use crate::{
    gameloop::Game, stats::stats::Stats, stats_manager::StatsManager,
};

mod digits;
mod gameloop;
mod num_parser;
mod scramble;
mod scrambles;
mod stats_manager;
#[allow(clippy::module_inception)]
mod stats {
    pub mod session;
    pub mod stat;
    pub mod stats;
}
mod timer;

fn main() -> Result<()> {
    // Parse arguments
    let mut session = "".to_owned();

    for arg in env::args().skip(1) {
        match arg.as_str() {
            "-a" | "--add" => {
                add_session()?;
                return Ok(());
            }
            "-l" | "--list" => {
                list_sessions()?;
                return Ok(());
            }
            "-h" | "--help" => {
                help();
                return Ok(());
            }
            _ => {
                // Invalid usage if scramble type already specified
                if session != *"" {
                    invalid_usage("multiple sessions provided");
                    std::process::exit(1);
                }
                session = arg;
            }
        }
    }

    // Saves screen, clears screen and hides cursor
    print!("\x1b[?1049h\x1b[2J\x1b[?25l");

    let stats_manager = if session == *"" {
        StatsManager::session_picker()?
    } else {
        StatsManager::open_session(&session)?
    };

    // Start the app
    let mut game = Game::new(stats_manager)?;
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
    stats.add_session(name.trim(), scramble_type.trim())?;

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
    println!(
        "Welcome to help for {} by {}\n",
        "rsTimer".fg(Fg::Green),
        Grad::new("Martan03", (0, 220, 255), (175, 80, 255))
    );
    help!(
        "Usage":
        "\n-h --help" => "Displays this help"
        "\n-l --list" => "Lists all sessions"
        "\n-a --add" => "Opens dialog to add new session"
    );
}

/// Prints invalid usage message
/// * 'msg' - message
fn invalid_usage(msg: &str) {
    eprintln!(
        "{} {msg}. Type {} to display help",
        "Invalid usage:".fg(Fg::Red),
        "rstimer -h".fg(Fg::Yellow)
    );
}
