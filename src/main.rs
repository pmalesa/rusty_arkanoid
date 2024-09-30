mod game;
mod player;
mod enemy;
mod block;

use colored::Colorize;

use ggez::GameResult;
use ggez::event::{ self };
use crate::game::GameState;

fn main() -> GameResult {
    println!("{}", "-- Rust game started --".blue().bold());

    let (mut ctx, event_loop) = ggez::ContextBuilder::new("rust_game", "Piotrek")
        .window_setup(ggez::conf::WindowSetup::default().title("Rust Game"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(1200.0, 900.0))
        .build()
        .expect("Could not create ggez context!");

    let state = GameState::new(&mut ctx)?;

    event::run(ctx, event_loop, state)
}
