use std::time::Duration;
use ggez::{Context, GameResult, graphics};
use ggez::event::{Keycode, Mod};
use event::{Assets, Transition, EventHandler};
use states::play_state::PlayState;

pub struct MenuState {}

impl MenuState {
    pub fn new(ctx: &mut Context, assets: &Assets) -> GameResult<MenuState> {
        Ok(MenuState {})
    }
}

impl EventHandler for MenuState {
    fn update(&mut self,
              ctx: &mut Context,
              assets: &Assets,
              _: Duration)
              -> GameResult<Transition> {
        println!("MENU STATE STUB!");
        Ok(Transition::None)
    }

    fn draw(&mut self, ctx: &mut Context, _: &Assets) -> GameResult<()> {
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
