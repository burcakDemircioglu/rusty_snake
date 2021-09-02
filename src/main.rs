use ggez::{self, event, graphics, GameResult};
mod game;
mod utilities;

fn main() -> GameResult {
    let context_builder = ggez::ContextBuilder::new("Rusty Snake", "BurcakKam");
    let (context, events_loop) = &mut context_builder.build()?;
    graphics::set_window_title(context, "Rusty Snake");

    let mut state = game::GameState::new(context);
    event::run(context, events_loop, &mut state)?;

    Ok(())
}
