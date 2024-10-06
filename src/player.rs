use ggez::graphics::{ self, Color, DrawParam, Mesh, DrawMode, Rect };
use ggez::{ Context, GameResult };
use ggez::input::keyboard::KeyCode;

pub struct Player {
    pub position: [f32; 2],
    pub velocity: [f32; 2],
    pub size: [f32; 2],
    pub color: Color,
    rectangle: Mesh,
}

impl Player {
    pub const DEFAULT_SPEED: f32 = 6.0;
    pub const DEFAULT_HEIGHT: f32 = 25.0;
    pub const DEFAULT_WIDTH: f32 = 100.0;

    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let size = [Player::DEFAULT_WIDTH, Player::DEFAULT_HEIGHT];
        let rectangle = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(0.0, 0.0, size[0], size[1]),
            Color::from_rgb(8, 166, 209),
        )?;
        
        let window_size = ctx.gfx.drawable_size();

        Ok(Player {
            position: [window_size.0 / 2.0 - 50.0, window_size.1 * 0.9],
            velocity: [0.0, 0.0],
            size,
            color: Color::BLUE,
            rectangle,
        })
    }

    pub fn update(&mut self, ctx: &mut Context) {
        if ctx.keyboard.is_key_pressed(KeyCode::Up) {
            self.position[1] -= Player::DEFAULT_SPEED;
            self.velocity[1] = -Player::DEFAULT_SPEED;
        } else {
            self.velocity[1] = 0.0;
        }

        if ctx.keyboard.is_key_pressed(KeyCode::Down) {
            self.position[1] += Player::DEFAULT_SPEED;
            self.velocity[1] = Player::DEFAULT_SPEED;
        } else {
            self.velocity[1] = 0.0;
        }

        if ctx.keyboard.is_key_pressed(KeyCode::Left) {
            self.position[0] -= Player::DEFAULT_SPEED;
            self.velocity[0] = -Player::DEFAULT_SPEED;
        } else {
            self.velocity[0] = 0.0;
        }

        if ctx.keyboard.is_key_pressed(KeyCode::Right) {
            self.position[0] += Player::DEFAULT_SPEED;
            self.velocity[0] = Player::DEFAULT_SPEED;
        } else {
            self.velocity[0] = 0.0;
        }

        // Clamp positon to window bounds
        self.position[0] = self.position[0].clamp(0.0, ctx.gfx.drawable_size().0 - self.size[0]);
        self.position[1] = self.position[1].clamp(ctx.gfx.drawable_size().1 * 0.8, ctx.gfx.drawable_size().1 - self.size[1]);
    }

    pub fn draw(&self, canvas: &mut graphics::Canvas) { 
        let params = DrawParam::default().dest(self.position);
        canvas.draw(&self.rectangle, params);
    }
}