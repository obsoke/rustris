use std::time::Duration;
use super::util::DurationExt;
use ggez::{Context, GameResult, graphics, event};
use ggez::event::{Assets,  Transition};
use ggez::event::*;

pub struct GameOverState {
    game_over_text: graphics::Text,
    final_score_text: graphics::Text,
    final_line_text: graphics::Text,
    instruction_text: graphics::Text,
}

impl GameOverState {
    pub fn new(ctx: &mut Context, assets: &Assets) -> GameResult<GameOverState> {
        let game_over = graphics::Text::new(ctx, "GAME OVER", assets.get_font("title")?)?;

        let instruction_src = "'R' to restart / 'M' for menu / 'Esc' to quit";
        let instruction_text = graphics::Text::new(ctx, instruction_src, assets.get_font("normal")?)?;
        let final_score = graphics::Text::new(ctx, "Final Score", assets.get_font("normal")?)?;
        let final_lines = graphics::Text::new(ctx, "Final Lines", assets.get_font("normal")?)?;

        Ok(GameOverState {
            game_over_text: game_over,
            final_score_text: final_score,
            final_line_text: final_lines,
            instruction_text: instruction_text,
        })
    }
}

impl event::EventHandler for GameOverState {
    fn update(&mut self, ctx: &mut Context, assets: &Assets, dt: Duration) -> GameResult<Transition> {
        Ok(Transition::None)
    }

    fn draw(&mut self, ctx: &mut Context, assets: &Assets) -> GameResult<()> {
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
        graphics::draw(ctx, &self.final_score_text, game_over_score_dest, 0.0)?;
        graphics::draw(ctx, &self.final_line_text, game_over_lines_dest, 0.0)?;
        graphics::draw(ctx, &self.instruction_text, game_over_end_dest, 0.0)?;


        Ok(())
    }

    fn key_down_event(&mut self, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        // match keycode {
        //     Keycode::Left => self.input.left.is_active = true,
        //     Keycode::Right => self.input.right.is_active = true,
        //     Keycode::Up => self.input.hard_drop.is_active = true,
        //     Keycode::Down => self.input.soft_drop.is_active = true,
        //     Keycode::Z => self.input.rotate_counterclockwise.is_active = true,
        //     Keycode::X => self.input.rotate_clockwise.is_active = true,
        //     _ => (),
        // }

    }

    fn key_up_event(&mut self, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        // match keycode {
        //     Keycode::Left => {
        //         self.input.left.is_active = false;
        //         self.input.left.delay_timer = INPUT_DELAY_TIME;
        //     }
        //     Keycode::Right => {
        //         self.input.right.is_active = false;
        //         self.input.right.delay_timer = INPUT_DELAY_TIME;
        //     },
        //     Keycode::Up => {
        //         self.input.hard_drop.is_active = false;
        //         self.input.hard_drop.delay_timer = INPUT_DELAY_TIME;
        //     },
        //     Keycode::Down => {
        //         self.input.soft_drop.is_active = false;
        //         self.input.soft_drop.delay_timer = INPUT_DELAY_TIME;
        //     },
        //     Keycode::Z => {
        //         self.input.rotate_counterclockwise.is_active = false;
        //         self.input.rotate_counterclockwise.delay_timer = INPUT_DELAY_TIME;
        //     },
        //     Keycode::X => {
        //         self.input.rotate_clockwise.is_active = false;
        //         self.input.rotate_clockwise.delay_timer = INPUT_DELAY_TIME;
        //     },
        //     _ => (),
        // }
    }
}
