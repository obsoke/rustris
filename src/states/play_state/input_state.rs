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
