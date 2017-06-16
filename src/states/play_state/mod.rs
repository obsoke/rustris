use std::time::Duration;
use std::ops::AddAssign;
use ggez::{Context, GameResult, graphics, event};


mod well;
mod tetromino;
mod shapes;
mod bag;
mod game_over;
mod util;

use ggez::event::*;
use states::{Transition};
use self::well::Well;
use self::tetromino::Piece;
use self::bag::PieceBag;
use self::util::DurationExt;
//use self::game_over::GameOverState;

const BLOCK_SIZE: f32 = 30.0;
const FALL_SPEED: f64 = 0.5;
const INPUT_DELAY_TIME: f64 = 0.09;

#[derive(Copy, Clone, Debug)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self {
            x: x,
            y: y,
        }
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
    cleared_lines: u16,
    level: u32,
    game_over: bool,

    // temp
    image: graphics::Image,
    font_big: graphics::Font,
    font_small: graphics::Font,
    game_over_text: graphics::Text,
    game_over_end_text: graphics::Text,
    game_over_final_score: graphics::Text,
    game_over_final_lines: graphics::Text,
}

impl PlayState {
    pub fn new(ctx: &mut Context) -> GameResult<PlayState> {
        let image = graphics::Image::new(ctx, "/block.png")?;
        let font_big = graphics::Font::new(ctx, "/DejaVuSansMono.ttf", 32)?;
        let font_small = graphics::Font::new(ctx, "/DejaVuSansMono.ttf", 18)?;
        let text = graphics::Text::new(ctx, "GAME OVER", &font_big)?;

        let end_text_src = "'R' to restart / 'M' for menu / 'Esc' to quit";
        // let end_text_src = r#"Press 'K' to Restart
        //                     Press 'M' to Return to Menu
        //                     Press 'Escape' to Exit"#;
        let end_text = graphics::Text::new(ctx, end_text_src, &font_small)?;
        let final_score = graphics::Text::new(ctx, "Final Score", &font_small)?;
        let final_lines = graphics::Text::new(ctx, "Final Lines", &font_small)?;

        let mut bag = PieceBag::new();
        let first_piece = bag.take_piece();

        let state = PlayState {
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

            image: image,
            font_big: font_big,
            font_small: font_small,
            game_over_text: text,
            game_over_end_text: end_text,
            game_over_final_score: final_score,
            game_over_final_lines: final_lines,
        };

        Ok(state)
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
            if self.input.rotate_counterclockwise.is_active != self.prev_input.rotate_counterclockwise.is_active {
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

impl event::EventHandler for PlayState {
    fn update(&mut self, ctx: &mut Context, dt: Duration) -> GameResult<Transition> {
        if self.game_over {
            // do game over stuff like check input for next action (restart,
            // back to main menu)
            let score_str = format!("Final Score: {}", self.score);
            let lines_str = format!("Final Lines: {}", self.cleared_lines);
            let final_score = graphics::Text::new(ctx, &score_str, &self.font_small)?;
            let final_lines = graphics::Text::new(ctx, &lines_str, &self.font_small)?;
            self.game_over_final_score = final_score;
            self.game_over_final_lines = final_lines;
            //return Ok(Transition::Push(Box::new(TestState)));
            return Ok(Transition::Pop);
        }

        // TODO: handle/respond user input
        self.handle_user_input(dt)?;

        // handle shadow piece
        // TODO: put behind option
        self.handle_shadow_piece()?;

        // handle gravity - return from update if our current piece landed
        if let Ok(false) = self.handle_gravity(dt) {
            return Ok(Transition::None);
        }

        self.handle_line_clears()?;

        self.prev_input = self.input;

        Ok(Transition::None)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::set_background_color(ctx, graphics::Color::new(0.0, 0.0, 0.0, 255.0));
        graphics::clear(ctx);

        self.well.draw(ctx, &self.image)?;
        self.current_piece
            .draw_shadow(ctx, &self.image, &self.current_piece.get_shadow_position())?;
        self.current_piece.draw(ctx, &self.image)?;

        if self.game_over {
            let coords = graphics::get_screen_coordinates(&ctx);

            let game_over_dest = graphics::Point::new(coords.w / 2.0, 100.0);
            let game_over_score_dest = graphics::Point::new(coords.w / 2.0, 200.0);
            let game_over_lines_dest = graphics::Point::new(coords.w / 2.0, 250.0);
            let game_over_end_dest = graphics::Point::new(coords.w / 2.0, 400.0);

            graphics::set_color(ctx, graphics::Color::new(0.0, 0.0, 0.0, 0.7))?;
            graphics::rectangle(ctx, graphics::DrawMode::Fill, graphics::Rect::new(0.0 + (coords.w / 2.0),
                                                                                   0.0 + ((coords.h * -1.0) / 2.0),
                                                                                   coords.w,
                                                                                   coords.h * -1.0))?;
            graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0))?;
            graphics::draw(ctx, &self.game_over_text, game_over_dest, 0.0)?;
            graphics::draw(ctx, &self.game_over_final_score, game_over_score_dest, 0.0)?;
            graphics::draw(ctx, &self.game_over_final_lines, game_over_lines_dest, 0.0)?;
            graphics::draw(ctx, &self.game_over_end_text, game_over_end_dest, 0.0)?;
        }

        graphics::present(ctx);

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
            },
            Keycode::Up => {
                self.input.hard_drop.is_active = false;
                self.input.hard_drop.delay_timer = INPUT_DELAY_TIME;
            },
            Keycode::Down => {
                self.input.soft_drop.is_active = false;
                self.input.soft_drop.delay_timer = INPUT_DELAY_TIME;
            },
            Keycode::Z => {
                self.input.rotate_counterclockwise.is_active = false;
                self.input.rotate_counterclockwise.delay_timer = INPUT_DELAY_TIME;
            },
            Keycode::X => {
                self.input.rotate_clockwise.is_active = false;
                self.input.rotate_clockwise.delay_timer = INPUT_DELAY_TIME;
            },
            _ => (),
        }
    }
}

struct TestState;

impl EventHandler for TestState {
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::set_background_color(ctx, graphics::Color::new(0.7, 0.7, 0.7, 255.0));
        graphics::clear(ctx);


        graphics::present(ctx);
        Ok(())
    }

    fn update(&mut self, ctx: &mut Context, _: Duration) -> GameResult<Transition> {
        println!("Welcome to test state!");
        Ok(Transition::None)
    }
}
