use std::time::Duration;
use ggez::{Context, GameResult, graphics};
use ggez::graphics::Point;
use event::{Assets, Transition, EventHandler, Keycode, Mod, Button};
mod option;

use self::option::Option;
use states::play_state::{PlayState, Position};
use states::play_state::tetromino::{Piece, PieceType};
use util::DurationExt;

/// Different representations of possible commands that could be received from
/// the user in the menu state.
enum MenuInputCommand {
    Up,
    Down,
    Select,
}

pub struct MenuState {
    title_text: graphics::Text,
    title_rotation: f64,
    piece: Piece,
    options: Vec<Option>,
    current_selection: usize,

    request_play: bool,
    request_exit: bool,
}

impl MenuState {
    pub fn new(ctx: &mut Context, assets: &Assets) -> GameResult<MenuState> {
        let title = graphics::Text::new(ctx, "Rustris", assets.get_font("title")?)?;
        let mut options_vec: Vec<Option> = Vec::new();
        let coords = graphics::get_screen_coordinates(&ctx);

        options_vec.push(Option::new(ctx, assets, "Play!", Point::new(coords.w / 2.0, 250.0)));
        options_vec.push(Option::new(ctx, assets, "Exit", Point::new(coords.w / 2.0, 325.0)));

        Ok(MenuState {
            title_text: title,
            title_rotation: 0.0,
            piece: Piece::new(PieceType::L),
            options: options_vec,
            current_selection: 0,

            request_play: false,
            request_exit: false,
        })
    }

    fn handle_input(&mut self, command: MenuInputCommand) {
        match command {
            MenuInputCommand::Up => {
                if self.current_selection <= 0 {
                    self.current_selection = self.options.len() - 1;
                }
                else {
                    self.current_selection -= 1;
                }
            },
            MenuInputCommand::Down => {
                if self.current_selection >= self.options.len() - 1{
                    self.current_selection = 0;
                }
                else {
                    self.current_selection += 1;
                }
            },
            MenuInputCommand::Select => {
                if self.current_selection == 0 {
                    self.request_play = true;
                }
                else if self.current_selection == 1 {
                    self.request_exit = true;
                }
            },
        }
    }
}

impl EventHandler for MenuState {
    fn update(&mut self,
              ctx: &mut Context,
              assets: &Assets,
              dt: Duration)
              -> GameResult<Transition> {
        self.title_rotation += dt.as_subsec_millis();

        if self.request_play {
            return Ok(Transition::Swap(Box::new(PlayState::new(ctx, assets).unwrap())));
        }
        if self.request_exit {
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
        let coords = graphics::get_screen_coordinates(&ctx);

        let title_dest = graphics::Point::new(coords.w / 2.0, 100.0);
        let pos = Position::new(5, 50);

        graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0))?;
        graphics::draw(ctx, &self.title_text, title_dest, 0.0)?;
        self.piece.draw_at_point(ctx, assets.get_image("block")?, pos, self.title_rotation)?;

        for option in &self.options {
            option.draw(ctx)?;
        }

        Ok(())
    }

    fn key_down_event(&mut self, keycode: Keycode, _keymod: Mod, repeat: bool) {
        if repeat {
            return;
        }

        match keycode {
            Keycode::Up => self.handle_input(MenuInputCommand::Up),
            Keycode::Down => self.handle_input(MenuInputCommand::Down),
            Keycode::Return => self.handle_input(MenuInputCommand::Select),
            _ => (),
        }
    }

    fn controller_button_down_event(&mut self, btn: Button, _instance_id: i32) {
        match btn {
            Button::DPadUp => self.handle_input(MenuInputCommand::Up),
            Button::DPadDown => self.handle_input(MenuInputCommand::Down),
            Button::A => self.handle_input(MenuInputCommand::Select),
            _ => (),
        }
    }
}
