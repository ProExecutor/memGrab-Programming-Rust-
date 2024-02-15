use glfw::{Action, Key};

pub struct Button {
    pressed: bool,
    held: bool,
    released: bool,
    new_state: Option<bool>,
    typed: bool,
    typed_time: f32,
}

impl Button {
    pub fn merge(&mut self, b: &Button) {
        self.pressed |= b.pressed;
        self.released |= b.released;
        self.held |= b.held;
    }
}

pub struct ControllerButtons {
    buttons: [Button; glfw::ffi::GLFW_GAMEPAD_BUTTON_LAST + 1],
    lt: f32,
    rt: f32,
    l_stick: (f32, f32),
    r_stick: (f32, f32),
}

impl ControllerButtons {
    pub fn set_all_to_zero(&mut self) {
        for button in self.buttons.iter_mut() {
            button.reset_to_zero();
        }

        self.lt = 0.0;
        self.rt = 0.0;

        self.l_stick = (0.0, 0.0);
        self.r_stick = (0.0, 0.0);
    }
}

pub fn is_key_held(key: Key) -> bool {
    // Implementation depends on your specific context
    unimplemented!()
}

pub fn is_key_pressed_on(key: Key) -> bool {
    // Implementation depends on your specific context
    unimplemented!()
}

pub fn is_key_released(key: Key) -> bool {
    // Implementation depends on your specific context
    unimplemented!()
}

pub fn is_key_typed(key: Key) -> bool {
    // Implementation depends on your specific context
    unimplemented!()
}

pub fn is_l_mouse_pressed() -> bool {
    // Implementation depends on your specific context
    unimplemented!()
}

pub fn is_r_mouse_pressed() -> bool {
    // Implementation depends on your specific context
    unimplemented!()
}

pub fn is_l_mouse_released() -> bool {
    // Implementation depends on your specific context
    unimplemented!()
}

pub fn is_r_mouse_released() -> bool {
    // Implementation depends on your specific context
    unimplemented!()
}

pub fn is_l_mouse_held() -> bool {
    // Implementation depends on your specific context
    unimplemented!()
}

pub fn is_r_mouse_held() -> bool {
    // Implementation depends on your specific context
    unimplemented!()
}

pub fn get_controller_buttons() -> ControllerButtons {
    // Implementation depends on your specific context
    unimplemented!()
}

impl Button {
    fn reset_to_zero(&mut self) {
        self.pressed = false;
        self.held = false;
        self.released = false;
        self.new_state = None;
        self.typed = false;
        self.typed_time = 0.0;
    }

    fn update(&mut self, delta_time: f32) {
        match self.new_state {
            Some(true) => {
                if self.held {
                    self.pressed = false;
                } else {
                    self.pressed = true;
                }

                self.held = true;
                self.released = false;
            }
            Some(false) => {
                self.held = false;
                self.pressed = false;
                self.released = true;
            }
            None => {
                self.pressed = false;
                self.released = false;
            }
        }

        if self.pressed {
            self.typed = true;
            self.typed_time = 0.48;
        } else if self.held {
            self.typed_time -= delta_time;

            if self.typed_time < 0.0 {
                self.typed_time += 0.07;
                self.typed = true;
            } else {
                self.typed = false;
            }
        } else {
            self.typed_time = 0.0;
            self.typed = false;
        }

        self.new_state = None;
    }
}

pub fn update_all_buttons(delta_time: f32) {
    // Implementation depends on your specific context
    unimplemented!()
}

pub fn reset_inputs_to_zero() {
    // Implementation depends on your specific context
    unimplemented!()
}
