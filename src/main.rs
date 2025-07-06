mod game;
mod player;
mod enemy;
mod block;
mod ball;
mod collision;

use colored::Colorize;

use ggez::GameResult;
use ggez::event::{ self };
use crate::game::GameState;

fn main() -> GameResult {
    println!("{}", "-- Rusty Arkanoid started --".blue().bold());

    let (mut ctx, event_loop) = ggez::ContextBuilder::new("rusty_arkanoid", "Piotrek")
        .window_setup(ggez::conf::WindowSetup::default().title("Rusty Arkanoid"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(1200.0, 900.0))
        .add_resource_path("assets")
        .build()
        .expect("Could not create ggez context!");

    let state = GameState::new(&mut ctx)?;

    event::run(ctx, event_loop, state)
}
