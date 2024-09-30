use ggez::graphics::{ self, Color, DrawParam, Mesh, DrawMode, Rect };
use ggez::{ Context, GameResult };
use ggez::input::keyboard::KeyCode;

pub struct Player {
    pub position: [f32; 2],
    pub size: [f32; 2],
    pub color: Color,
    rectangle: Mesh,
}

impl Player {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let size = [50.0, 50.0];
        let rectangle = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(0.0, 0.0, size[0], size[1]),
            Color::BLUE,
        )?;
        
        Ok(Player {
            position: [575.0, 425.0],
            size,
            color: Color::BLUE,
            rectangle,
        })
    }

    pub fn update(&mut self, ctx: &mut Context) {
        let speed = 5.0;
    
        if ctx.keyboard.is_key_pressed(KeyCode::Up) {
            self.position[1] -= speed;
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Down) {
            self.position[1] += speed;
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Left) {
            self.position[0] -= speed;
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Right) {
            self.position[0] += speed;
        }

        // Clamp positon to window bounds
        self.position[0] = self.position[0].clamp(0.0, 1200.0 - self.size[0]);
        self.position[1] = self.position[1].clamp(0.0, 900.0 - self.size[1]);
    }

    pub fn draw(&self, canvas: &mut graphics::Canvas) { 
        let params = DrawParam::default().dest(self.position);
        canvas.draw(&self.rectangle, params);
    }
}