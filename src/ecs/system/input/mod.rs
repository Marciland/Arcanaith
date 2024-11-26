mod mouse;

use crate::{ecs::component::ComponentManager, game::GameEvent, scenes::Scene};
use ash::vk::Extent2D;
use glam::Vec2;
use indexmap::IndexSet;
use mouse::{MouseButton, MousePosition};
use std::collections::{HashMap, HashSet};
use winit::{
    dpi::PhysicalPosition,
    event::{DeviceId, ElementState},
    event_loop::EventLoopProxy,
    keyboard::Key,
};

pub use mouse::{MouseEvent, MouseHandler};

pub struct InputSystem {
    cursor_positions: HashMap<DeviceId, Vec2>,
    // set -> only once per key per frame
    keyboard_pressed_inputs: IndexSet<Key>,
    active_keyboard_inputs: HashSet<Key>,
    // don't clear, keeps track over frames
    partial_mouse_inputs: HashMap<MouseButton, MousePosition>,
    mouse_inputs: Vec<MouseEvent>,
}

impl Default for InputSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl InputSystem {
    pub fn new() -> Self {
        Self {
            cursor_positions: HashMap::with_capacity(2),
            keyboard_pressed_inputs: IndexSet::with_capacity(10),
            active_keyboard_inputs: HashSet::with_capacity(5),
            partial_mouse_inputs: HashMap::with_capacity(10),
            mouse_inputs: Vec::with_capacity(10),
        }
    }

    pub fn update_cursor_position(
        &mut self,
        id: DeviceId,
        position: PhysicalPosition<f64>,
        window_size: Extent2D,
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

    pub fn process_inputs(
        &mut self,
        current_scene: &Scene,
        component_manager: &mut ComponentManager,
        event_proxy: &EventLoopProxy<GameEvent>,
    ) {
        // handling events
        if let Scene::Game(game) = current_scene {
            game.handle_player_events(
                &self.keyboard_pressed_inputs,
                &self.active_keyboard_inputs,
                &self.mouse_inputs,
                component_manager,
            );
        };

        current_scene.handle_mouse_events(&self.mouse_inputs, component_manager, event_proxy);
        current_scene.handle_key_events(
            &self.keyboard_pressed_inputs,
            component_manager,
            event_proxy,
        );

        // clear each frame
        self.mouse_inputs.clear();
        self.keyboard_pressed_inputs.clear();
    }
}

pub trait InputHandler {
    fn handle_mouse_events(
        &self,
        events: &[MouseEvent],
        component_manager: &mut ComponentManager,
        event_proxy: &EventLoopProxy<GameEvent>,
    );
    fn handle_key_events(
        &self,
        pressed_keys: &IndexSet<Key>,
        component_manager: &mut ComponentManager,
        event_proxy: &EventLoopProxy<GameEvent>,
    );
}
