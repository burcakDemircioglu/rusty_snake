use ggez::{
    self, graphics,
    input::keyboard::{self, KeyCode},
    nalgebra as na, Context,
};

use crate::utilities::constants;
use crate::utilities::utility;

pub fn set_controls(context: &mut Context, body_speed: &mut na::Point2<i32>) {
    if keyboard::is_key_pressed(context, KeyCode::Right) && body_speed.x != -1 {
        body_speed.x = 1;
        body_speed.y = 0;
    } else if keyboard::is_key_pressed(context, KeyCode::Left) && body_speed.x != 1 {
        body_speed.x = -1;
        body_speed.y = 0;
    } else if keyboard::is_key_pressed(context, KeyCode::Down) && body_speed.y != -1 {
        body_speed.x = 0;
        body_speed.y = 1;
    } else if keyboard::is_key_pressed(context, KeyCode::Up) && body_speed.y != 1 {
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
    let head = body_positions[body_positions.len() - 1];
    let mut new_head = na::Point2::new(head.x, head.y);

    let (screen_width, screen_height) = graphics::drawable_size(context);
    let dt = ggez::timer::delta(context).as_secs_f32();

    if body_speed.x < 0 {
        new_head.x -= constants::BODY_SPEED * dt;
    } else if body_speed.x > 0 {
        new_head.x += constants::BODY_SPEED * dt;
    } else if body_speed.y < 0 {
        new_head.y -= constants::BODY_SPEED * dt;
    } else if body_speed.y > 0 {
        new_head.y += constants::BODY_SPEED * dt;
    }

    utility::clamp(&mut new_head, screen_width, screen_height);

    if !elongate {
        body_positions.remove(0);
    }
    body_positions.push(new_head);
}
