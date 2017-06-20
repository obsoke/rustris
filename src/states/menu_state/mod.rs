use std::time::Duration;
use ggez::{Context, GameResult, graphics};
use event::{Assets, Transition, EventHandler, Keycode, Mod};
use states::play_state::PlayState;
//use states::play_state::tetromino::Piece;
use util::DurationExt;

pub struct MenuState {
    title_text: graphics::Text,
    title_rotation: f64,
}

impl MenuState {
    pub fn new(ctx: &mut Context, assets: &Assets) -> GameResult<MenuState> {
        let title = graphics::Text::new(ctx, "Rustris", assets.get_font("title")?)?;
        Ok(MenuState {
            title_text: title,
            title_rotation: 0.0,
        })
    }
}

impl EventHandler for MenuState {
    fn update(&mut self,
              ctx: &mut Context,
              assets: &Assets,
              dt: Duration)
              -> GameResult<Transition> {
        println!("MENU STATE STUB!");
        self.title_rotation += dt.as_subsec_millis();
        Ok(Transition::None)
    }

    fn draw(&mut self, ctx: &mut Context, _: &Assets) -> GameResult<()> {
        let coords = graphics::get_screen_coordinates(&ctx);

        let title_dest = graphics::Point::new(coords.w / 2.0, 100.0);

        graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0))?;
        graphics::draw(ctx, &self.title_text, title_dest, 0.0)?;

        Ok(())
    }

    fn key_down_event(&mut self, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            // Keycode::R => self.request_replay = true,
            // Keycode::M => self.request_menu = true,
            _ => (),
        }
    }
}
