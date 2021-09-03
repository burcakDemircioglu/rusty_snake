use ggez::{
    self,
    input::keyboard::{self, KeyCode},
    nalgebra as na, Context,
};

use crate::utilities::constants;

pub fn clamp(position: &mut na::Point2<f32>, max_width: f32, max_height: f32) {
    if position.x < 0.0 - (constants::BODY_SIZE / 2.0) {
        position.x += max_width;
    } else if position.x > max_width + (constants::BODY_SIZE / 2.0) {
        position.x -= max_width;
    }

    if position.y < 0.0 - (constants::BODY_SIZE / 2.0) {
        position.y += max_height;
    } else if position.y > max_height + (constants::BODY_SIZE / 2.0) {
        position.y -= max_height;
    }
}

pub fn is_hit(
    body_positions: &mut std::vec::Vec<na::Point2<f32>>,
    food_positions: &mut std::vec::Vec<na::Point2<f32>>,
) -> (bool, i32) {
    let head = body_positions[body_positions.len() - 1];

    for food_index in 0..food_positions.len() {
        let food = food_positions[food_index];
        if (head.x + constants::BODY_SIZE / 2.0 > food.x - constants::FOOD_SIZE / 2.0
            && head.x + constants::BODY_SIZE / 2.0 < food.x + constants::FOOD_SIZE / 2.0
            && head.y + constants::BODY_SIZE / 2.0 > food.y - constants::FOOD_SIZE / 2.0
            && head.y + constants::BODY_SIZE / 2.0 < food.y + constants::FOOD_SIZE / 2.0)
            || (head.x - constants::BODY_SIZE / 2.0 > food.x - constants::FOOD_SIZE / 2.0
                && head.x - constants::BODY_SIZE / 2.0 < food.x + constants::FOOD_SIZE / 2.0
                && head.y - constants::BODY_SIZE / 2.0 > food.y - constants::FOOD_SIZE / 2.0
                && head.y - constants::BODY_SIZE / 2.0 < food.y + constants::FOOD_SIZE / 2.0)
        {
            return (true, food_index as i32);
        }
    }
    return (false, -1);
}
