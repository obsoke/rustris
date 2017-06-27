/// Represents an input field. An input can be active or inactive. When active.
/// there are two separate times which dictate when the action tied to that
/// input can execute. The flow of action when an input is held down goes like
/// this:
///
/// 1) Key/Button is pressed, action is performed
/// 2) The initial delay timer begins to count down.
/// 3) After the initial delay timer has expired, the action is executed once again.
/// 4) The secondary delay timer begins to count down.
/// 5) After the secondary delay timer has expired, the action is executed once again.
/// 6) The secondary timer is reset.
/// 7) GOTO Step 4
#[derive(Debug, Copy, Clone)]
pub struct InputStateField {
    pub is_active: bool,
    pub initial_delay_timer: f64,
    pub secondary_delay_timer: f64,
}

impl InputStateField {
    pub fn reset(&mut self) {
        self.is_active = false;
        self.initial_delay_timer = 0.0;
        self.secondary_delay_timer = 0.0;
    }
}

impl Default for InputStateField {
    fn default() -> Self {
        InputStateField {
            is_active: false,
            initial_delay_timer: 0.0,
            secondary_delay_timer: 0.0,
        }
    }
}

// Ideally, this would all be more general so it could be used in any state.
// Maybe something like the 'Command' pattern.
/// Represents the commands of `PlayState` mapped to input.
#[derive(Debug, Copy, Clone)]
pub struct InputState {
    pub left: InputStateField,
    pub right: InputStateField,
    pub soft_drop: InputStateField,
    pub hard_drop: InputStateField,
    pub rotate_clockwise: InputStateField,
    pub rotate_counterclockwise: InputStateField,
    pub drop: InputStateField,
    pub hold: InputStateField,
}


impl Default for InputState {
    fn default() -> Self {
        InputState {
            left: InputStateField::default(),
            right: InputStateField::default(),
            soft_drop: InputStateField::default(),
            hard_drop: InputStateField::default(),
            rotate_clockwise: InputStateField::default(),
            rotate_counterclockwise: InputStateField::default(),
            drop: InputStateField::default(),
            hold: InputStateField::default(),
        }
    }
}
