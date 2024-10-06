use ggez::graphics::{ self, Color, DrawParam, Mesh, DrawMode, Rect };
use ggez::{ Context, GameResult };
use rand::Rng;

pub struct Block {
    pub position: [f32; 2],
    pub color: Color,
    pub active: bool,
    rectangle: Mesh,

    time_since_last_change: f32,
    next_change_time: f32,
}

impl Block {
    pub const BLOCK_SIZE: f32 = 40.0;

    pub fn new(ctx: &mut Context, position: [f32; 2]) -> GameResult<Self> {
        let color = Self::generate_color();
        let rectangle = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(0.0, 0.0, Block::BLOCK_SIZE, Block::BLOCK_SIZE),
            color,
        )?;

        let mut rng = rand::thread_rng();
        let next_change_time = rng.gen_range(0.1..=1.0);

        Ok(Block {
            position: position,
            color: color,
            active: true,
            rectangle,

            time_since_last_change: 0.0,
            next_change_time,
        })
    }

    pub fn update(&mut self, ctx: &mut Context) {
        if !self.active {
            return;
        }

        let delta_time = ctx.time.delta().as_secs_f32();
        self.time_since_last_change += delta_time;

        if self.time_since_last_change >= self.next_change_time {
            self.color = Self::generate_color();
            self.time_since_last_change = 0.0;
            let mut rng = rand::thread_rng();
            self.next_change_time = rng.gen_range(0.1..=1.0);
        }
        
    }

    pub fn draw(&self, canvas: &mut graphics::Canvas) {
        if !self.active {
            return;
        }

        let params = DrawParam::default()
            .dest(self.position)
            .color(self.color);

        canvas.draw(&self.rectangle, params);
    }

    pub fn generate_color() -> Color {
        let mut rng = rand::thread_rng();
        let r: u8 = rng.gen();
        let g: u8 = rng.gen();
        let b: u8 = rng.gen();
        Color::from_rgb(r, g, b)
    }
}