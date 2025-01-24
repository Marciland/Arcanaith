mod mouse;

use glam::Vec2;
use indexmap::IndexSet;
use mouse::MouseButton;
use rendering::WindowSize;
use std::collections::{HashMap, HashSet};
use winit::{
    dpi::PhysicalPosition,
    event::{DeviceId, ElementState},
    event_loop::EventLoopProxy,
    keyboard::Key,
};

pub use mouse::{MouseEvent, MouseHandler, MousePosition};

use super::super::{component::ComponentManager, entity::Entity, ECS};

pub struct InputSystem {
    cursor_positions: HashMap<DeviceId, Vec2>,
    // set -> only once per key per frame
    pub keyboard_pressed_inputs: IndexSet<Key>,
    active_keyboard_inputs: HashSet<Key>,
    // don't clear, keeps track over frames
    partial_mouse_inputs: HashMap<MouseButton, MousePosition>,
    pub mouse_inputs: Vec<MouseEvent>,
}

impl Default for InputSystem {
    fn default() -> Self {
        Self {
            cursor_positions: HashMap::with_capacity(2),
            keyboard_pressed_inputs: IndexSet::with_capacity(10),
            active_keyboard_inputs: HashSet::with_capacity(5),
            partial_mouse_inputs: HashMap::with_capacity(10),
            mouse_inputs: Vec::with_capacity(10),
        }
    }
}

impl InputSystem {
    pub fn update_cursor_position(
        &mut self,
        id: DeviceId,
        position: PhysicalPosition<f64>,
        window_size: &WindowSize,
    ) {
        let normalized_position = Vec2 {
            x: (position.x / f64::from(window_size.width)) as f32 * 2.0 - 1.0,
            y: (position.y / f64::from(window_size.height)) as f32 * 2.0 - 1.0,
        };
        self.cursor_positions.insert(id, normalized_position);
    }

    pub fn update_keyboard_input(&mut self, state: ElementState, key: Key) {
        match state {
            ElementState::Pressed => {
                self.keyboard_pressed_inputs.insert(key.clone());
                self.active_keyboard_inputs.insert(key);
            }
            ElementState::Released => {
                self.active_keyboard_inputs.remove(&key);
            }
        }
    }

    pub fn add_mouse_input(
        &mut self,
        device_id: DeviceId,
        mouse_button: winit::event::MouseButton,
        state: ElementState,
    ) {
        match state {
            ElementState::Pressed => self.handle_pressed(mouse_button, device_id),

            ElementState::Released => self.handle_released(mouse_button, device_id),
        }
    }

    pub fn set_next_entity_to_active<E>(
        component_manager: &mut ComponentManager<E>,
        currently_active: Entity,
    ) {
        let Some(active_input) = component_manager.input_storage.get_mut(currently_active) else {
            return;
        };

        let Some(next_entity) = active_input.next else {
            return;
        };

        active_input.is_active = false;

        let Some(next_input) = component_manager.input_storage.get_mut(next_entity) else {
            return;
        };

        next_input.is_active = true;
    }

    pub fn set_previous_entity_to_active<E>(
        component_manager: &mut ComponentManager<E>,
        currently_active: Entity,
    ) {
        let Some(active_input) = component_manager.input_storage.get_mut(currently_active) else {
            return;
        };

        let Some(previous_entity) = active_input.previous else {
            return;
        };

        active_input.is_active = false;

        let Some(previous_input) = component_manager.input_storage.get_mut(previous_entity) else {
            return;
        };

        previous_input.is_active = true;
    }
}

pub trait InputHandler<E> {
    fn handle_mouse_events(
        &self,
        ecs: &ECS<E>,
        events: &[MouseEvent],
        event_proxy: &EventLoopProxy<E>,
    );
    fn handle_key_events(
        &self,
        ecs: &mut ECS<E>,
        pressed_keys: &IndexSet<Key>,
        event_proxy: &EventLoopProxy<E>,
    );
}
