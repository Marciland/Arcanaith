use crate::game::GameState;
use std::collections::HashMap;
use winit::{
    dpi::PhysicalPosition,
    event::{DeviceId, ElementState, KeyEvent, MouseButton},
    keyboard::Key::Character,
};

struct MouseEvent {
    _button: MouseButton,
    _state: ElementState,
}

pub struct InputSystem {
    cursor_positions: HashMap<DeviceId, PhysicalPosition<f64>>,
    keyboard_inputs: Vec<KeyEvent>,
    mouse_inputs: Vec<MouseEvent>,
}

impl InputSystem {
    pub fn new() -> Self {
        Self {
            cursor_positions: HashMap::with_capacity(10),
            keyboard_inputs: Vec::with_capacity(10),
            mouse_inputs: Vec::with_capacity(10),
        }
    }

    pub fn update_cursor_position(&mut self, id: DeviceId, position: PhysicalPosition<f64>) {
        self.cursor_positions.insert(id, position);
    }

    pub fn add_keyboard_input(&mut self, event: KeyEvent) {
        self.keyboard_inputs.push(event);
    }

    pub fn add_mouse_input(&mut self, button: MouseButton, state: ElementState) {
        self.mouse_inputs.push(MouseEvent {
            _button: button,
            _state: state,
        });
    }

    pub fn process_inputs(&mut self, _game_state: &GameState) {
        self.keyboard_inputs.reverse();
        self.mouse_inputs.reverse();

        for _ in 0..self.keyboard_inputs.len() {
            if let Some(event) = self.keyboard_inputs.pop() {
                if let Character(_key) = event.logical_key {
                    // TODO handle keyboard input based on game_state
                }
            }
        }

        for _ in 0..self.mouse_inputs.len() {
            if let Some(_event) = self.mouse_inputs.pop() {
                // TODO handle mouse input based on game_state
            }
        }
    }
}
