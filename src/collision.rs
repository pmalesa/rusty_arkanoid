use ggez::graphics::Color;

use crate::ball::Ball;
use crate::block::Block;
use crate::player::Player;

pub const COLLISION_MARGIN: f32 = 10.0;
pub const COLLISION_COOLDOWN_TIME: f32 = 0.05;

pub fn collision_ball_block(ball: &Ball, block: &Block) -> bool {
    if !ball.active {
        return false;
    }

    let closest_point = closest_point(ball, block.position, [Block::BLOCK_SIZE, Block::BLOCK_SIZE]);

    // Calculate the distance between the ball's center and this closest point
    let distance_x = ball.position[0] - closest_point.0;
    let distance_y = ball.position[1] - closest_point.1;

    let distance_squared = distance_x.powi(2) + distance_y.powi(2);

    distance_squared + COLLISION_MARGIN <= ball.radius.powi(2)
}

pub fn reflect_velocity_block_to_ball(ball: &mut Ball, block: &Block) {
    if !ball.active || !block.active {
        return;
    }

    if ball.collision_cooldown > 0.0 {
        return;        
    }
    ball.collision_cooldown = COLLISION_COOLDOWN_TIME;

    let closest_point = closest_point(ball, block.position, [Block::BLOCK_SIZE, Block::BLOCK_SIZE]);

    // Calculate the vector from the closest point to the ball's center
    let normal_x = ball.position[0] - closest_point.0;
    let normal_y = ball.position[1] - closest_point.1;

    // Avoid division by zero
    if normal_x == 0.0 && normal_y == 0.0 {
        return
    }

    // Normalize the normal vector
    let length = (normal_x.powi(2) + normal_y.powi(2)).sqrt();
    let nx = normal_x / length;
    let ny = normal_y / length;

    // Dot product of velocity and normal
    let dot = ball.velocity[0] * nx + ball.velocity[1] * ny;

    // Reflect the velocity
    ball.velocity[0] -= 2.0 * dot * nx;
    ball.velocity[1] -= 2.0 * dot * ny;

    clamp_ball_velocity(ball);

    // Position correction to prevent the ball from sticking to the player
    let penetration_depth = ball.radius - length;
    if penetration_depth > 0.0 {
        ball.position[0] += nx * penetration_depth;
        ball.position[1] += ny * penetration_depth;
    }
}

pub fn collision_ball_player(ball: &Ball, player: &Player) -> bool {
    if !ball.active {
        return false;
    }

    let closest_point: (f32, f32) = closest_point(ball, player.position, player.size);

    // Calculate the distance between the ball's center and this closest point
    let distance_x = ball.position[0] - closest_point.0;
    let distance_y = ball.position[1] - closest_point.1;

    let distance_squared = distance_x.powi(2) + distance_y.powi(2);

    distance_squared + COLLISION_MARGIN <= ball.radius.powi(2)
}

pub fn reflect_velocity_player_to_ball(ball: &mut Ball, player: &Player) {
    if !ball.active {
        return;
    }

    if ball.collision_cooldown > 0.0 {
        return;        
    }
    ball.collision_cooldown = COLLISION_COOLDOWN_TIME;

    let closest_point: (f32, f32) = closest_point(ball, player.position, player.size);

    // Calculate the vector from the closest point to the ball's center
    let normal_x = ball.position[0] - closest_point.0;
    let normal_y = ball.position[1] - closest_point.1;

    // Avoid division by zero
    if normal_x == 0.0 && normal_y == 0.0 {
        ball.velocity[0] = -ball.velocity[0];
        ball.velocity[1] = -ball.velocity[1];
        return
    }

    // Normalize the normal vector
    let length = (normal_x.powi(2) + normal_y.powi(2)).sqrt();
    let nx = normal_x / length;
    let ny = normal_y / length;

    // Calculate relative velocity
    let rel_vx = ball.velocity[0] - player.velocity[0];
    let rel_vy = ball.velocity[1] - player.velocity[1];

    // Dot product of relative velocity and normal
    let dot = rel_vx * nx + rel_vy * ny;

    // Reflect the relative velocity across the normal
    let reflected_rel_vx = rel_vx - 2.0 * dot * nx;
    let reflected_rel_vy = rel_vy - 2.0 * dot * ny;

    // Update the ball's absolute velocity
    ball.velocity[0] = reflected_rel_vx + player.velocity[0];
    ball.velocity[1] = reflected_rel_vy + player.velocity[1];

    clamp_ball_velocity(ball);

    // // Position correction to prevent the ball from sticking to the player
    let penetration_depth = (ball.radius - length).max(0.0);
    if penetration_depth > 0.0 {
        let correction_factor = 1.0;
        ball.position[0] += correction_factor * nx * penetration_depth;
        ball.position[1] += correction_factor * ny * penetration_depth;
    }
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

pub fn clamp_ball_velocity(ball: &mut Ball) {
    let speed = (ball.velocity[0].powi(2) + ball.velocity[1].powi(2)).sqrt();
    if speed < Ball::DEFAULT_BALL_SPEED {
        let factor = Ball::DEFAULT_BALL_SPEED / speed;
        ball.velocity[0] *= factor;
        ball.velocity[1] *= factor;
    } else if speed > Ball::MAX_BALL_SPEED {
        let factor = Ball::MAX_BALL_SPEED / speed;
        ball.velocity[0] *= factor;
        ball.velocity[1] *= factor;    
    }
}

pub fn closest_point(ball: &Ball, obj_position: [f32; 2], obj_size: [f32; 2]) -> (f32, f32) {
    let closest_x = clamp(ball.position[0], obj_position[0], obj_position[0] + obj_size[0]);
    let closest_y = clamp(ball.position[1], obj_position[1], obj_position[1] + obj_size[1]);
    (closest_x, closest_y)
}