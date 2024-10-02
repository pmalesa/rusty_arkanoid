use ggez::graphics::{ self, Color, DrawParam, Mesh, DrawMode };
use ggez::{ Context, GameResult };

pub struct Ball {
    pub position: [f32; 2],
    pub velocity: [f32; 2],
    pub radius: f32,
    pub color: Color,
    circle: Mesh,
}

impl Ball {
    pub const DEFAULT_BALL_RADIUS: f32 = 10.0;
    pub const MIN_BALL_SPEED: f32 = 2.0;
    pub const DEFAULT_BALL_SPEED: f32 = 5.0;
    pub const MAX_BALL_SPEED: f32 = 10.0;

    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let circle = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            [0.0, 0.0],
            Ball::DEFAULT_BALL_RADIUS,
            0.1,
            Color::WHITE,
        )?;

        let window_size = ctx.gfx.drawable_size();

        Ok(Ball {
            position: [window_size.0 / 2.0 - Ball::DEFAULT_BALL_RADIUS, window_size.1 / 2.0 - Ball::DEFAULT_BALL_RADIUS],
            velocity: [0.0, Ball::DEFAULT_BALL_SPEED],
            radius: Ball::DEFAULT_BALL_RADIUS,
            color: Color::WHITE,
            circle,
        })
    }

    pub fn update(&mut self, ctx: &mut Context) {
        self.position[0] += self.velocity[0];
        self.position[1] += self.velocity[1];
    
        let window_size = ctx.gfx.drawable_size();
        if self.position[0] - self.radius <= 0.0 || self.position[0] + self.radius >= window_size.0 {
            self.velocity[0] = -self.velocity[0];
        }

        if self.position[1] - self.radius <= 0.0 || self.position[1] + self.radius >= window_size.1 {
            self.velocity[1] = -self.velocity[1];
        }
    }

    pub fn draw(&self, canvas: &mut graphics::Canvas) {
        let params = DrawParam::default()
            .dest(self.position)
            .color(self.color);

        canvas.draw(&self.circle, params);
    }
}