use ggez::{
    self, graphics,
    input::keyboard::{self, KeyCode},
    nalgebra as na, Context,
};
use rand::{thread_rng, Rng};

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

pub fn hit_food(
    body_positions: &mut std::vec::Vec<na::Point2<f32>>,
    food_positions: &mut std::vec::Vec<na::Point2<f32>>,
) -> (bool, i32) {
    let head = body_positions[body_positions.len() - 1];

    for food_index in 0..food_positions.len() {
        let food = food_positions[food_index];
        if is_hit(head, food, constants::BODY_SIZE, constants::FOOD_SIZE) {
            return (true, food_index as i32);
        }
    }
    return (false, -1);
}

pub fn hit_snake(body_positions: &mut std::vec::Vec<na::Point2<f32>>) -> bool {
    let head = body_positions[body_positions.len() - 1];

    for body_index in 0..body_positions.len() - 1 {
        let body = body_positions[body_index];
        if is_hit(head, body, constants::BODY_SIZE, constants::BODY_SIZE) {
            return true;
        }
    }
    return false;
}

pub fn is_hit(
    point1: na::Point2<f32>,
    point2: na::Point2<f32>,
    point1_size: f32,
    point2_size: f32,
) -> bool {
    if (point1.x + point1_size / 2.0 > point2.x - point2_size / 2.0
        && point1.x + point1_size / 2.0 < point2.x + point2_size / 2.0
        && point1.y + point1_size / 2.0 > point2.y - point2_size / 2.0
        && point1.y + point1_size / 2.0 < point2.y + point2_size / 2.0)
        || (point1.x - point1_size / 2.0 > point2.x - point2_size / 2.0
            && point1.x - point1_size / 2.0 < point2.x + point2_size / 2.0
            && point1.y - point1_size / 2.0 > point2.y - point2_size / 2.0
            && point1.y - point1_size / 2.0 < point2.y + point2_size / 2.0)
    {
        return true;
    }
    return false;
}

pub fn create_init_body_positions(context: &mut Context) -> std::vec::Vec<na::Point2<f32>> {
    let (screen_width, screen_height) = graphics::drawable_size(context);
    let mut rng = thread_rng();

    let mut body_positions = std::vec::Vec::new();
    body_positions.push(na::Point2::<f32>::new(
        rng.gen_range(0.0, screen_width - constants::BODY_SIZE),
        rng.gen_range(0.0, screen_height - constants::BODY_SIZE),
    ));
    return body_positions;
}
