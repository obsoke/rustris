pub mod tetromino;
mod well;
mod shapes;
mod bag;
mod ui_element;

use std::time::Duration;
use std::ops::AddAssign;
use ggez::{Context, GameResult};
use self::well::Well;
use self::tetromino::{Piece, PieceType};
use self::bag::PieceBag;
use self::ui_element::{UIBlockView, UITextView};
use states::game_over_state::GameOverState;
use event::{Assets, Transition, EventHandler, Keycode, Mod, Button};
use util::DurationExt;

#[derive(Copy, Clone, Debug)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
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

const INITIAL_DELAY_TIME: f64 = 0.25;
const SECONDARY_DELAY_TIME: f64 = 0.05;

#[derive(Debug, Copy, Clone)]
struct InputStateField {
    is_active: bool,
    initial_delay_timer: f64,
    secondary_delay_timer: f64,
}

impl InputStateField {
    fn reset(&mut self) {
        self.is_active = false;
        self.initial_delay_timer = 0.0;
        self.secondary_delay_timer = 0.0;
    }
}

impl Default for InputStateField {
    fn default() -> Self {
        InputStateField {
            is_active: false,
            initial_delay_timer: 0.0,
            secondary_delay_timer: 0.0,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct InputState {
    left: InputStateField,
    right: InputStateField,
    soft_drop: InputStateField,
    hard_drop: InputStateField,
    rotate_clockwise: InputStateField,
    rotate_counterclockwise: InputStateField,
    drop: InputStateField,
    hold: InputStateField,
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
            hold: InputStateField::default(),
        }
    }
}

const BLOCK_SIZE: f32 = 30.0;
const BASE_FALL_SPEED: f64 = 1.0;
const FALL_SPEED_DIVISOR: f64 = 4.0;
const LINES_PER_LEVEL: i32 = 10;
const MAX_LEVEL: u32 = 20;

pub struct PlayState {
    input: InputState,
    prev_input: InputState,

    well: Well,
    bag: PieceBag,
    current_piece: Piece,
    hold_piece_type: Option<PieceType>,
    can_hold: bool,

    fall_timer: f64,
    time_until_gravity: f64,
    score: u32,
    cleared_lines: u32,
    lines_until_next_level: i32,
    level: u32,
    game_over: bool,

    // ui elements
    ui_level: UITextView,
    ui_lines: UITextView,
    ui_score: UITextView,
    ui_next: UIBlockView,
    ui_hold: UIBlockView,
}

impl PlayState {
    pub fn new(ctx: &mut Context, assets: &Assets) -> GameResult<PlayState> {
        let mut bag = PieceBag::new();
        let first_piece = bag.take_piece();
        let first_type = first_piece.get_type();

        Ok(PlayState {
            input: InputState::default(),
            prev_input: InputState::default(),

            well: Well::new(),
            bag: bag,
            current_piece: first_piece,
            hold_piece_type: None,

            fall_timer: 0.0,
            time_until_gravity: BASE_FALL_SPEED,
            score: 0,
            cleared_lines: 0,
            lines_until_next_level: LINES_PER_LEVEL,
            level: 0,
            can_hold: true,
            game_over: false,

            ui_next: UIBlockView::new(ctx, assets, Position::new(775, 70), "Next", Some(first_type)),
            ui_hold: UIBlockView::new(ctx, assets, Position::new(775, 250), "Hold", None),
            ui_level: UITextView::new(ctx, assets, Position::new(775, 420), "Level", "1"),
            ui_lines: UITextView::new(ctx, assets, Position::new(775, 500), "Lines", "0"),
            ui_score: UITextView::new(ctx, assets, Position::new(775, 580), "Score", "0"),
        })
    }

    fn handle_user_input(&mut self, dt: Duration) -> GameResult<()> {
        if self.input.left.is_active {
            // initial piece movement
            if self.input.left.initial_delay_timer == 0.0 {
                self.input.left.initial_delay_timer += dt.as_subsec_millis();
                self.move_piece(Position::new(-1, 0));
            }
            // initial movement delay
            else if self.input.left.initial_delay_timer <= INITIAL_DELAY_TIME {
                self.input.left.initial_delay_timer += dt.as_subsec_millis();
            }
            // secondary piece movement
            else if self.input.left.secondary_delay_timer == 0.0 {
                self.input.left.secondary_delay_timer += dt.as_subsec_millis();
                self.move_piece(Position::new(-1, 0));
            }
            // secondary movement delay
            else {
                self.input.left.secondary_delay_timer += dt.as_subsec_millis();
                if self.input.left.secondary_delay_timer >= SECONDARY_DELAY_TIME {
                    self.input.left.secondary_delay_timer = 0.0;
                }
            }
        } else if self.input.right.is_active {
            // initial piece movement
            if self.input.right.initial_delay_timer == 0.0 {
                self.input.right.initial_delay_timer += dt.as_subsec_millis();
                self.move_piece(Position::new(1, 0));
            }
            // initial movement delay
            else if self.input.right.initial_delay_timer <= INITIAL_DELAY_TIME {
                self.input.right.initial_delay_timer += dt.as_subsec_millis();
            }
            // secondary piece movement
            else if self.input.right.secondary_delay_timer == 0.0 {
                self.input.right.secondary_delay_timer += dt.as_subsec_millis();
                self.move_piece(Position::new(1, 0));
            }
            // secondary movement delay
            else {
                self.input.right.secondary_delay_timer += dt.as_subsec_millis();
                if self.input.right.secondary_delay_timer >= SECONDARY_DELAY_TIME {
                    self.input.right.secondary_delay_timer = 0.0;
                }
            }
        } else if self.input.soft_drop.is_active {
            // initial piece movement
            if self.input.soft_drop.initial_delay_timer == 0.0 {
                self.input.soft_drop.initial_delay_timer += dt.as_subsec_millis();
                self.move_piece(Position::new(0, 1));
            }
            // initial movement delay
            else if self.input.soft_drop.initial_delay_timer <= INITIAL_DELAY_TIME {
                self.input.soft_drop.initial_delay_timer += dt.as_subsec_millis();
            }
            // secondary piece movement
            else if self.input.soft_drop.secondary_delay_timer == 0.0 {
                self.input.soft_drop.secondary_delay_timer += dt.as_subsec_millis();
                self.move_piece(Position::new(0, 1));
            }
            // secondary movement delay
            else {
                self.input.soft_drop.secondary_delay_timer += dt.as_subsec_millis();
                if self.input.soft_drop.secondary_delay_timer >= SECONDARY_DELAY_TIME {
                    self.input.soft_drop.secondary_delay_timer = 0.0;
                }
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
                self.can_hold = true;
            }
        } else if self.input.hold.is_active {
            if self.input.hold.is_active != self.prev_input.hold.is_active {
                self.handle_hold()?;
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

        if self.fall_timer >= self.time_until_gravity {
            let current_shape = self.current_piece.get_shape();
            self.fall_timer = 0.0;
            self.current_piece.potential_top_left.y += 1;

            let did_land = self.well
                .check_for_landing(&current_shape, &self.current_piece.potential_top_left);

            if did_land {
                if self.current_piece.top_left.y < 2 {
                    self.game_over = true;
                    return Ok(false);
                }

                // game isn't over - take another piece and move to next frame
                self.well.land(&self.current_piece);
                self.current_piece = self.bag.take_piece();
                self.can_hold = true;
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

    fn handle_hold(&mut self) -> GameResult<()> {
        // can only perform a hold once per piece turn
        // a piece turn ends when the current piece lands
        if self.can_hold {
            let current_type = self.current_piece.get_type();
            if let Some(piece_type) = self.hold_piece_type {
                if piece_type != self.current_piece.get_type() {
                    self.current_piece = Piece::new(piece_type);
                    self.hold_piece_type = Some(current_type);
                    self.can_hold = false;
                }
            } else {
                self.current_piece = self.bag.take_piece();
                self.hold_piece_type = Some(current_type);
                self.can_hold = false;
            }

        }
        Ok(())
    }

    fn handle_line_clears(&mut self) -> GameResult<()> {
        // check for line clears
        let mut lines_cleared: u32 = 0;
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

        self.lines_until_next_level -= lines_cleared as i32;
        if self.lines_until_next_level <= 0 {
            self.increase_level();
            self.lines_until_next_level = LINES_PER_LEVEL;
        }
        self.cleared_lines += lines_cleared;

        Ok(())
    }

    fn increase_score(&mut self, base_score: u32, level: u32) {
        self.score += base_score * (level + 1);
    }

    fn increase_level(&mut self) {
        if self.level < MAX_LEVEL {
            self.level += 1;

            // increase gravity
            let change = self.time_until_gravity / FALL_SPEED_DIVISOR;
            self.time_until_gravity -= change;
            println!("New gravity value: {}", self.time_until_gravity);
        }
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
                                                                   self.cleared_lines,
                                                                   self.level + 1)?)));
        }

        self.handle_user_input(dt)?;
        self.prev_input = self.input;

        // handle shadow piece
        // TODO: put behind option
        self.handle_shadow_piece()?;

        // handle gravity - return from update if our current piece landed
        if let Ok(false) = self.handle_gravity(dt) {
            return Ok(Transition::None);
        }

        self.handle_line_clears()?;

        // update ui
        self.ui_hold.update(ctx, assets, self.hold_piece_type);
        self.ui_next.update(ctx, assets, Some(self.bag.peek_at_next_piece().get_type()));
        self.ui_level.update(ctx, assets, &(&self.level + 1).to_string());
        self.ui_lines.update(ctx, assets, &self.cleared_lines.to_string());
        self.ui_score.update(ctx, assets, &self.score.to_string());


        Ok(Transition::None)
    }

    fn draw(&mut self, ctx: &mut Context, assets: &Assets) -> GameResult<()> {
        self.well.draw(ctx, assets.get_image("block")?)?;
        self.current_piece
            .draw_shadow(ctx,
                         assets.get_image("block")?,
                         &self.current_piece.get_shadow_position())?;
        self.current_piece.draw(ctx, assets.get_image("block")?)?;

        self.ui_next.draw(ctx, assets)?;
        self.ui_hold.draw(ctx, assets)?;
        self.ui_level.draw(ctx)?;
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
            Keycode::Space => self.input.hold.is_active = true,
            _ => (),
        }

    }

    fn key_up_event(&mut self, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Left => self.input.left.reset(),
            Keycode::Right => self.input.right.reset(),
            Keycode::Up => self.input.hard_drop.reset(),
            Keycode::Down => self.input.soft_drop.reset(),
            Keycode::Z => self.input.rotate_counterclockwise.reset(),
            Keycode::X => self.input.rotate_clockwise.reset(),
            Keycode::Space => self.input.hold.reset(),
            _ => (),
        }
    }

    fn controller_button_down_event(&mut self, btn: Button, _instance_id: i32) {
        match btn {
            Button::DPadLeft => self.input.left.is_active = true,
            Button::DPadRight => self.input.right.is_active = true,
            Button::DPadUp => self.input.hard_drop.is_active = true,
            Button::DPadDown => self.input.soft_drop.is_active = true,
            Button::A => self.input.rotate_counterclockwise.is_active = true,
            Button::X => self.input.rotate_clockwise.is_active = true,
            Button::LeftShoulder => self.input.hold.is_active = true,
            _ => (),
        }
    }

    fn controller_button_up_event(&mut self, btn: Button, _instance_id: i32) {
        match btn {
            Button::DPadLeft => self.input.left.reset(),
            Button::DPadRight => self.input.right.reset(),
            Button::DPadUp => self.input.hard_drop.reset(),
            Button::DPadDown => self.input.soft_drop.reset(),
            Button::A => self.input.rotate_counterclockwise.reset(),
            Button::X => self.input.rotate_clockwise.reset(),
            Button::LeftShoulder => self.input.hold.reset(),
            _ => (),
        }
    }
}
