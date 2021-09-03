use std::convert::TryInto;

use crate::utilities::constants;
use crate::utilities::draw;
use crate::utilities::update;
use crate::utilities::utility;
use crate::utilities::utility::hit_snake;
use ggez::{
    self, event, graphics,
    input::keyboard::{self, KeyCode},
    nalgebra as na, Context,
};
use rand::{thread_rng, Rng};

pub struct GameState {
    body_speed: na::Point2<i32>,
    body_positions: std::vec::Vec<na::Point2<f32>>,
    food_positions: std::vec::Vec<na::Point2<f32>>,
    score: i32,
}
impl GameState {
    pub fn new(context: &mut Context) -> Self {
        let mut rng = thread_rng();
        let (screen_width, screen_hight) = graphics::drawable_size(context);

        let mut food_position = std::vec::Vec::new();
        food_position.push(na::Point2::<f32>::new(
            rng.gen_range(0.0, screen_width - constants::FOOD_SIZE),
            rng.gen_range(0.0, screen_hight - constants::FOOD_SIZE),
        ));

        GameState {
            body_positions: utility::create_init_body_positions(context),
            food_positions: food_position,
            score: 0,
            body_speed: na::Point2::new(rng.gen_bool(0.5) as i32, rng.gen_bool(0.5) as i32),
        }
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, context: &mut Context) -> ggez::GameResult {
        let mut rng = thread_rng();
        let (screen_width, screen_hight) = graphics::drawable_size(context);

        let (hit, eaten_food_index) =
            utility::hit_food(&mut self.body_positions, &mut self.food_positions);
        if hit {
            self.score += 1;
            self.food_positions
                .remove(eaten_food_index.try_into().unwrap());

            self.food_positions.push(na::Point2::<f32>::new(
                rng.gen_range(0.0, screen_width - constants::FOOD_SIZE),
                rng.gen_range(0.0, screen_hight - constants::FOOD_SIZE),
            ));
        }

        update::set_controls(context, &mut self.body_speed);
        update::move_snake(context, &mut self.body_positions, &mut self.body_speed, hit);
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> ggez::GameResult {
        graphics::clear(context, graphics::BLACK);

        if hit_snake(&mut self.body_positions) {
            draw::draw_game_over_screen(context, self.score)?;

            if keyboard::is_key_pressed(context, KeyCode::Space) {
                self.score = 0;
                self.body_positions = utility::create_init_body_positions(context);
            }
            return Ok(());
        }

        draw::draw_body(context, &mut self.body_positions)?;
        draw::draw_foods(context, &mut self.food_positions)?;
        draw::draw_score_board(context, self.score)?;

        graphics::present(context)?;
        Ok(())
    }
}
