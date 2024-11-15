use ggez::graphics::{ self, Color, DrawParam, Mesh, DrawMode };
use ggez::{ Context, GameResult };
use rand::Rng;


pub struct Ball {
    pub position: [f32; 2],
    pub velocity: [f32; 2],
    pub radius: f32,
    pub color: Color,    
    pub active: bool,
    pub respawn_timer: f32,
    pub collision_cooldown: f32,

    circle: Mesh,
    window_size: (f32, f32),
    respawn_time: f32,
}

impl Ball {
    pub const DEFAULT_BALL_RADIUS: f32 = 10.0;
    pub const DEFAULT_BALL_SPEED: f32 = 7.0;
    pub const MAX_BALL_SPEED: f32 = 10.0;
    pub const WALL_COLLISION_MARGIN: f32 = 5.0;

    pub fn new(ctx: &mut Context, window_size: (f32, f32)) -> GameResult<Self> {
        let circle = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            [0.0, 0.0],
            Ball::DEFAULT_BALL_RADIUS,
            0.1,
            Color::WHITE,
        )?;

        Ok(Ball {
            position: [window_size.0 / 2.0 - Ball::DEFAULT_BALL_RADIUS, window_size.1 / 2.0 - Ball::DEFAULT_BALL_RADIUS],
            velocity: [0.0, Ball::DEFAULT_BALL_SPEED],
            radius: Ball::DEFAULT_BALL_RADIUS,
            color: Color::WHITE,
            circle,
            window_size,
            respawn_time: 3.0,
            active: true,
            respawn_timer: 0.0,
            collision_cooldown: 0.0
        })
    }

    pub fn update(&mut self, ctx: &mut Context) {
        let delta_time = ctx.time.delta().as_secs_f32();
        if self.active {
            self.position[0] += self.velocity[0];
            self.position[1] += self.velocity[1];
        
            let window_size = ctx.gfx.drawable_size();
            if self.position[0] - self.radius <= Ball::WALL_COLLISION_MARGIN ||
               self.position[0] + self.radius + Ball::WALL_COLLISION_MARGIN >= window_size.0 {
                self.velocity[0] = -self.velocity[0];
            }
    
            if self.position[1] - self.radius <= Ball::WALL_COLLISION_MARGIN ||
               self.position[1] + self.radius + Ball::WALL_COLLISION_MARGIN >= window_size.1 {
                self.velocity[1] = -self.velocity[1];
            }

            // CLAMP FUNCTION HERE (TODO)

            if self.is_outside_window() {
                self.despawn();
            }

            if self.collision_cooldown > 0.0 {
                self.collision_cooldown -= delta_time;
                if self.collision_cooldown < 0.0 {
                    self.collision_cooldown = 0.0;
                }
            }
        } else {
            self.respawn_timer -= delta_time;
            if self.respawn_timer <= 0.0 {
                self.respawn();
            }
        }
    }

    pub fn draw(&self, canvas: &mut graphics::Canvas) {
        let params = DrawParam::default()
            .dest(self.position)
            .color(self.color);

        canvas.draw(&self.circle, params);
    }

    fn is_outside_window(&self) -> bool {
        if self.position[0] < 0.0 || 
           self.position[0] > self.window_size.0 || 
           self.position[1] < 0.0 || 
           self.position[1] > self.window_size.1 
        {
            return true;
        } 
        false
    }

    fn reset_position(&mut self) {
        self.position = [
            self.window_size.0 / 2.0 - Ball::DEFAULT_BALL_RADIUS,
            self.window_size.1 / 2.0 - Ball::DEFAULT_BALL_RADIUS
        ];
    }

    fn reset_velocity(&mut self) {
        self.velocity = [0.0, Ball::DEFAULT_BALL_SPEED]
    }

    fn despawn(&mut self) {
        self.active = false;
        self.respawn_timer = self.respawn_time;
    }

    fn respawn(&mut self) {
        self.reset_position();
        self.reset_velocity();
        self.active = true;
    }
}