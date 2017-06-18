use std::time::Duration;
use std::ops::AddAssign;
use ggez::{Context, GameResult};

mod well;
mod tetromino;
mod shapes;
mod bag;
mod util;
mod ui_element;

use self::well::Well;
use self::tetromino::Piece;
use self::bag::PieceBag;
use self::util::DurationExt;
use self::ui_element::UIElement;
use states::{Assets, Transition};
use states::game_over_state::GameOverState;
use event::*;

const BLOCK_SIZE: f32 = 30.0;
const FALL_SPEED: f64 = 0.5;
const INPUT_DELAY_TIME: f64 = 0.06;

#[derive(Copy, Clone, Debug)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self { x: x, y: y }
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, other: Position) {
        *self = Position {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
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

pub struct PlayState {
    input: InputState,
    prev_input: InputState,

    well: Well,
    bag: PieceBag,
    current_piece: Piece,

    fall_timer: f64,
    score: u32,
    cleared_lines: u32,
    level: u32,
    game_over: bool,

    // ui elements
    ui_lines: UIElement,
    ui_score: UIElement,
}

impl PlayState {
    pub fn new(ctx: &mut Context, assets: &Assets) -> GameResult<PlayState> {
        let mut bag = PieceBag::new();
        let first_piece = bag.take_piece();

        Ok(PlayState {
            input: InputState::default(),
            prev_input: InputState::default(),

            well: Well::new(),
            bag: bag,
            current_piece: first_piece,

            fall_timer: 0.0,
            score: 0,
            cleared_lines: 0,
            level: 0,
            game_over: false,

            ui_lines: UIElement::new(ctx, assets, Position::new(775, 300), "Lines", "0"),
            ui_score: UIElement::new(ctx, assets, Position::new(775, 450), "Score", "0"),
        })
    }

    fn handle_user_input(&mut self, dt: Duration) -> GameResult<()> {
        if self.input.left.is_active {
            self.input.left.delay_timer += dt.as_subsec_millis();
            if self.input.left.delay_timer >= INPUT_DELAY_TIME {
                self.move_piece(Position::new(-1, 0));
                self.input.left.delay_timer = 0.0;
            }
        } else if self.input.right.is_active {
            self.input.right.delay_timer += dt.as_subsec_millis();
            if self.input.right.delay_timer >= INPUT_DELAY_TIME {
                self.move_piece(Position::new(1, 0));
                self.input.right.delay_timer = 0.0;
            }
        } else if self.input.soft_drop.is_active {
            self.input.soft_drop.delay_timer += dt.as_subsec_millis();
            if self.input.soft_drop.delay_timer >= INPUT_DELAY_TIME {
                self.move_piece(Position::new(0, 1));
                self.input.soft_drop.delay_timer = 0.0;
            }
        } else if self.input.rotate_clockwise.is_active {
            if self.input.rotate_clockwise.is_active != self.prev_input.rotate_clockwise.is_active {
                self.rotate_piece(1);
            }
        } else if self.input.rotate_counterclockwise.is_active {
            if self.input.rotate_counterclockwise.is_active !=
               self.prev_input.rotate_counterclockwise.is_active {
                self.rotate_piece(-1);
            }
        } else if self.input.hard_drop.is_active {
            if self.input.hard_drop.is_active != self.prev_input.hard_drop.is_active {
                self.current_piece.top_left = self.current_piece.get_shadow_position();
                self.well.land(&self.current_piece);
                self.current_piece = self.bag.take_piece();
            }
        }

        Ok(())
    }

    fn move_piece(&mut self, potential_new_position: Position) {
        self.current_piece.potential_top_left += potential_new_position;
        let current_shape = self.current_piece.get_shape();
        let collision_found = self.well
            .check_for_collisions(&current_shape, &self.current_piece.potential_top_left);

        if collision_found {
            self.current_piece.potential_top_left = self.current_piece.top_left;
        }

        self.current_piece.top_left = self.current_piece.potential_top_left; // advance tetromino
    }

    fn rotate_piece(&mut self, direction: i32) {
        let next_shape = self.current_piece.get_next_shape(direction);
        let collision_found = self.well
            .check_for_collisions(&next_shape, &self.current_piece.top_left);

        if !collision_found {
            self.current_piece.change_shape(direction);
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
                self.current_piece.change_shape(direction);
            } else {
                let mut potential_position = self.current_piece.top_left;
                potential_position.x -= 1;
                let collision_found = self.well
                    .check_for_collisions(&next_shape, &potential_position);

                if !collision_found {
                    self.current_piece.top_left = potential_position;
                    self.current_piece.potential_top_left = potential_position;
                    self.current_piece.change_shape(direction);
                }
            }
        }

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

    fn handle_line_clears(&mut self) -> GameResult<()> {
        // check for line clears
        let mut lines_cleared = 0;
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
                lines_cleared += 1;
            }
        }

        // add to score
        let level = self.level;
        match lines_cleared {
            1 => self.increase_score(40, level),
            2 => self.increase_score(100, level),
            3 => self.increase_score(300, level),
            4 => self.increase_score(1200, level),
            _ => (),
        }

        self.cleared_lines += lines_cleared;

        Ok(())
    }

    fn increase_score(&mut self, base_score: u32, level: u32) {
        self.score += base_score * (level + 1);
    }
}


impl EventHandler for PlayState {
    fn update(&mut self,
              ctx: &mut Context,
              assets: &Assets,
              dt: Duration)
              -> GameResult<Transition> {
        if self.game_over {
            return Ok(Transition::Push(Box::new(GameOverState::new(ctx,
                                                                   assets,
                                                                   self.score,
                                                                   self.cleared_lines)?)));
        }

        self.handle_user_input(dt)?;

        // handle shadow piece
        // TODO: put behind option
        self.handle_shadow_piece()?;

        // handle gravity - return from update if our current piece landed
        if let Ok(false) = self.handle_gravity(dt) {
            return Ok(Transition::None);
        }

        self.handle_line_clears()?;
        self.ui_lines.update(ctx, assets, &self.cleared_lines.to_string());
        self.ui_score.update(ctx, assets, &self.score.to_string());

        self.prev_input = self.input;

        // update ui

        Ok(Transition::None)
    }

    fn draw(&mut self, ctx: &mut Context, assets: &Assets) -> GameResult<()> {
        self.well.draw(ctx, assets.get_image("block")?)?;
        self.current_piece
            .draw_shadow(ctx,
                         assets.get_image("block")?,
                         &self.current_piece.get_shadow_position())?;
        self.current_piece.draw(ctx, assets.get_image("block")?)?;

        self.ui_lines.draw(ctx)?;
        self.ui_score.draw(ctx)?;

        Ok(())
    }

    fn key_down_event(&mut self, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Left => self.input.left.is_active = true,
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
            Keycode::Right => {
                self.input.right.is_active = false;
                self.input.right.delay_timer = INPUT_DELAY_TIME;
            }
            Keycode::Up => {
                self.input.hard_drop.is_active = false;
                self.input.hard_drop.delay_timer = INPUT_DELAY_TIME;
            }
            Keycode::Down => {
                self.input.soft_drop.is_active = false;
                self.input.soft_drop.delay_timer = INPUT_DELAY_TIME;
            }
            Keycode::Z => {
                self.input.rotate_counterclockwise.is_active = false;
                self.input.rotate_counterclockwise.delay_timer = INPUT_DELAY_TIME;
            }
            Keycode::X => {
                self.input.rotate_clockwise.is_active = false;
                self.input.rotate_clockwise.delay_timer = INPUT_DELAY_TIME;
            }
            _ => (),
        }
    }
}
