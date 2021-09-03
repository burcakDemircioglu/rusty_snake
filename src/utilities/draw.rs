use ggez::{
    self,
    graphics::{self, DrawMode, DrawParam, Mesh, Text},
    nalgebra as na, Context, GameError,
};

use crate::utilities::constants;

pub fn draw_body(
    context: &mut Context,
    body_positions: &mut std::vec::Vec<na::Point2<f32>>,
) -> Result<(), GameError> {
    for body_part in body_positions {
        let bounds = graphics::Rect::new(
            body_part.x,
            body_part.y,
            constants::BODY_SIZE,
            constants::BODY_SIZE,
        );
        let body_part_mesh =
            Mesh::new_rectangle(context, DrawMode::fill(), bounds, graphics::WHITE)?;
        graphics::draw(context, &body_part_mesh, DrawParam::default())?;
    }
    Ok(())
}

pub fn draw_foods(
    context: &mut Context,
    food_positions: &mut std::vec::Vec<na::Point2<f32>>,
) -> Result<(), GameError> {
    for food in food_positions {
        let bounds =
            graphics::Rect::new(food.x, food.y, constants::FOOD_SIZE, constants::FOOD_SIZE);
        let food_mesh = Mesh::new_rectangle(context, DrawMode::fill(), bounds, graphics::WHITE)?;
        graphics::draw(context, &food_mesh, DrawParam::default())?;
    }
    Ok(())
}

pub fn draw_score_board(context: &mut Context, score: i32) -> Result<(), GameError> {
    let text = Text::new(format!("Score: {}", score));
    let mut draw_param = graphics::DrawParam::default();
    draw_param.dest = na::Point2::new(constants::SCORE_PADDING, constants::SCORE_PADDING).into();
    graphics::draw(context, &text, draw_param)?;
    Ok(())
}
