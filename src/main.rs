use crate::gameloop::Gamedata;
use crossterm::Result;

mod digits;
mod gameloop;
mod num_parser;
mod timer;

fn main() -> Result<()> {
    println!("\x1b[?1049h\x1b[H\x1b[J");

    let mut game = Gamedata::new();
    game.start_game()?;

    println!("\x1b[?1049l");

    Ok(())
}
