use std::{
    env::args,
    io::{stdin, stdout, Write},
};

use app::App;
use args::Action;
use error::Error;

use crate::{args::Args, stats::stats::Stats};

mod app;
mod args;
mod asci;
mod config;
mod error;
mod scramble;
mod sessions;
mod stats;
mod timer;
mod widgets;

fn main() -> Result<(), Error> {
    let args = match Args::parse(args()) {
        Ok(args) => args,
        Err(_) => {
            println!("Error parsing arguments");
            return Ok(());
        }
    };

    run(args)
}

/// Runs the app based on arguments
fn run(args: Args) -> Result<(), Error> {
    match args.action {
        Some(Action::Add) => add_session(),
        Some(Action::Help) => Ok(Args::help()),
        Some(Action::List) => Ok(list_sessions()),
        None => run_timer(args.session),
    }
}

/// Starts app - if session is None, it opens session picker
fn run_timer(session: Option<String>) -> Result<(), Error> {
    let mut app = match session {
        Some(session) => App::open(session),
        None => App::new(),
    };
    app.run()
}

/// Add session prompt
fn add_session() -> Result<(), Error> {
    println!("Adding session. Please fill out the prompt.");
    print!("Session name: ");
    stdout().flush()?;
    let mut name = String::new();
    stdin().read_line(&mut name)?;

    print!("Scramble type: ");
    stdout().flush()?;
    let mut scramble_type = String::new();
    stdin().read_line(&mut scramble_type)?;

    let mut stats = Stats::load();
    stats.add_session(name.trim(), scramble_type.trim())?;

    stats.save()?;

    Ok(())
}

/// Lists all sessions
fn list_sessions() {
    let stats = Stats::load();
    stats.print_sessions();
}

// /// Prints error message to stderr
// /// * `msg` - error message
// fn err_print(msg: &str) {
//     eprintln!("{} {msg}", "Error:".fg(Color::Red));
// }

// /// Prints invalid usage message to stderr
// /// * `msg` - error message
// fn invalid_usage(msg: &str) {
//     eprintln!(
//         "{} {msg}. Type {} to display help",
//         "Invalid usage:".fg(Color::Red),
//         "rstimer -h".fg(Color::Yellow)
//     );
// }
