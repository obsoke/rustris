use ggez::{Context, GameResult, graphics, event};
use ggez::graphics::{DrawMode, Point, Rect};
use std::time::Duration;

mod well;
mod tetromino;
mod shapes;
use self::well::Well;
use self::tetromino::Piece;

const BLOCK_SIZE: f32 = 30.0;

pub struct Position {
    x: u32,
    y: u32,
}

pub struct PlayState {
    well: Well,
}

impl PlayState {
    pub fn new (_: &mut Context) -> GameResult<PlayState> {
        let state = PlayState {
            well: Well::new(),
        };

        Ok(state)
    }
}

impl event::EventHandler for PlayState {
    fn update(&mut self, _: &mut Context, _: Duration) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        self.well.draw(ctx);

        graphics::present(ctx);

        Ok(())
    }
}
