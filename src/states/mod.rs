pub mod menu_state;
pub mod play_state;
pub mod game_over_state;
pub mod intro_state;
pub mod shared;

use std::time::Duration;
use sdl2::mouse;
use ggez::{Context, GameResult, timer};
use event::{Assets, EventHandler, Transition, Keycode, Mod, Button, Axis};
use states::intro_state::IntroState;

/// A `StateManager` will manage requests to push, pop or swap states on the
/// state stack. It owns the `Assets` struct and dictates whether the game
/// continues to run or not.
///
/// States must be put into a `Box` before being handed to the `StateManager`.
pub struct StateManager {
    running: bool,
    states: Vec<Box<EventHandler>>,
}

impl StateManager {
    // TODO: Pass the initial state as an argument.
    /// Create a new `StateManager` and initializes the first state.
    pub fn new(ctx: &mut Context, assets: &Assets) -> StateManager {
        let state = Box::new(IntroState::new(ctx, assets).unwrap());

        StateManager {
            running: true,
            states: vec![state], // create empty state stack
        }
    }

    /// Returns `true` if the manager is running. If `false`, the game will
    /// quit.
    pub fn is_running(&self) -> bool {
        self.running
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
    fn push(&mut self, boxed_state: Box<EventHandler>) {
        self.states.push(boxed_state)
    }

    /// Removes all states from the state stack and then pushes the given
    /// `boxed_state` onto the stack.
    fn swap(&mut self, boxed_state: Box<EventHandler>) {
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
    fn update(
        &mut self,
        ctx: &mut Context,
        assets: &Assets,
        dt: Duration,
    ) -> GameResult<Transition> {
        let transition = match self.states.last_mut() {
            Some(state) => state.update(ctx, assets, dt),
            None => Ok(Transition::None),
        };

        self.handle_transition(transition?);

        Ok(Transition::None)
    }

    /// `StateManager::draw` handles drawing for all states on the stack. This
    /// enables the use of having pause screens or other states render on top of
    /// the current state. Due to this, `StateManager::draw` also controls both
    /// clearing and swapping of buffers.
    fn draw(&mut self, ctx: &mut Context, assets: &Assets) -> GameResult<()> {
        // draw everything in the stack
        use ggez::graphics;

        graphics::set_background_color(ctx, graphics::Color::new(0.0, 0.0, 0.0, 255.0));
        graphics::clear(ctx);

        for (_, state) in self.states.iter_mut().enumerate() {
            state.draw(ctx, assets)?;
        }

        graphics::present(ctx);
        timer::sleep(Duration::from_secs(0));
        Ok(())
    }
    fn mouse_button_down_event(&mut self, _button: mouse::MouseButton, _x: i32, _y: i32) {
        if let Some(state) = self.states.last_mut() {
            state.mouse_button_down_event(_button, _x, _y);
        }
    }

    fn mouse_button_up_event(&mut self, _button: mouse::MouseButton, _x: i32, _y: i32) {
        if let Some(state) = self.states.last_mut() {
            state.mouse_button_up_event(_button, _x, _y);
        }
    }

    fn mouse_motion_event(
        &mut self,
        _state: mouse::MouseState,
        _x: i32,
        _y: i32,
        _xrel: i32,
        _yrel: i32,
    ) {

        if let Some(state) = self.states.last_mut() {
            state.mouse_motion_event(_state, _x, _y, _xrel, _yrel);
        }
    }

    fn mouse_wheel_event(&mut self, _x: i32, _y: i32) {
        if let Some(state) = self.states.last_mut() {
            state.mouse_wheel_event(_x, _y);
        }
    }

    fn key_down_event(&mut self, _keycode: Keycode, _keymod: Mod, _repeat: bool, assets: &Assets) {
        if let Some(state) = self.states.last_mut() {
            state.key_down_event(_keycode, _keymod, _repeat, assets);
        }
    }

    fn key_up_event(&mut self, _keycode: Keycode, _keymod: Mod, _repeat: bool) {
        if let Some(state) = self.states.last_mut() {
            state.key_up_event(_keycode, _keymod, _repeat);
        }
    }

    fn controller_button_down_event(&mut self, _btn: Button, _instance_id: i32, assets: &Assets) {
        if let Some(state) = self.states.last_mut() {
            state.controller_button_down_event(_btn, _instance_id, assets);
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
