use ggez::{Context, GameResult, graphics, event};
use ggez::graphics::{DrawMode, Point, Rect};
use std::time::Duration;

pub struct PlayState {}

impl PlayState {
    pub fn new (ctx: &mut Context) -> GameResult<PlayState> {
        let state = PlayState { };
        Ok(state)
    }
}

impl event::EventHandler for PlayState {
    fn update(&mut self, ctx: &mut Context, dt: Duration) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        graphics::circle(ctx, DrawMode::Fill, Point {x: 200.0, y: 200.0}, 100.0, 32)?;
        graphics::rectangle(ctx, DrawMode::Fill, Rect{ x: 0.0, y: 0.0, w: 100.0, h: 100.0 })?;

        graphics::present(ctx);

        Ok(())
    }
}
