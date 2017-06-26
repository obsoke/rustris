// The MIT License (MIT)
// Copyright (c) 2016-2017 ggez-dev
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

// This is a custom version of the ggez `event.rs` module. Why a custom
// version?
//
// 1) I needed a way to have `update()` and `draw()` to return
// `GameResult<Transition>` for my state management system.
//
// 2) I needed to add said `Transition` enum to `event.rs`. My attempts at
// making `EventHandler` into a generic so the return type can be generic have
// failed when I ran into lifetime issues when puting `EventHandler<T>` into a
// `Box`.` The easiest solution around this was to add `Transition` to this
// module directly.
//
// 3) Additionally, I wanted my state manager to own an `Assets` struct so I
// could load my game assets once and pass it to whatever state needed it. To do
// this, I had to alter the signatures of both `update()` and `draw()` to take a
// reference to the `Assets` struct.


//! The `event` module contains traits and structs to actually run your game mainloop
//! and handle top-level state, as well as handle input events such as keyboard
//! and mouse.

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

use std::collections::HashMap;
use std::time::Duration;

use sdl2::event::Event::*;
use sdl2::event as SdlEvent;
use sdl2::mouse;
use sdl2::keyboard;

use ggez::graphics;
use ggez::audio;
use ggez::{GameResult, Context};
use ggez::timer;

use states::StateManager;

pub struct Assets {
    images: HashMap<String, graphics::Image>,
    font: HashMap<String, graphics::Font>,
    music: HashMap<String, audio::Source>,
    sfx: HashMap<String, audio::Source>,
}

impl Assets {
    pub fn new() -> Self {
        Self {
            images: HashMap::new(),
            font: HashMap::new(),
            music: HashMap::new(),
            sfx: HashMap::new(),
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

    pub fn add_music(&mut self, name: &str, audio: audio::Source) -> GameResult<()> {
        self.music.insert(name.to_string(), audio);
        Ok(())
    }

    pub fn get_music(&self, name: &str) -> GameResult<&audio::Source> {
        let audio = self.music.get(name);
        Ok(audio.unwrap())
    }

    pub fn get_music_count(&self) -> u32 {
        self.music.len() as u32
    }

    pub fn add_sfx(&mut self, name: &str, audio: audio::Source) -> GameResult<()> {
        self.sfx.insert(name.to_string(), audio);
        Ok(())
    }

    pub fn get_sfx(&self, name: &str) -> GameResult<&audio::Source> {
        let audio = self.sfx.get(name);
        Ok(audio.unwrap())
    }
}

pub enum Transition {
    None,
    Push(Box<EventHandler>), // Pushes another state on the stack
    Swap(Box<EventHandler>), // Removes current state from stack before adding the new one
    Pop, // Remove state on top of stack
    Drain, // Clear the stack, quitting the game
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
    fn update(
        &mut self,
        ctx: &mut Context,
        assets: &Assets,
        dt: Duration,
    ) -> GameResult<Transition>;

    /// Called to do the drawing of your game.
    /// You probably want to start this with
    /// `graphics::clear()` and end it with
    /// `graphics::present()` and `timer::sleep_until_next_frame()`
    fn draw(&mut self, ctx: &mut Context, assets: &Assets) -> GameResult<()>;

    fn mouse_button_down_event(&mut self, _button: mouse::MouseButton, _x: i32, _y: i32) {}

    fn mouse_button_up_event(&mut self, _button: mouse::MouseButton, _x: i32, _y: i32) {}

    fn mouse_motion_event(
        &mut self,
        _state: mouse::MouseState,
        _x: i32,
        _y: i32,
        _xrel: i32,
        _yrel: i32,
    ) {
    }

    fn mouse_wheel_event(&mut self, _x: i32, _y: i32) {}

    fn key_down_event(&mut self, _keycode: Keycode, _keymod: Mod, _repeat: bool, _assets: &Assets) {
    }

    fn key_up_event(&mut self, _keycode: Keycode, _keymod: Mod, _repeat: bool) {}

    fn controller_button_down_event(&mut self, _btn: Button, _instance_id: i32, _assets: &Assets) {}
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

/// Runs the game's main loop, calling event callbacks on the given state
/// object as events occur.
///
/// This is a custom version of ggez's event method. Since I added both an
/// `Asset` and `StateManager`, I needed to tweak the default game loop a
/// bit to update managers rather than states directly.
pub fn run(ctx: &mut Context) -> GameResult<()> {
    {
        let mut assets = Assets::new();
        assets.add_image(
            "block",
            graphics::Image::new(ctx, "/block.png")?,
        )?;
        assets.add_font(
            "title",
            graphics::Font::new(ctx, "/DejaVuSansMono.ttf", 32)?,
        )?;
        assets.add_font(
            "normal",
            graphics::Font::new(ctx, "/DejaVuSansMono.ttf", 18)?,
        )?;

        let mut play_0 = audio::Source::new(ctx, "/music/Track2.ogg")?;
        let mut play_1 = audio::Source::new(ctx, "/music/Track4.ogg")?;
        let mut menu = audio::Source::new(ctx, "/music/Track3.ogg")?;
        play_0.set_volume(0.4);
        play_1.set_volume(0.4);
        menu.set_volume(0.4);
        assets.add_music("play_0", play_0)?;
        assets.add_music("play_1", play_1)?;
        assets.add_music("menu", menu)?;

        let mut click = audio::Source::new(ctx, "/sfx/click.ogg")?;
        let mut gameover_win = audio::Source::new(ctx, "/sfx/gameover_win.ogg")?;
        let mut gameover_lose = audio::Source::new(ctx, "/sfx/gameover_lose.ogg")?;
        click.set_volume(0.5);
        gameover_win.set_volume(0.5);
        gameover_lose.set_volume(0.5);
        assets.add_sfx("click", click)?;
        assets.add_sfx("gameover_win", gameover_win)?;
        assets.add_sfx("gameover_lose", gameover_lose)?;

        let mut state_manager = StateManager::new(ctx, &assets);

        let mut event_pump = ctx.sdl_context.event_pump()?;

        while state_manager.is_running() {
            ctx.timer_context.tick();

            for event in event_pump.poll_iter() {
                match event {
                    Quit { .. } => {
                        state_manager.quit();
                        // println!("Quit event: {:?}", t);
                    }
                    KeyDown {
                        keycode,
                        keymod,
                        repeat,
                        ..
                    } => {
                        if let Some(key) = keycode {
                            if key == keyboard::Keycode::Escape {
                                ctx.quit()?;
                            } else {
                                state_manager.key_down_event(key, keymod, repeat, &assets)
                            }
                        }
                    }
                    KeyUp {
                        keycode,
                        keymod,
                        repeat,
                        ..
                    } => {
                        if let Some(key) = keycode {
                            state_manager.key_up_event(key, keymod, repeat)
                        }
                    }
                    MouseButtonDown { mouse_btn, x, y, .. } => {
                        state_manager.mouse_button_down_event(mouse_btn, x, y)
                    }
                    MouseButtonUp { mouse_btn, x, y, .. } => {
                        state_manager.mouse_button_up_event(mouse_btn, x, y)
                    }
                    MouseMotion {
                        mousestate,
                        x,
                        y,
                        xrel,
                        yrel,
                        ..
                    } => state_manager.mouse_motion_event(mousestate, x, y, xrel, yrel),
                    MouseWheel { x, y, .. } => state_manager.mouse_wheel_event(x, y),
                    ControllerButtonDown { button, which, .. } => {
                        state_manager.controller_button_down_event(button, which, &assets)
                    }
                    ControllerButtonUp { button, which, .. } => {
                        state_manager.controller_button_up_event(button, which)
                    }
                    ControllerAxisMotion { axis, value, which, .. } => {
                        state_manager.controller_axis_event(axis, value, which)
                    }
                    Window { win_event: SdlEvent::WindowEvent::FocusGained, .. } => {
                        state_manager.focus_event(true)
                    }
                    Window { win_event: SdlEvent::WindowEvent::FocusLost, .. } => {
                        state_manager.focus_event(false)
                    }
                    _ => {}
                }
            }

            let dt = timer::get_delta(ctx);
            state_manager.update(ctx, &assets, dt)?;
            state_manager.draw(ctx, &assets)?;
        }
    }

    Ok(())
}
