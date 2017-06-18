//! This is a custom version of the ggez `event.rs` module.
//! The `event` module contains traits and structs to actually run your game mainloop
//! and handle top-level state, as well as handle input events such as keyboard
//! and mouse.

use std::collections::HashMap;

/// A key code.
pub use sdl2::keyboard::Keycode;

/// A struct that holds the state of modifier buttons such as ctrl or shift.
pub use sdl2::keyboard::Mod;
/// A mouse button press.
pub use sdl2::mouse::MouseButton;
/// A struct containing the mouse state at a given instant.
pub use sdl2::mouse::MouseState;

/// A controller button.
pub use sdl2::controller::Button;
/// A controller axis.
pub use sdl2::controller::Axis;

use sdl2::event::Event::*;
use sdl2::event;
use sdl2::mouse;
use sdl2::keyboard;


use ggez::graphics;
use ggez::{GameResult, GameError, Context};
use ggez::timer;

use std::time::Duration;

pub struct Assets {
    images: HashMap<String, graphics::Image>,
    font: HashMap<String, graphics::Font>,
}

impl Assets {
    pub fn new() -> Self {
        Self {
            images: HashMap::new(),
            font: HashMap::new(),
        }
    }

    pub fn add_image(&mut self, name: &str, image: graphics::Image) -> GameResult<()> {
        self.images.insert(name.to_string(), image);
        Ok(())
    }

    pub fn get_image(&self, name: &str) -> GameResult<&graphics::Image> {
        let img = self.images.get(name);
        Ok(img.unwrap())
    }

    pub fn add_font(&mut self, name: &str, font: graphics::Font) -> GameResult<()> {
        self.font.insert(name.to_string(), font);
        Ok(())
    }

    pub fn get_font(&self, name: &str) -> GameResult<&graphics::Font> {
        let font = self.font.get(name);
        Ok(font.unwrap())
    }
}

pub enum Transition {
    None,
    Push(Box<EventHandler>), // Pushes another state on the stack
    Swap(Box<EventHandler>), // Removes current state from stack before adding the new one
    Pop, // Remove state on top of stack
}


/// A trait defining event callbacks; your primary interface with
/// `ggez`'s event loop.  Have a type implement this trait and
/// override at least the update() and draw() methods, then pass it to
/// `event::run()` to run the game's mainloop.
///
/// The default event handlers do nothing, apart from
/// `key_down_event()`, which will by default exit the game if escape
/// is pressed.  Just override the methods you want to do things with.
pub trait EventHandler {
    /// Called upon each physics update to the game.
    /// This should be where the game's logic takes place.
    fn update(&mut self,
              ctx: &mut Context,
              assets: &Assets,
              dt: Duration)
              -> GameResult<Transition>;

    /// Called to do the drawing of your game.
    /// You probably want to start this with
    /// `graphics::clear()` and end it with
    /// `graphics::present()` and `timer::sleep_until_next_frame()`
    fn draw(&mut self, ctx: &mut Context, assets: &Assets) -> GameResult<()>;

    fn mouse_button_down_event(&mut self, _button: mouse::MouseButton, _x: i32, _y: i32) {}

    fn mouse_button_up_event(&mut self, _button: mouse::MouseButton, _x: i32, _y: i32) {}

    fn mouse_motion_event(&mut self,
                          _state: mouse::MouseState,
                          _x: i32,
                          _y: i32,
                          _xrel: i32,
                          _yrel: i32) {
    }

    fn mouse_wheel_event(&mut self, _x: i32, _y: i32) {}

    fn key_down_event(&mut self, _keycode: Keycode, _keymod: Mod, _repeat: bool) {}

    fn key_up_event(&mut self, _keycode: Keycode, _keymod: Mod, _repeat: bool) {}

    fn controller_button_down_event(&mut self, _btn: Button, _instance_id: i32) {}
    fn controller_button_up_event(&mut self, _btn: Button, _instance_id: i32) {}
    fn controller_axis_event(&mut self, _axis: Axis, _value: i16, _instance_id: i32) {}

    fn focus_event(&mut self, _gained: bool) {}

    /// Called upon a quit event.  If it returns true,
    /// the game does not exit.
    fn quit_event(&mut self) -> bool {
        println!("Quitting game");
        false
    }
}
