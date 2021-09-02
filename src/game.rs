use crate::utilities::constants;
use crate::utilities::draw;
use crate::utilities::utility;
use ggez::{self, event, graphics, nalgebra as na, Context};
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

        let mut body_position = std::vec::Vec::new();
        body_position.push(na::Point2::<f32>::new(
            rng.gen_range(0.0, screen_width - constants::BODY_SIZE),
            rng.gen_range(0.0, screen_hight - constants::BODY_SIZE),
        ));

        let mut food_position = std::vec::Vec::new();
        food_position.push(na::Point2::<f32>::new(
            rng.gen_range(0.0, screen_width - constants::FOOD_SIZE),
            rng.gen_range(0.0, screen_hight - constants::FOOD_SIZE),
        ));

        GameState {
            body_positions: body_position,
            food_positions: food_position,
            score: 0,
            body_speed: na::Point2::new(rng.gen_bool(0.5) as i32, rng.gen_bool(0.5) as i32),
        }
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, context: &mut Context) -> ggez::GameResult {
        utility::set_controls(context, &mut self.body_positions);
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> ggez::GameResult {
        draw::draw_body(context, &mut self.body_positions)?;
        draw::draw_foods(context, &mut self.food_positions)?;

        graphics::present(context)?;
        Ok(())
    }
}
