use std::{
    env::args,
    io::{stdin, stdout, Write},
};

use eyre::Result;
use termint::{
    enums::fg::Fg,
    help,
    widgets::{grad::Grad, span::StrSpanExtension},
};

use crate::{
    args::ArgParser, game::Game, stats::stats::Stats,
    stats_manager::StatsManager,
};

mod args;
mod asci;
mod game;
mod scramble;
mod stats_manager;
#[allow(clippy::module_inception)]
mod stats {
    pub mod session;
    pub mod stat;
    pub mod stats;
}
mod timer;

fn main() {
    let arg_parser = match ArgParser::parse(args()) {
        Ok(arg_parser) => arg_parser,
        Err(e) => {
            invalid_usage(e.to_string().as_str());
            return;
        }
    };

    if let Err(e) = run(arg_parser) {
        err_print(e.to_string().as_str());
    }
}

/// Runs the app based on arguments
fn run(arg_parser: ArgParser) -> Result<()> {
    if arg_parser.help {
        help();
    } else if arg_parser.add {
        add_session()?;
    } else if arg_parser.list {
        list_sessions()?;
    } else {
        // Saves screen, clears screen and hides cursor
        print!("\x1b[?1049h\x1b[2J\x1b[?25l");

        let res = run_timer(arg_parser.session);

        // Restores screen
        print!("\x1b[?1049l\x1b[?25h");
        stdout().flush()?;
        return res;
    }

    Ok(())
}

/// Runs timer - open session is empty, opens session picker
/// * `arg_parser` - session to be opened
fn run_timer(session: String) -> Result<()> {
    let stats = if session.is_empty() {
        StatsManager::picker()?
    } else {
        StatsManager::open(&session)?
    };

    let mut game = Game::new(stats)?;
    game.start_game()?;

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

/// Prints error message to stderr
/// * `msg` - error message
fn err_print(msg: &str) {
    eprintln!("{} {msg}", "Error:".fg(Fg::Red));
}

/// Prints invalid usage message to stderr
/// * `msg` - error message
fn invalid_usage(msg: &str) {
    eprintln!(
        "{} {msg}. Type {} to display help",
        "Invalid usage:".fg(Fg::Red),
        "rstimer -h".fg(Fg::Yellow)
    );
}
