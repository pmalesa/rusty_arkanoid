use crate::player::Player;
use crate::block::Block;
use crate::ball::Ball;
use crate::collision::{
    collision_ball_block,
    reflect_velocity_block_to_ball,
    collision_ball_player,
    reflect_velocity_player_to_ball,
};

use ggez::event::EventHandler;
use ggez::graphics::{ Canvas, Color };
use ggez::{ Context, GameResult };

pub struct GameState {
    player: Player,
    blocks: Vec<Block>,
    ball: Ball,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let player = Player::new(ctx)?;
        let rows = 8;
        let cols = 25;
        let blocks = GameState::generate_blocks(rows, cols, ctx)?;
        let ball = Ball::new(ctx, ctx.gfx.drawable_size())?;

        Ok(GameState { player, blocks, ball })
    }

    pub fn generate_blocks(rows: usize, cols: usize, ctx: &mut Context) -> GameResult<Vec<Block>> {
        let mut blocks = Vec::with_capacity(rows * cols);
        let padding = 5.0;

        for row in 0..rows {
            for col in 0..cols {
                let x: f32 = col as f32 * (Block::BLOCK_SIZE + padding) + padding;
                let y: f32 = row as f32 * (Block::BLOCK_SIZE + padding) + padding;

                let position = [x, y];
                let block = Block::new(ctx, position)?;
                blocks.push(block);
            }
        }

        Ok(blocks)
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {        
        self.player.update(ctx);
        self.ball.update(ctx);
        for block in &mut self.blocks {
            block.update(ctx);
        }

        // Handle collisions between the ball and the player
        if collision_ball_player(&self.ball, &self.player) {
            reflect_velocity_player_to_ball(&mut self.ball, &self.player);
        }

        // Handle collisions between the ball and the blocks
        for block in self.blocks.iter_mut() {
            if !block.active {
                continue;
            }

            if collision_ball_block(&self.ball, &block) {
                reflect_velocity_block_to_ball(&mut self.ball, &block);
                block.active = false;
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);

        self.player.draw(&mut canvas);
        self.ball.draw(&mut canvas);
        for block in &self.blocks {
            block.draw(&mut canvas);
        }

        canvas.finish(ctx)
    }
}