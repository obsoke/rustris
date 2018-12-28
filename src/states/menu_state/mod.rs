mod spawner;

use std::time::Duration;
use ggez::{Context, GameResult, graphics};
use ggez::event::{Keycode, Mod, Button};
use ggez::graphics::{Point, Color};
use crate::states::shared::option::{Option, OptionInputCommand};
use crate::states::play_state::PlayState;
use crate::states::{State, Assets, Transition};
use crate::util::{DurationExt, play_click_sfx};
use self::spawner::Spawner;

pub struct MenuState {
    title_text: graphics::Text,
    title_shadow: graphics::Text,
    title_rotation: f64,
    piece_spawner: Spawner,
    options: Vec<Option>,
    current_selection: usize,

    request_play: bool,
    request_exit: bool,
}

impl MenuState {
    pub fn new(ctx: &mut Context, assets: &Assets) -> GameResult<MenuState> {
        let title = graphics::Text::new(ctx, "Rustris", assets.get_font("title")?)?;
        let title_shadow = graphics::Text::new(ctx, "Rustris", assets.get_font("title_shadow")?)?;

        let coords = graphics::get_screen_coordinates(ctx);
        let mut options_vec: Vec<Option> = Vec::new();
        options_vec.push(Option::new(
            ctx,
            assets,
            "Play!",
            Point::new(coords.w / 2.0, 250.0),
        ));
        options_vec.push(Option::new(
            ctx,
            assets,
            "Exit",
            Point::new(coords.w / 2.0, 325.0),
        ));

        Ok(MenuState {
            title_text: title,
            title_shadow: title_shadow,
            title_rotation: 0.0,
            piece_spawner: Spawner::new(),
            options: options_vec,
            current_selection: 0,

            request_play: false,
            request_exit: false,
        })
    }

    // Ideally, I would not be coupling `Assets` to this method. Would a
    // messaging system be fast enough to handle audio system stuff? Maybe
    // something to try for v2.
    fn handle_input(&mut self, command: OptionInputCommand, assets: &Assets) {
        match command {
            OptionInputCommand::Up => {
                play_click_sfx(assets).expect("Could not play click sfx in menu state -> up");
                if self.current_selection == 0 {
                    self.current_selection = self.options.len() - 1;
                } else {
                    self.current_selection -= 1;
                }
            }
            OptionInputCommand::Down => {
                play_click_sfx(assets).expect("Could not play click sfx in menu state -> down");
                if self.current_selection >= self.options.len() - 1 {
                    self.current_selection = 0;
                } else {
                    self.current_selection += 1;
                }
            }
            OptionInputCommand::Select => {
                if self.current_selection == 0 {
                    self.request_play = true;
                } else if self.current_selection == 1 {
                    self.request_exit = true;
                }
            }
        }
    }
}

impl State for MenuState {
    fn update(
        &mut self,
        ctx: &mut Context,
        assets: &Assets,
        dt: Duration,
    ) -> GameResult<Transition> {
        // play & loop menu theme
        let current_song = assets.get_music("menu")?;
        if current_song.paused() {
            current_song.resume();
        } else {
            current_song.play()?;
        }

        self.title_rotation += dt.as_subsec_millis();

        self.piece_spawner.update(ctx, assets, dt);

        if self.request_play {
            assets.get_music("menu")?.pause();
            return Ok(Transition::Swap(
                Box::new(PlayState::new(ctx, assets).unwrap()),
            ));
        } else if self.request_exit {
            assets.get_music("menu")?.pause();
            return Ok(Transition::Pop);
        }

        // for mut option in &mut self.options {
        for (i, option) in self.options.iter_mut().enumerate() {
            if i == self.current_selection {
                option.update(true)?;
            } else {
                option.update(false)?;
            }
        }

        Ok(Transition::None)
    }

    fn draw(&mut self, ctx: &mut Context, assets: &Assets) -> GameResult<()> {
        let coords = graphics::get_screen_coordinates(ctx);

        // draw background
        graphics::set_color(ctx, Color::new(1.0, 1.0, 1.0, 1.0))?;
        graphics::draw(
            ctx,
            assets.get_image("menu_bg")?,
            Point::new(coords.w / 2.0, coords.h / -2.0),
            self.title_rotation as f32 * 0.3,
        )?;

        // draw piece spawner & all spawned pieces
        self.piece_spawner.draw(ctx, assets);

        let title_dest = graphics::Point::new(coords.w / 2.0, 100.0);
        // shadow/stroke
        graphics::set_color(ctx, graphics::Color::new(0.0, 0.0, 0.0, 0.8))?;
        graphics::draw(ctx, &self.title_shadow, title_dest, 0.0)?;
        // main text
        graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0))?;
        graphics::draw(ctx, &self.title_text, title_dest, 0.0)?;

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
            Keycode::Up => self.handle_input(OptionInputCommand::Up, assets),
            Keycode::Down => self.handle_input(OptionInputCommand::Down, assets),
            Keycode::Return => self.handle_input(OptionInputCommand::Select, assets),
            _ => (),
        }
    }

    fn controller_button_down_event(&mut self, btn: Button, _instance_id: i32, assets: &Assets) {
        match btn {
            Button::DPadUp => self.handle_input(OptionInputCommand::Up, assets),
            Button::DPadDown => self.handle_input(OptionInputCommand::Down, assets),
            Button::A => self.handle_input(OptionInputCommand::Select, assets),
            _ => (),
        }
    }
}
