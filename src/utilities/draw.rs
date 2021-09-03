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

pub fn draw_game_over_screen(context: &mut Context, score: i32) -> Result<(), GameError> {
    let mut game_over_text = Text::new(format!("Game Over!"));
    game_over_text.set_font(graphics::Font::default(), graphics::Scale::uniform(40.0));

    let (screen_width, screen_height) = graphics::drawable_size(context);
    let screen_width_half = screen_width * 0.5;
    let screen_height_half = screen_height * 0.5;
    let (game_over_text_w, game_over_text_h) = game_over_text.dimensions(context);
    let game_over_pos = na::Point2::new(
        screen_width_half - (game_over_text_w / 2) as f32,
        screen_height_half,
    );

    let mut draw_param = graphics::DrawParam::default();
    draw_param.dest = game_over_pos.into();
    graphics::draw(context, &game_over_text, draw_param)?;

    let mut start_again_text = Text::new(format!("Press space to start again."));
    start_again_text.set_font(graphics::Font::default(), graphics::Scale::uniform(20.0));

    let (screen_width, screen_height) = graphics::drawable_size(context);
    let screend_width_half = screen_width * 0.5;
    let screen_height_half = screen_height * 0.5;
    let start_again_text_w = start_again_text.dimensions(context).0;
    let start_again_pos = na::Point2::new(
        screend_width_half - (start_again_text_w / 2) as f32,
        screen_height_half + game_over_text_h as f32 + 10.0,
    );

    let mut draw_param = graphics::DrawParam::default();
    draw_param.dest = start_again_pos.into();
    graphics::draw(context, &start_again_text, draw_param)
}