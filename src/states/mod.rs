pub mod game_over_state;
pub mod intro_state;
pub mod menu_state;
pub mod play_state;
pub mod shared;

use crate::states::intro_state::IntroState;
use ggez::event::{Axis, Button, EventHandler, Keycode, Mod, MouseButton, MouseState};
use ggez::{audio, graphics, timer, Context, GameResult};
use std::collections::HashMap;
use std::time::Duration;

/// A global structure that stores all game assets. This is passed down into a
/// state by the `StateManager`.
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

    /// Add an image asset to the asset manager.
    pub fn add_image(&mut self, name: &str, image: graphics::Image) -> GameResult<()> {
        self.images.insert(name.to_string(), image);
        Ok(())
    }

    /// Returns an image with the key `name` from the asset manager.
    pub fn get_image(&self, name: &str) -> GameResult<&graphics::Image> {
        let img = self.images.get(name);
        Ok(img.unwrap())
    }

    /// Add a font asset to the asset manager.
    pub fn add_font(&mut self, name: &str, font: graphics::Font) -> GameResult<()> {
        self.font.insert(name.to_string(), font);
        Ok(())
    }

    /// Returns a font with the key `name` from the asset manager.
    pub fn get_font(&self, name: &str) -> GameResult<&graphics::Font> {
        let font = self.font.get(name);
        Ok(font.unwrap())
    }

    /// Add an audio asset meant for music to the asset manager.
    pub fn add_music(&mut self, name: &str, audio: audio::Source) -> GameResult<()> {
        self.music.insert(name.to_string(), audio);
        Ok(())
    }

    /// Returns an audio asset meant for music with the key `name` from the
    /// asset manager.
    pub fn get_music(&self, name: &str) -> GameResult<&audio::Source> {
        let audio = self.music.get(name);
        Ok(audio.unwrap())
    }

    /// Returns the number of audio assets meant for music playback currently in
    /// the asset manager.
    pub fn get_music_count(&self) -> u32 {
        self.music.len() as u32
    }

    /// Add an audio asset meant for sound effects to the asset manager.
    pub fn add_sfx(&mut self, name: &str, audio: audio::Source) -> GameResult<()> {
        self.sfx.insert(name.to_string(), audio);
        Ok(())
    }

    /// Returns an audio asset meant for sound effects with the key `name` from
    /// the asset manager.
    pub fn get_sfx(&self, name: &str) -> GameResult<&audio::Source> {
        let audio = self.sfx.get(name);
        Ok(audio.unwrap())
    }
}

/// Describes a potential state transition. `EventHandler::update` returns a
/// `GameResult<Transition>` that can be used to request a state change from the
/// state manager.
pub enum Transition {
    /// Don't change states.
    None,
    /// Push another state ontop of the current state.
    Push(Box<dyn State>),
    /// Remove all states from the stack and then push a new one.
    Swap(Box<dyn State>),
    /// Remove the state currently at the top of the stack.
    Pop,
    /// Clear the stack which in turn quits the game.
    Drain,
}

pub trait State {
    fn update(
        &mut self,
        ctx: &mut Context,
        assets: &Assets,
        dt: Duration,
    ) -> GameResult<Transition>;
    fn draw(&mut self, ctx: &mut Context, assets: &Assets) -> GameResult<()>;
    fn key_down_event(&mut self, _keycode: Keycode, _keymod: Mod, _repeat: bool, _assets: &Assets) {
    }
    fn key_up_event(&mut self, _keycode: Keycode, _keymod: Mod, _repeat: bool) {}
    fn mouse_button_down_event(&mut self, _button: MouseButton, _x: i32, _y: i32) {}
    fn mouse_button_up_event(&mut self, _button: MouseButton, _x: i32, _y: i32) {}
    fn mouse_motion_event(&mut self, _state: MouseState, _x: i32, _y: i32, _xrel: i32, _yrel: i32) {
    }
    fn mouse_wheel_event(&mut self, _x: i32, _y: i32) {}
    fn controller_button_down_event(&mut self, _btn: Button, _instance_id: i32, _assets: &Assets) {}
    fn controller_button_up_event(&mut self, _btn: Button, _instance_id: i32) {}
    fn controller_axis_event(&mut self, _axis: Axis, _value: i16, _instance_id: i32) {}
    fn focus_event(&mut self, _gained: bool) {}
    fn quit_event(&mut self) -> bool {
        false
    }
}

/// A `StateManager` will manage requests to push, pop or swap states on the
/// state stack. It owns the `Assets` struct and dictates whether the game
/// continues to run or not.
///
/// States must be put into a `Box` before being handed to the `StateManager`.
pub struct StateManager {
    assets: Assets,
    running: bool,
    states: Vec<Box<dyn State>>,
}

impl StateManager {
    /// Create a new `StateManager` and initializes the first state.
    pub fn new(ctx: &mut Context) -> StateManager {
        let assets = StateManager::initialize_assets(ctx).unwrap();
        let state = Box::new(IntroState::new(ctx, &assets).unwrap());

        StateManager {
            running: true,
            states: vec![state],
            assets,
        }
    }

    fn initialize_assets(ctx: &mut Context) -> GameResult<Assets> {
        let mut assets = Assets::new();
        assets.add_image("block", graphics::Image::new(ctx, "/block.png")?)?;
        assets.add_image("menu_bg", graphics::Image::new(ctx, "/blackhole2.png")?)?;
        assets.add_image("game_bg", graphics::Image::new(ctx, "/space_bg.png")?)?;
        assets.add_font(
            "title",
            graphics::Font::new(ctx, "/DejaVuSansMono.ttf", 32)?,
        )?;
        assets.add_font(
            "title_shadow",
            graphics::Font::new(ctx, "/DejaVuSansMono.ttf", 33)?,
        )?;
        assets.add_font(
            "normal",
            graphics::Font::new(ctx, "/DejaVuSansMono.ttf", 18)?,
        )?;
        assets.add_font("ui", graphics::Font::new(ctx, "/DejaVuSansMono.ttf", 16)?)?;

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

        Ok(assets)
    }

    /// Clears the state stack and sets `running` to false which quits the game.
    pub fn quit(&mut self) {
        self.states.clear();
        self.running = false
    }
}

impl StateManager {
    /// Calls the state transition handler depending on the `Transition` given
    /// as an argument.
    fn handle_transition(&mut self, transition: Transition) {
        match transition {
            Transition::None => (),
            Transition::Pop => self.pop(),
            Transition::Swap(state) => self.swap(state),
            Transition::Push(state) => self.push(state),
            Transition::Drain => self.drain(),
        }
    }

    /// Pops the state at the top of the stack. If the size of the state stack
    /// after this operation is `0`, the game quits.
    fn pop(&mut self) {
        self.states.pop();

        if self.states.is_empty() {
            self.quit();
        }
    }

    /// Pushes a state onto the state stack.
    fn push(&mut self, boxed_state: Box<dyn State>) {
        self.states.push(boxed_state)
    }

    /// Removes all states from the state stack and then pushes the given
    /// `boxed_state` onto the stack.
    fn swap(&mut self, boxed_state: Box<dyn State>) {
        self.states.clear();
        self.push(boxed_state);
    }

    /// Removes all states from the state stack and quits the game.
    fn drain(&mut self) {
        self.states.clear();
        self.quit();
    }
}

impl EventHandler for StateManager {
    fn update(&mut self, ctx: &mut Context, dt: Duration) -> GameResult<()> {
        if !self.running {
            ctx.quit()?;
        }

        let transition = match self.states.last_mut() {
            Some(state) => state.update(ctx, &self.assets, dt),
            None => Ok(Transition::None),
        };

        self.handle_transition(transition?);

        Ok(())
    }

    /// `StateManager::draw` handles drawing for all states on the stack. This
    /// enables the use of having pause screens or other states render on top of
    /// the current state. Due to this, `StateManager::draw` also controls both
    /// clearing and swapping of buffers.
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // draw everything in the stack
        use ggez::graphics;

        graphics::set_background_color(ctx, graphics::Color::new(0.0, 0.0, 0.0, 255.0));
        graphics::clear(ctx);

        for (_, state) in self.states.iter_mut().enumerate() {
            state.draw(ctx, &self.assets)?;
        }

        graphics::present(ctx);
        timer::sleep(Duration::from_secs(0));
        Ok(())
    }
    fn mouse_button_down_event(&mut self, _button: MouseButton, _x: i32, _y: i32) {
        if let Some(state) = self.states.last_mut() {
            state.mouse_button_down_event(_button, _x, _y);
        }
    }

    fn mouse_button_up_event(&mut self, _button: MouseButton, _x: i32, _y: i32) {
        if let Some(state) = self.states.last_mut() {
            state.mouse_button_up_event(_button, _x, _y);
        }
    }

    fn mouse_motion_event(&mut self, _state: MouseState, _x: i32, _y: i32, _xrel: i32, _yrel: i32) {
        if let Some(state) = self.states.last_mut() {
            state.mouse_motion_event(_state, _x, _y, _xrel, _yrel);
        }
    }

    fn mouse_wheel_event(&mut self, _x: i32, _y: i32) {
        if let Some(state) = self.states.last_mut() {
            state.mouse_wheel_event(_x, _y);
        }
    }

    fn key_down_event(&mut self, _keycode: Keycode, _keymod: Mod, _repeat: bool) {
        if let Some(state) = self.states.last_mut() {
            state.key_down_event(_keycode, _keymod, _repeat, &self.assets);
        }
    }

    fn key_up_event(&mut self, _keycode: Keycode, _keymod: Mod, _repeat: bool) {
        if let Some(state) = self.states.last_mut() {
            state.key_up_event(_keycode, _keymod, _repeat);
        }
    }

    fn controller_button_down_event(&mut self, _btn: Button, _instance_id: i32) {
        if let Some(state) = self.states.last_mut() {
            state.controller_button_down_event(_btn, _instance_id, &self.assets);
        }
    }
    fn controller_button_up_event(&mut self, _btn: Button, _instance_id: i32) {
        if let Some(state) = self.states.last_mut() {
            state.controller_button_up_event(_btn, _instance_id);
        }
    }
    fn controller_axis_event(&mut self, _axis: Axis, _value: i16, _instance_id: i32) {
        if let Some(state) = self.states.last_mut() {
            state.controller_axis_event(_axis, _value, _instance_id);
        }
    }

    fn focus_event(&mut self, _gained: bool) {
        if let Some(state) = self.states.last_mut() {
            state.focus_event(_gained);
        }
    }

    /// Called upon a quit event.  If it returns true,
    /// the game does not exit.
    fn quit_event(&mut self) -> bool {
        match self.states.last_mut() {
            Some(state) => state.quit_event(),
            None => false,
        }
    }
}
