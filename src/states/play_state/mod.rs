use ggez::{Context, GameResult, graphics, event};
use std::time::Duration;

mod well;
mod tetromino;
mod shapes;
mod bag;
mod util;
use self::well::Well;
use self::tetromino::Piece;
use self::bag::PieceBag;

const BLOCK_SIZE: f32 = 30.0;
const FALL_SPEED: f64 = 0.5;

#[derive(Copy, Clone, Debug)]
pub struct Position {
    x: u32,
    y: u32,
}

pub struct PlayState {
    well: Well,
    bag: PieceBag,
    current_piece: Piece,

    fall_timer: f64,
    score: u32,
    cleared_lines: u16,
    game_over: bool,
}

impl PlayState {
    pub fn new (_: &mut Context) -> GameResult<PlayState> {
        let mut bag = PieceBag::new();
        let first_piece = bag.take_piece();
        let state = PlayState {
            well: Well::new(),
            bag: bag,
            current_piece: first_piece,

            fall_timer: 0.0,
            score: 0,
            cleared_lines: 0,
            game_over: false,
        };

        Ok(state)
    }
}

impl event::EventHandler for PlayState {
    fn update(&mut self, _: &mut Context, dt: Duration) -> GameResult<()> {
        use self::util::DurationExt;

        if self.game_over {
            // do game over stuff
            return Ok(());
        }

        self.fall_timer += dt.subsec_nanos() as f64 / 1_000_000_000.0;

        // get the shape of our current piece - used in collision calculations
        // in this loop iteration
        let current_shape = self.current_piece.get_shape();

        // GRAVITY - if fall timer threshold has been hit, move current piece down by one
        // row & reset timer
        if self.fall_timer >= FALL_SPEED {
            self.fall_timer = 0.0;
            self.current_piece.potential_top_left.y += 1;

            let did_land = self.well.check_for_landing(
                &current_shape,
                &self.current_piece.potential_top_left
            );

            if did_land {
                if self.current_piece.top_left.y < 2 {
                    println!("game over!");
                    self.game_over = true;
                    return Ok(());
                }

                // game isn't over - take another piece and move to next frame
                self.well.land(&self.current_piece);
                self.current_piece = self.bag.take_piece();
                return Ok(());
            }

            // piece did not land - advance!
            self.current_piece.top_left = self.current_piece.potential_top_left;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::set_background_color(ctx, graphics::Color::new(0.0, 0.0, 0.0, 255.0));
        graphics::clear(ctx);

        self.well.draw(ctx)?;
        self.current_piece.draw_shadow(ctx, &Position { x: 0, y: 0 })?;
        self.current_piece.draw(ctx)?;

        graphics::present(ctx);

        Ok(())
    }
}
