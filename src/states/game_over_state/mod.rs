use std::time::Duration;
use ggez::{Context, GameResult, graphics, event};
use ggez::event::{Mod, Keycode};
use event::{Assets, Transition, EventHandler};

use states::play_state::PlayState;
use states::menu_state::MenuState;

pub struct GameOverState {
    request_menu: bool,
    request_replay: bool,

    game_over_text: graphics::Text,
    final_score_text: graphics::Text,
    final_line_text: graphics::Text,
    instruction_text: graphics::Text,
}

impl GameOverState {
    pub fn new(ctx: &mut Context,
               assets: &Assets,
               final_score_value: u32,
               final_cleared: u32)
               -> GameResult<GameOverState> {
        let game_over = graphics::Text::new(ctx, "GAME OVER", assets.get_font("title")?)?;

        let instruction_src = "'R' to restart / 'M' for menu / 'Esc' to quit";
        let instruction_text =
            graphics::Text::new(ctx, instruction_src, assets.get_font("normal")?)?;

        let score_str = format!("Final Score: {}", final_score_value);
        let lines_str = format!("Final Lines: {}", final_cleared);
        let final_score = graphics::Text::new(ctx, &score_str, assets.get_font("normal")?)?;
        let final_lines = graphics::Text::new(ctx, &lines_str, assets.get_font("normal")?)?;

        Ok(GameOverState {
            request_menu: false,
            request_replay: false,

            game_over_text: game_over,
            final_score_text: final_score,
            final_line_text: final_lines,
            instruction_text: instruction_text,
        })
    }
}

impl EventHandler for GameOverState {
    fn update(&mut self,
              ctx: &mut Context,
              assets: &Assets,
              _: Duration)
              -> GameResult<Transition> {
        if self.request_menu {
            return Ok(Transition::Swap(Box::new(MenuState::new(ctx, assets)?)));
        } else if self.request_replay {
            return Ok(Transition::Swap(Box::new(PlayState::new(ctx, assets)?)));
        }

        Ok(Transition::None)
    }

    fn draw(&mut self, ctx: &mut Context, _: &Assets) -> GameResult<()> {
        let coords = graphics::get_screen_coordinates(&ctx);

        let game_over_dest = graphics::Point::new(coords.w / 2.0, 100.0);
        let game_over_score_dest = graphics::Point::new(coords.w / 2.0, 200.0);
        let game_over_lines_dest = graphics::Point::new(coords.w / 2.0, 250.0);
        let game_over_end_dest = graphics::Point::new(coords.w / 2.0, 400.0);

        graphics::set_color(ctx, graphics::Color::new(0.0, 0.0, 0.0, 0.7))?;
        graphics::rectangle(ctx,
                            graphics::DrawMode::Fill,
                            graphics::Rect::new(0.0 + (coords.w / 2.0),
                                                0.0 + ((coords.h * -1.0) / 2.0),
                                                coords.w,
                                                coords.h * -1.0))?;
        graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0))?;
        graphics::draw(ctx, &self.game_over_text, game_over_dest, 0.0)?;
        graphics::draw(ctx, &self.final_score_text, game_over_score_dest, 0.0)?;
        graphics::draw(ctx, &self.final_line_text, game_over_lines_dest, 0.0)?;
        graphics::draw(ctx, &self.instruction_text, game_over_end_dest, 0.0)?;


        Ok(())
    }

    fn key_down_event(&mut self, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::R => self.request_replay = true,
            Keycode::M => self.request_menu = true,
            _ => (),
        }
    }
}
