pub mod play_state;

use std::time::Duration;

use sdl2::mouse;

use ggez::{Context, GameResult};
use ggez::event::{EventHandler, Keycode, Mod, Button, Axis};

/// A state manager
pub struct StateManager {
    running: bool,
    states: Vec<Box<EventHandler>>
}

impl StateManager {
    pub fn new<S>(state: S) -> Self where S: EventHandler + 'static {
        Self {
            running: true,
            states: vec![Box::new(state)], // create empty state stack
        }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }
}

impl EventHandler for StateManager {
    fn update(&mut self, ctx: &mut Context, dt: Duration) -> GameResult<()> {
        match self.states.last_mut() {
            Some(state) => state.update(ctx, dt),
            None => Ok(()),
        }
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        match self.states.last_mut() {
            Some(state) => state.draw(ctx),
            None => Ok(()),
        }
    }
    fn mouse_button_down_event(&mut self, _button: mouse::MouseButton, _x: i32, _y: i32) {
        match self.states.last_mut() {
            Some(state) => state.mouse_button_down_event(_button, _x, _y),
            None => (),
        }
    }

    fn mouse_button_up_event(&mut self, _button: mouse::MouseButton, _x: i32, _y: i32) {
        match self.states.last_mut() {
            Some(state) => state.mouse_button_up_event(_button, _x, _y),
            None => (),
        }
    }

    fn mouse_motion_event(&mut self,
                          _state: mouse::MouseState,
                          _x: i32,
                          _y: i32,
                          _xrel: i32,
                          _yrel: i32) {
        match self.states.last_mut() {
            Some(state) => state.mouse_motion_event(_state, _x, _y, _xrel, _yrel),
            None => (),
        }

    }

    fn mouse_wheel_event(&mut self, _x: i32, _y: i32) {
        match self.states.last_mut() {
            Some(state) => state.mouse_wheel_event(_x, _y),
            None => (),
        }
    }

    fn key_down_event(&mut self, _keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match self.states.last_mut() {
            Some(state) => state.key_down_event(_keycode, _keymod, _repeat),
            None => (),
        }
    }

    fn key_up_event(&mut self, _keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match self.states.last_mut() {
            Some(state) => state.key_up_event(_keycode, _keymod, _repeat),
            None => (),
        }
    }

    fn controller_button_down_event(&mut self, _btn: Button, _instance_id: i32) {
        match self.states.last_mut() {
            Some(state) => state.controller_button_down_event(_btn, _instance_id),
            None => (),
        }
    }
    fn controller_button_up_event(&mut self, _btn: Button, _instance_id: i32) {
        match self.states.last_mut() {
            Some(state) => state.controller_button_up_event(_btn, _instance_id),
            None => (),
        }
    }
    fn controller_axis_event(&mut self, _axis: Axis, _value: i16, _instance_id: i32) {
        match self.states.last_mut() {
            Some(state) => state.controller_axis_event(_axis, _value, _instance_id),
            None => (),
        }
    }

    fn focus_event(&mut self, _gained: bool) {
        match self.states.last_mut() {
            Some(state) => state.focus_event(_gained),
            None => (),
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
