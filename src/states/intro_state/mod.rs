use std::time::Duration;
use ggez::{Context, GameResult, graphics};
use ggez::event::{Mod, Keycode};
use crate::event::{Assets, Transition, EventHandler, Button};
use crate::states::menu_state::MenuState;
use crate::util::DurationExt;

const FADE_TIME: f32 = 3.0;
const WAIT_TIME: f32 = 1.5;

pub struct IntroState {
    intro_text: graphics::Text,
    hit_any_key: bool,
    fader: f32,
    waiter: f32,
    fade_in: f32,
    fade_out: f32,
}

impl IntroState {
    /// A `GameEndState` takes values from `PlayState` to render certain values
    /// such as no. of lines cleared, highest level cleared, final score, etc.
    pub fn new(ctx: &mut Context, assets: &Assets) -> GameResult<Self> {
        let intro_text = graphics::Text::new(
            ctx,
            "a game by obsoke",
            assets.get_font("normal")?,
        )?;


        Ok(IntroState {
            intro_text: intro_text,
            hit_any_key: false,
            fader: 0.0,
            waiter: WAIT_TIME,
            fade_in: 0.0,
            fade_out: FADE_TIME,
        })
    }

    pub fn handle_input(&mut self) {
        self.hit_any_key = true;
    }
}

impl EventHandler for IntroState {
    fn update(
        &mut self,
        ctx: &mut Context,
        assets: &Assets,
        dt: Duration,
    ) -> GameResult<Transition> {

        if self.hit_any_key {
            return Ok(Transition::Swap(Box::new(MenuState::new(ctx, assets)?)));
        }

        if self.fade_in < FADE_TIME {
            self.fade_in += dt.as_subsec_millis() as f32;
            self.fader = self.fade_in;
        } else if self.waiter > 0.0 {
            self.waiter -= dt.as_subsec_millis() as f32;
        } else if self.fade_out > 0.0 {
            self.fade_out -= dt.as_subsec_millis() as f32;
            self.fader = self.fade_out;
        } else {
            return Ok(Transition::Swap(Box::new(MenuState::new(ctx, assets)?)));
        }

        Ok(Transition::None)
    }

    fn draw(&mut self, ctx: &mut Context, _: &Assets) -> GameResult<()> {
        let coords = graphics::get_screen_coordinates(ctx);

        let intro_text_dest = graphics::Point::new(coords.w / 2.0, 300.0);

        graphics::set_color(
            ctx,
            graphics::Color::new(1.0, 1.0, 1.0, self.fader / FADE_TIME),
        )?;
        graphics::draw(ctx, &self.intro_text, intro_text_dest, 0.0)?;

        Ok(())
    }

    fn key_down_event(&mut self, keycode: Keycode, _keymod: Mod, repeat: bool, _assets: &Assets) {
        if repeat {
            return;
        }

        match keycode {
            _ => self.handle_input(),
        }
    }

    fn controller_button_down_event(&mut self, btn: Button, _instance_id: i32, _assets: &Assets) {
        match btn {
            _ => self.handle_input(),
        }
    }
}
