use crate::gameloop::Game;
use crossterm::Result;

mod digits;
mod gameloop;
mod num_parser;
mod timer;
mod scramble;
mod scrambles;

fn main() -> Result<()> {
    println!("\x1b[?1049h\x1b[H\x1b[J\x1b[?25l");

    let mut game = Game::new("3x3".to_owned());
    game.start_game()?;

    println!("\x1b[?1049l");

    Ok(())
}
