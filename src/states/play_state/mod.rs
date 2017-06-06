use ggez::{Context, GameResult, graphics, event};
use std::time::Duration;

mod well;
mod tetromino;
mod shapes;
mod bag;
mod util;

use ggez::event::*;
use self::well::Well;
use self::tetromino::Piece;
use self::bag::PieceBag;
use self::util::DurationExt;

const BLOCK_SIZE: f32 = 30.0;
const FALL_SPEED: f64 = 0.5;
const INPUT_DELAY_TIME: f64 = 0.05;

#[derive(Copy, Clone, Debug)]
pub struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
pub struct InputStateField {
    is_active: bool,
    delay_timer: f64,
}

impl Default for InputStateField {
    fn default() -> Self {
        InputStateField {
            is_active: false,
            delay_timer: INPUT_DELAY_TIME,
        }
    }
}

#[derive(Debug)]
pub struct InputState {
    left: InputStateField,
    right: InputStateField,
    soft_drop: InputStateField,
    hard_drop: InputStateField,
    rotate_clockwise: InputStateField,
    rotate_counterclockwise: InputStateField,
    drop: InputStateField,
}

impl Default for InputState {
    fn default() -> Self {

        InputState {
            left: InputStateField::default(),
            right: InputStateField::default(),
            soft_drop: InputStateField::default(),
            hard_drop: InputStateField::default(),
            rotate_clockwise: InputStateField::default(),
            rotate_counterclockwise: InputStateField::default(),
            drop: InputStateField::default(),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Command {
    Left,
    None,
}

pub struct PlayState {
    current_command: Command,
    input: InputState,

    well: Well,
    bag: PieceBag,
    current_piece: Piece,

    fall_timer: f64,
    score: u32,
    cleared_lines: u16,
    game_over: bool,

    // temp
    image: graphics::Image,
}


impl PlayState {
    pub fn new(ctx: &mut Context) -> GameResult<PlayState> {
        let image = graphics::Image::new(ctx, "/block.png")?;
        let mut bag = PieceBag::new();
        let first_piece = bag.take_piece();

        let state = PlayState {
            current_command: Command::None,
            input: InputState::default(),

            well: Well::new(),
            bag: bag,
            current_piece: first_piece,

            fall_timer: 0.0,
            score: 0,
            cleared_lines: 0,
            game_over: false,

            image: image,
        };

        Ok(state)
    }

    fn handle_user_input(&mut self, dt: Duration) -> GameResult<()> {
        if self.input.left.is_active {
            self.input.left.delay_timer += dt.as_subsec_millis();
            if self.input.left.delay_timer >= INPUT_DELAY_TIME {
                self.current_piece.potential_top_left.x -= 1;
                self.input.left.delay_timer = 0.0;
            }
        } else if self.input.right.is_active {
            self.input.right.delay_timer += dt.as_subsec_millis();
            if self.input.right.delay_timer >= INPUT_DELAY_TIME {
                self.current_piece.potential_top_left.x += 1;
                self.input.right.delay_timer = 0.0;
            }
        } else if self.input.soft_drop.is_active {
            self.input.soft_drop.delay_timer += dt.as_subsec_millis();
            if self.input.soft_drop.delay_timer >= INPUT_DELAY_TIME {
                self.current_piece.potential_top_left.y += 1;
                self.input.soft_drop.delay_timer = 0.0;
            }
        }

        Ok(())
    }

    /// Advance the fall time. If enough time has passed, allow gravity to
    /// affect the current piece.
    /// Returns a `GameResult<false>` if a landing occured.
    /// Returns a `GameResult<true>` if no landing occured and the piece can advance.
    fn handle_gravity(&mut self, dt: Duration) -> GameResult<bool> {
        self.fall_timer += dt.as_subsec_millis();

        if self.fall_timer >= FALL_SPEED {
            let current_shape = self.current_piece.get_shape();
            self.fall_timer = 0.0;
            self.current_piece.potential_top_left.y += 1;

            let did_land = self.well
                .check_for_landing(&current_shape, &self.current_piece.potential_top_left);

            if did_land {
                if self.current_piece.top_left.y < 2 {
                    println!("game over!");
                    self.game_over = true;
                    return Ok(false);
                }

                // game isn't over - take another piece and move to next frame
                self.well.land(&self.current_piece);
                self.current_piece = self.bag.take_piece();
                return Ok(false);
            }

            // piece did not land - advance!
            self.current_piece.top_left = self.current_piece.potential_top_left;
        }

        Ok(true)
    }

    fn handle_shadow_piece(&mut self) -> GameResult<()> {
        let mut shadow_position = self.current_piece.top_left;
        let mut potential_shadow_position = shadow_position;
        loop {
            potential_shadow_position.y += 1;
            let collision_found = self.well
                .check_for_landing(&self.current_piece.get_shape(), &potential_shadow_position);

            if collision_found {
                break;
            }

            shadow_position = potential_shadow_position;
            self.current_piece.set_shadow_position(shadow_position);
        }

        Ok(())
    }

    fn handle_lines_clears(&mut self) -> GameResult<()> {
        // check for line clears
        for r in (0..self.well.data.len()).rev() {
            let mut is_row_filled = true;
            for (c, _) in self.well.data[r].iter().enumerate() {
                if self.well.data[r][c] == 0 {
                    is_row_filled = false;
                    break; // no need to continue iterating, line is not clear...
                }
            }

            if is_row_filled {
                // TODO: implement more line clearing algorithms
                // TODO: make the current line clearing algorithm user selectable
                self.well.naive_line_clear(r);
                self.cleared_lines += 1;
            }
        }

        Ok(())
    }

    fn handle_collisions(&mut self) -> GameResult<()> {
        if self.input.left.is_active || self.input.right.is_active ||
           self.input.soft_drop.is_active {
            let current_shape = self.current_piece.get_shape();
            let collision_found = self.well
                .check_for_collisions(&current_shape, &self.current_piece.potential_top_left);

            if collision_found {
                self.current_piece.potential_top_left = self.current_piece.top_left;
            }

            self.current_piece.top_left = self.current_piece.potential_top_left; // advance tetromino
            Ok(())
        } else if self.input.rotate_clockwise.is_active ||
                  self.input.rotate_counterclockwise.is_active {
            let next_shape = self.current_piece.get_next_shape();
            let collision_found = self.well
                .check_for_collisions(&next_shape, &self.current_piece.top_left);

            if !collision_found {
                self.current_piece.change_shape();
            } else {
                // wall kick attempt!
                // need to do 2 checks:
                // move one piece to the right & perform all above checks
                let mut potential_position = self.current_piece.top_left; // creates a copy of 'Position' struct
                potential_position.x += 1;
                let collision_found = self.well
                    .check_for_collisions(&next_shape, &potential_position);

                if !collision_found {
                    self.current_piece.top_left = potential_position;
                    self.current_piece.potential_top_left = potential_position;
                    self.current_piece.change_shape();
                } else {
                    let mut potential_position = self.current_piece.top_left;
                    potential_position.x -= 1;
                    let collision_found = self.well
                        .check_for_collisions(&next_shape, &potential_position);

                    if !collision_found {
                        self.current_piece.top_left = potential_position;
                        self.current_piece.potential_top_left = potential_position;
                        self.current_piece.change_shape();
                    }
                }
            }

            Ok(())
        } else if self.input.hard_drop.is_active {
            self.current_piece.top_left = self.current_piece.get_shadow_position();
            self.well.land(&self.current_piece);
            self.current_piece = self.bag.take_piece();

            Ok(())
        } else {
            Ok(())
        }
    }
}

impl event::EventHandler for PlayState {
    fn update(&mut self, _: &mut Context, dt: Duration) -> GameResult<()> {
        println!("[state.update] Start of update method");
        if self.game_over {
            // do game over stuff
            return Ok(());
        }

        // TODO: handle/respond user input
        self.handle_user_input(dt)?;

        // handle shadow piece
        // TODO: put behind option
        self.handle_shadow_piece()?;

        self.handle_collisions()?;

        // handle gravity - return from update if our current piece landed
        if let Ok(false) = self.handle_gravity(dt) {
            return Ok(());
        }

        self.handle_lines_clears()?;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::set_background_color(ctx, graphics::Color::new(0.0, 0.0, 0.0, 255.0));
        graphics::clear(ctx);

        self.well.draw(ctx, &self.image)?;
        self.current_piece
            .draw_shadow(ctx, &self.image, &self.current_piece.get_shadow_position())?;
        self.current_piece.draw(ctx, &self.image)?;

        graphics::present(ctx);

        Ok(())
    }

    fn key_down_event(&mut self, keycode: Keycode, _keymod: Mod, repeat: bool) {
        println!("[state.key_down_event] Key down event occured");
        println!("Key: {}, Repeat?: {}", keycode, repeat);
        match keycode {
            Keycode::Left => {
                self.input.left.is_active = true;
                self.current_command = Command::Left;
            }
            Keycode::Right => self.input.right.is_active = true,
            Keycode::Up => self.input.hard_drop.is_active = true,
            Keycode::Down => self.input.soft_drop.is_active = true,
            Keycode::Z => self.input.rotate_counterclockwise.is_active = true,
            Keycode::X => self.input.rotate_clockwise.is_active = true,
            _ => (),
        }

    }

    fn key_up_event(&mut self, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Left => {
                self.input.left.is_active = false;
                self.input.left.delay_timer = INPUT_DELAY_TIME;
            }
            Keycode::Right => self.input.right.is_active = false,
            Keycode::Up => self.input.hard_drop.is_active = false,
            Keycode::Down => self.input.soft_drop.is_active = false,
            Keycode::Z => self.input.rotate_counterclockwise.is_active = false,
            Keycode::X => self.input.rotate_clockwise.is_active = false,
            _ => (),
        }
    }
}
