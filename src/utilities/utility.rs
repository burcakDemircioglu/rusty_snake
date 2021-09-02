use ggez::{
    self,
    input::keyboard::{self, KeyCode},
    nalgebra as na, Context,
};

use crate::utilities::constants;

pub fn set_controls(context: &mut Context, body_positions: &mut std::vec::Vec<na::Point2<f32>>) {
    let mut queue = body_positions[0];
    let head = body_positions[body_positions.len()-1];

    if keyboard::is_key_pressed(context, KeyCode::Right) {
        queue.x = head.x + constants::BODY_SIZE;
        queue.y = head.y;
        body_positions.remove(0);
        body_positions.push(queue);
    }

    if keyboard::is_key_pressed(context, KeyCode::Left) {
        queue.x = head.x - constants::BODY_SIZE;
        queue.y = head.y;
        body_positions.remove(0);
        body_positions.push(queue);
    }

    if keyboard::is_key_pressed(context, KeyCode::Down) {
        queue.x = head.x;
        queue.y = head.y + constants::BODY_SIZE;
        body_positions.remove(0);
        body_positions.push(queue);
    }

    if keyboard::is_key_pressed(context, KeyCode::Up) {
        queue.x = head.x;
        queue.y = head.y - constants::BODY_SIZE;
        body_positions.remove(0);
        body_positions.push(queue);
    }
}
