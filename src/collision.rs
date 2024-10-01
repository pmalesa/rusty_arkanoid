use ggez::graphics::Color;

use crate::ball::Ball;
use crate::block::Block;

pub fn ball_block_collision(ball: &Ball, block: &Block) -> bool {
    let closest_point = ball_block_closest_point(ball, block);

    // Calculate the distance between the ball's center and this closest point
    let distance_x = ball.position[0] - closest_point.0;
    let distance_y = ball.position[1] - closest_point.1;

    let distance_squared = distance_x.powi(2) + distance_y.powi(2);

    distance_squared <= ball.radius.powi(2)
}


pub fn reflect_velocity_block(ball: &mut Ball, block: &Block) {
    let closest_point = ball_block_closest_point(ball, block);

    // Calculate the vector from the closest point to the ball's center
    let normal_x = ball.position[0] - closest_point.0;
    let normal_y = ball.position[1] - closest_point.1;

    // Avoid division by zero
    if normal_x == 0.0 && normal_y == 0.0 {
        return
    }

    let length = (normal_x.powi(2) + normal_y.powi(2)).sqrt();
    let nx = normal_x / length;
    let ny = normal_y / length;

    // Dot product of velocity and normal
    let dot = ball.velocity[0] * nx + ball.velocity[1] * ny;

    // Reflect the velocity
    ball.velocity[0] -= 2.0 * dot * nx;
    ball.velocity[1] -= 2.0 * dot * ny;
}

pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

pub fn ball_block_closest_point(ball: &Ball, block: &Block) -> (f32, f32) {
    let closest_x = clamp(ball.position[0], block.position[0], block.position[0] + Block::BLOCK_SIZE);
    let closest_y = clamp(ball.position[1], block.position[1], block.position[1] + Block::BLOCK_SIZE);
    (closest_x, closest_y)
}
