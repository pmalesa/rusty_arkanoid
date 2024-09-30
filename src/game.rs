use crate::player::Player;
use crate::block::Block;

use ggez::event::EventHandler;
use ggez::graphics::{ Canvas, Color, DrawMode, DrawParam, Mesh, Rect };
use ggez::{ Context, GameResult };

pub struct GameState {
    player: Player,
    blocks: Vec<Block>,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let player = Player::new(ctx)?;
        let rows = 16;
        let cols = 40;
        let blocks = GameState::generate_blocks(rows, cols, ctx)?;

        Ok(GameState { player, blocks })
    }

    pub fn generate_blocks(rows: usize, cols: usize, ctx: &mut Context) -> GameResult<Vec<Block>> {
        let mut blocks = Vec::with_capacity(rows * cols);
        let padding = 5.0;

        for row in 0..rows {
            for col in 0..cols {
                let x = col as f32 * (Block::BLOCK_SIZE + padding) + padding;
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
        for block in &mut self.blocks {
            block.update(ctx);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);

        for block in &self.blocks {
            block.draw(&mut canvas);
        }

        self.player.draw(&mut canvas);

        canvas.finish(ctx)
    }
}