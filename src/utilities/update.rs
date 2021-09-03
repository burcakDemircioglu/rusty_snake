use ggez::{
    self, graphics,
    input::keyboard::{self, KeyCode},
    nalgebra as na, Context,
};

use crate::utilities::constants;
use crate::utilities::utility;

pub fn set_controls(context: &mut Context, body_speed: &mut na::Point2<i32>) {
    if keyboard::is_key_pressed(context, KeyCode::Right) {
        body_speed.x = 1;
        body_speed.y = 0;
    } else if keyboard::is_key_pressed(context, KeyCode::Left) {
        body_speed.x = -1;
        body_speed.y = 0;
    } else if keyboard::is_key_pressed(context, KeyCode::Down) {
        body_speed.x = 0;
        body_speed.y = 1;
    } else if keyboard::is_key_pressed(context, KeyCode::Up) {
        body_speed.x = 0;
        body_speed.y = -1;
    }
}

pub fn move_snake(
    context: &mut Context,
    body_positions: &mut std::vec::Vec<na::Point2<f32>>,
    body_speed: &mut na::Point2<i32>,
    elongate: bool,
) {
    let mut queue = na::Point2::new(body_positions[0].x, body_positions[0].y);
    let head = body_positions[body_positions.len() - 1];
    let (screen_width, screen_height) = graphics::drawable_size(context);
    let dt = ggez::timer::delta(context).as_secs_f32();

    if body_speed.x < 0 {
        queue.x = head.x - (constants::BODY_SPEED * dt);
        queue.y = head.y;
    } else if body_speed.x > 0 {
        queue.x = head.x + (constants::BODY_SPEED * dt);
        queue.y = head.y;
    } else if body_speed.y < 0 {
        queue.x = head.x;
        queue.y = head.y - (constants::BODY_SPEED * dt);
    } else if body_speed.y > 0 {
        queue.x = head.x;
        queue.y = head.y + (constants::BODY_SPEED * dt);
    } else {
        return;
    }

    utility::clamp(&mut queue, screen_width, screen_height);

    if !elongate {
        body_positions.remove(0);
    }
    body_positions.push(queue);
}
