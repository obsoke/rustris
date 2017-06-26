pub mod tetromino;
mod well;
mod shapes;
mod bag;
mod ui_element;
mod input_state;

use std::time::Duration;
use ggez::{Context, GameResult};
use ggez::graphics::Point;
use self::well::Well;
use self::tetromino::{Piece, PieceType};
use self::bag::PieceBag;
use self::ui_element::{UIBlockView, UITextView};
use self::input_state::InputState;
use states::game_over_state::{GameEndState, GameEndMode};
use event::{Assets, Transition, EventHandler, Keycode, Mod, Button};
use util::{DurationExt, play_click_sfx};

const INITIAL_DELAY_TIME: f64 = 0.15;
const SECONDARY_DELAY_TIME: f64 = 0.05;

const BLOCK_SIZE: f32 = 30.0;
const BASE_FALL_SPEED: f64 = 1.0;
const FALL_SPEED_DIVISOR: f64 = 4.0;
const LINES_PER_LEVEL: i32 = 10;
const MAX_LEVEL: u32 = 15;
const NON_PLAY_SONGS: u32 = 1; // .... this sucks

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

    current_track_name: String,

    // ui elements
    ui_level: UITextView,
    ui_lines: UITextView,
    ui_score: UITextView,
    ui_next: UIBlockView,
    ui_hold: UIBlockView,
}

impl PlayState {
    pub fn new(ctx: &mut Context, assets: &Assets) -> GameResult<PlayState> {
        use rand;
        use rand::Rng;

        let mut bag = PieceBag::new();
        let first_piece = bag.take_piece();
        let first_type = first_piece.get_type();

        // this is a little hacky... each song is being added as play_0, play_1
        // and so on... if there happens 5 songs that aren't properly named,
        // eventually this will cause a panic. also, the last song is always the
        // menu song, we will not include that in our potential songs to play in
        // the hackiest way possible
        let song_count = assets.get_music_count();
        let song_no = rand::thread_rng().gen_range(0, song_count - NON_PLAY_SONGS);
        let song_name = format!("play_{}", song_no);

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

            current_track_name: song_name,

            ui_next: UIBlockView::new(
                ctx,
                assets,
                Point::new(775.0, 70.0),
                "Next",
                Some(first_type),
            ),
            ui_hold: UIBlockView::new(ctx, assets, Point::new(775.0, 250.0), "Hold", None),
            ui_level: UITextView::new(ctx, assets, Point::new(775.0, 420.0), "Level", "1"),
            ui_lines: UITextView::new(ctx, assets, Point::new(775.0, 500.0), "Lines", "0"),
            ui_score: UITextView::new(ctx, assets, Point::new(775.0, 580.0), "Score", "0"),
        })
    }

    fn handle_user_input(&mut self, dt: Duration, assets: &Assets) -> GameResult<()> {
        if self.input.left.is_active {
            // initial piece movement
            if self.input.left.initial_delay_timer == 0.0 {
                self.input.left.initial_delay_timer += dt.as_subsec_millis();
                self.move_piece(Point::new(-1.0, 0.0), assets);
            }
            // initial movement delay
            else if self.input.left.initial_delay_timer <= INITIAL_DELAY_TIME {
                self.input.left.initial_delay_timer += dt.as_subsec_millis();
            }
            // secondary piece movement
            else if self.input.left.secondary_delay_timer == 0.0 {
                self.input.left.secondary_delay_timer += dt.as_subsec_millis();
                self.move_piece(Point::new(-1.0, 0.0), assets);
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
                self.move_piece(Point::new(1.0, 0.0), assets);
            }
            // initial movement delay
            else if self.input.right.initial_delay_timer <= INITIAL_DELAY_TIME {
                self.input.right.initial_delay_timer += dt.as_subsec_millis();
            }
            // secondary piece movement
            else if self.input.right.secondary_delay_timer == 0.0 {
                self.input.right.secondary_delay_timer += dt.as_subsec_millis();
                self.move_piece(Point::new(1.0, 0.0), assets);
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
                self.move_piece(Point::new(0.0, 1.0), assets);
            }
            // initial movement delay
            else if self.input.soft_drop.initial_delay_timer <= INITIAL_DELAY_TIME {
                self.input.soft_drop.initial_delay_timer += dt.as_subsec_millis();
            }
            // secondary piece movement
            else if self.input.soft_drop.secondary_delay_timer == 0.0 {
                self.input.soft_drop.secondary_delay_timer += dt.as_subsec_millis();
                self.move_piece(Point::new(0.0, 1.0), assets);
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
                self.rotate_piece(assets, 1);
            }
        } else if self.input.rotate_counterclockwise.is_active {
            if self.input.rotate_counterclockwise.is_active !=
                self.prev_input.rotate_counterclockwise.is_active
            {
                self.rotate_piece(assets, -1);
            }
        } else if self.input.hard_drop.is_active {
            if self.input.hard_drop.is_active != self.prev_input.hard_drop.is_active {
                self.current_piece.top_left = self.current_piece.get_shadow_position();
                self.well.land(&self.current_piece);
                self.current_piece = self.bag.take_piece();
                self.can_hold = true;
                play_click_sfx(assets).expect("Could not play click after hard drop");
            }
        } else if self.input.hold.is_active &&
                   self.input.hold.is_active != self.prev_input.hold.is_active
        {
            self.handle_hold()?;
        }

        Ok(())
    }

    fn move_piece(&mut self, potential_new_position: Point, assets: &Assets) {
        self.current_piece.potential_top_left.x += potential_new_position.x;
        self.current_piece.potential_top_left.y += potential_new_position.y;

        let current_shape = self.current_piece.get_shape();
        let collision_found = self.well.check_for_collisions(
            &current_shape,
            &self.current_piece.potential_top_left,
        );

        if collision_found {
            self.current_piece.potential_top_left = self.current_piece.top_left;
        } else {
            // if we can move, play our movement audio!
            play_click_sfx(assets).expect("Could not play audio after movement.");
        }

        self.current_piece.top_left = self.current_piece.potential_top_left; // advance tetromino
    }

    fn rotate_piece(&mut self, assets: &Assets, direction: i32) {
        let next_shape = self.current_piece.get_next_shape(direction);
        let collision_found = self.well.check_for_collisions(
            &next_shape,
            &self.current_piece.top_left,
        );

        if !collision_found {
            self.current_piece.change_shape(direction);
            play_click_sfx(assets).expect("Could not play click after rotating");
        } else {
            // wall kick attempt!
            // need to do 2 checks:
            // move one piece to the right & perform all above checks
            let mut potential_position = self.current_piece.top_left; // creates a copy of 'Position' struct
            potential_position.x += 1.0;
            let collision_found = self.well.check_for_collisions(
                &next_shape,
                &potential_position,
            );

            if !collision_found {
                self.current_piece.top_left = potential_position;
                self.current_piece.potential_top_left = potential_position;
                self.current_piece.change_shape(direction);
                play_click_sfx(assets).expect("Could not play click after rotating");
            } else {
                let mut potential_position = self.current_piece.top_left;
                potential_position.x -= 1.0;
                let collision_found = self.well.check_for_collisions(
                    &next_shape,
                    &potential_position,
                );

                if !collision_found {
                    self.current_piece.top_left = potential_position;
                    self.current_piece.potential_top_left = potential_position;
                    self.current_piece.change_shape(direction);
                    play_click_sfx(assets).expect("Could not play click after rotating");
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
            self.current_piece.potential_top_left.y += 1.0;

            let did_land = self.well.check_for_landing(
                &current_shape,
                &self.current_piece.potential_top_left,
            );

            if did_land {
                if self.current_piece.top_left.y < 2.0 {
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
            potential_shadow_position.y += 1.0;
            let collision_found = self.well.check_for_landing(
                &self.current_piece.get_shape(),
                &potential_shadow_position,
            );

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
        self.level += 1;

        // increase gravity
        let change = self.time_until_gravity / FALL_SPEED_DIVISOR;
        self.time_until_gravity -= change;
    }
}


impl EventHandler for PlayState {
    fn update(
        &mut self,
        ctx: &mut Context,
        assets: &Assets,
        dt: Duration,
    ) -> GameResult<Transition> {
        // currently necessary to keep audio looping
        let current_song = assets.get_music(&self.current_track_name)?;
        if current_song.paused() {
            current_song.resume();
        } else {
            current_song.play()?;
        }

        if self.game_over {
            assets.get_music(&self.current_track_name)?.pause();
            return Ok(Transition::Push(Box::new(GameEndState::new(
                ctx,
                assets,
                GameEndMode::Lose,
                self.score,
                self.cleared_lines,
                self.level + 1,
            )?)));
        } else if self.level >= MAX_LEVEL {
            assets.get_music(&self.current_track_name)?.pause();
            return Ok(Transition::Push(Box::new(GameEndState::new(
                ctx,
                assets,
                GameEndMode::Win,
                self.score,
                self.cleared_lines,
                self.level + 1,
            )?)));

        }

        // we pass Assets along so we can play sounds - not the greatest pattern
        // (ideally, maybe a messaging system?)
        self.handle_user_input(dt, assets)?;
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
        self.ui_next.update(
            ctx,
            assets,
            Some(self.bag.peek_at_next_piece().get_type()),
        );
        self.ui_level.update(
            ctx,
            assets,
            &(&self.level + 1).to_string(),
        );
        self.ui_lines.update(
            ctx,
            assets,
            &self.cleared_lines.to_string(),
        );
        self.ui_score.update(ctx, assets, &self.score.to_string());


        Ok(Transition::None)
    }

    fn draw(&mut self, ctx: &mut Context, assets: &Assets) -> GameResult<()> {
        self.well.draw(ctx, assets.get_image("block")?)?;
        self.current_piece.draw_shadow(
            ctx,
            assets.get_image("block")?,
            &self.current_piece.get_shadow_position(),
        )?;
        self.current_piece.draw(ctx, assets.get_image("block")?)?;

        self.ui_next.draw(ctx, assets)?;
        self.ui_hold.draw(ctx, assets)?;
        self.ui_level.draw(ctx)?;
        self.ui_lines.draw(ctx)?;
        self.ui_score.draw(ctx)?;

        Ok(())
    }

    fn key_down_event(&mut self, keycode: Keycode, _keymod: Mod, _repeat: bool, _assets: &Assets) {
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

    fn controller_button_down_event(&mut self, btn: Button, _instance_id: i32, _assets: &Assets) {
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
