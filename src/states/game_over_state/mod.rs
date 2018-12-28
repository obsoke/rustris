use crate::states::menu_state::MenuState;
use crate::states::play_state::PlayState;
use crate::states::shared::option::{Option, OptionInputCommand};
use crate::states::{Assets, State, Transition};
use crate::util::play_click_sfx;
use ggez::event::{Button, Keycode, Mod};
use ggez::graphics::Point2;
use ggez::{graphics, Context, GameResult};
use std::time::Duration;

/// Describes whether to render `GameEndState` under either "Player Wins" or
/// "Player Loses" conditions.
pub enum GameEndMode {
    Win,
    Lose,
}

pub struct GameEndState {
    request_replay: bool,
    request_menu: bool,
    request_quit: bool,
    options: Vec<Option>,
    current_selection: usize,

    game_end_text: graphics::Text,
    final_score_text: graphics::Text,
    final_line_text: graphics::Text,
    final_level_text: graphics::Text,
}

impl GameEndState {
    /// A `GameEndState` takes values from `PlayState` to render certain values
    /// such as no. of lines cleared, highest level cleared, final score, etc.
    pub fn new(
        ctx: &mut Context,
        assets: &Assets,
        mode: &GameEndMode,
        final_score_value: u32,
        final_cleared: u32,
        final_level: u32,
    ) -> GameResult<Self> {
        let game_over: graphics::Text;
        game_over = match mode {
            GameEndMode::Lose => graphics::Text::new(ctx, "GAME OVER", assets.get_font("title")?)?,
            GameEndMode::Win => graphics::Text::new(ctx, "YOU WIN!", assets.get_font("title")?)?,
        };

        let score_str = format!("Final Score: {}", final_score_value);
        let lines_str = format!("Final Lines: {}", final_cleared);
        let level_str = format!("Final Level: {}", final_level);
        let final_score = graphics::Text::new(ctx, &score_str, assets.get_font("normal")?)?;
        let final_lines = graphics::Text::new(ctx, &lines_str, assets.get_font("normal")?)?;
        let final_level = graphics::Text::new(ctx, &level_str, assets.get_font("normal")?)?;

        let coords = graphics::get_screen_coordinates(ctx);
        let mut options_vec: Vec<Option> = Vec::new();
        options_vec.push(Option::new(
            ctx,
            assets,
            "Play again",
            Point2::new(coords.w / 2.0, 450.0),
        ));
        options_vec.push(Option::new(
            ctx,
            assets,
            "Return to Menu",
            Point2::new(coords.w / 2.0, 525.0),
        ));
        options_vec.push(Option::new(
            ctx,
            assets,
            "Quit",
            Point2::new(coords.w / 2.0, 600.0),
        ));

        match mode {
            GameEndMode::Win => assets.get_sfx("gameover_win")?.play()?,
            GameEndMode::Lose => assets.get_sfx("gameover_lose")?.play()?,
        }

        Ok(GameEndState {
            request_replay: false,
            request_menu: false,
            request_quit: false,
            options: options_vec,
            current_selection: 0,

            game_end_text: game_over,
            final_score_text: final_score,
            final_line_text: final_lines,
            final_level_text: final_level,
        })
    }

    // Ideally, I would not be coupling `Assets` to this method. Would a
    // messaging system be fast enough to handle audio system stuff? Maybe
    // something to try for v2.
    fn handle_input(&mut self, command: &OptionInputCommand, assets: &Assets) {
        match command {
            OptionInputCommand::Up => {
                play_click_sfx(assets).expect("Could not play click sfx in game end state -> up");
                if self.current_selection == 0 {
                    self.current_selection = self.options.len() - 1;
                } else {
                    self.current_selection -= 1;
                }
            }
            OptionInputCommand::Down => {
                play_click_sfx(assets).expect("Could not play click sfx in game end state -> down");
                if self.current_selection >= self.options.len() - 1 {
                    self.current_selection = 0;
                } else {
                    self.current_selection += 1;
                }
            }
            OptionInputCommand::Select => {
                if self.current_selection == 0 {
                    self.request_replay = true;
                } else if self.current_selection == 1 {
                    self.request_menu = true;
                } else if self.current_selection == 2 {
                    self.request_quit = true;
                }
            }
        }
    }
}

impl State for GameEndState {
    fn update(
        &mut self,
        ctx: &mut Context,
        assets: &Assets,
        _: Duration,
    ) -> GameResult<Transition> {
        if self.request_menu {
            return Ok(Transition::Swap(Box::new(MenuState::new(ctx, assets)?)));
        } else if self.request_replay {
            return Ok(Transition::Swap(Box::new(PlayState::new(ctx, assets)?)));
        } else if self.request_quit {
            return Ok(Transition::Drain);
        }

        for (i, option) in self.options.iter_mut().enumerate() {
            if i == self.current_selection {
                option.update(true)?;
            } else {
                option.update(false)?;
            }
        }

        Ok(Transition::None)
    }

    fn draw(&mut self, ctx: &mut Context, _: &Assets) -> GameResult<()> {
        let coords = graphics::get_screen_coordinates(ctx);

        let game_over_dest = Point2::new(coords.w / 2.0, 100.0);
        let game_over_score_dest = Point2::new(coords.w / 2.0, 200.0);
        let game_over_lines_dest = Point2::new(coords.w / 2.0, 250.0);
        let game_over_level_dest = Point2::new(coords.w / 2.0, 300.0);

        graphics::set_color(ctx, graphics::Color::new(0.0, 0.0, 0.0, 0.7))?;
        graphics::rectangle(
            ctx,
            graphics::DrawMode::Fill,
            graphics::Rect::new(
                0.0 + (coords.w / 2.0),
                0.0 + ((coords.h * -1.0) / 2.0),
                coords.w,
                coords.h * -1.0,
            ),
        )?;
        graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0))?;
        graphics::draw(ctx, &self.game_end_text, game_over_dest, 0.0)?;
        graphics::draw(ctx, &self.final_score_text, game_over_score_dest, 0.0)?;
        graphics::draw(ctx, &self.final_line_text, game_over_lines_dest, 0.0)?;
        graphics::draw(ctx, &self.final_level_text, game_over_level_dest, 0.0)?;

        for option in &self.options {
            option.draw(ctx)?;
        }

        Ok(())
    }

    fn key_down_event(&mut self, keycode: Keycode, _keymod: Mod, repeat: bool, assets: &Assets) {
        if repeat {
            return;
        }

        match keycode {
            Keycode::Up => self.handle_input(&OptionInputCommand::Up, assets),
            Keycode::Down => self.handle_input(&OptionInputCommand::Down, assets),
            Keycode::Return => self.handle_input(&OptionInputCommand::Select, assets),
            _ => (),
        }
    }

    fn controller_button_down_event(&mut self, btn: Button, _instance_id: i32, assets: &Assets) {
        match btn {
            Button::DPadUp => self.handle_input(&OptionInputCommand::Up, assets),
            Button::DPadDown => self.handle_input(&OptionInputCommand::Down, assets),
            Button::A => self.handle_input(&OptionInputCommand::Select, assets),
            _ => (),
        }
    }
}
