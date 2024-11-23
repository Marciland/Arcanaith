mod game;
mod menu;

use crate::{
    ecs::{
        component::{composition::InputWithPosition, ComponentManager},
        entity::Entity,
        system::ResourceSystem,
    },
    game::{GameEvent, GameState},
};
use ash::vk::Extent2D;
use glam::Vec2;
use indexmap::IndexSet;
use mouse::{MouseButton, MouseEvent, MousePosition};
use std::collections::{HashMap, HashSet};
use winit::{
    dpi::PhysicalPosition,
    event::{DeviceId, ElementState},
    event_loop::EventLoopProxy,
    keyboard::Key,
};

pub mod mouse;

pub struct InputSystem {
    cursor_positions: HashMap<DeviceId, Vec2>,
    // set -> only once per key per frame
    keyboard_pressed_inputs: IndexSet<Key>,
    active_keyboard_inputs: HashSet<Key>,
    // don't clear, keeps track over frames
    partial_mouse_inputs: HashMap<MouseButton, MousePosition>,
    mouse_inputs: Vec<MouseEvent>,
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
            ElementState::Pressed => mouse::handle_pressed(
                &mut self.partial_mouse_inputs,
                &self.cursor_positions,
                mouse_button,
                device_id,
            ),

            ElementState::Released => mouse::handle_released(
                &mut self.partial_mouse_inputs,
                &mut self.mouse_inputs,
                &self.cursor_positions,
                mouse_button,
                device_id,
            ),
        }
    }

    pub fn process_inputs(
        &mut self,
        game_state: &GameState,
        component_manager: &mut ComponentManager,
        resource_system: &ResourceSystem,
        event_proxy: &EventLoopProxy<GameEvent>,
    ) {
        // get all positions of components that have both an input and a position
        let mut components: Vec<InputWithPosition> =
            Vec::with_capacity(component_manager.input_storage.size());
        let mut active_entity: Option<Entity> = None;
        for (entity, input) in component_manager.input_storage.iter_mut() {
            if input.is_active {
                active_entity = Some(entity);
            }
            if let Some(position) = component_manager.position_storage.get(entity) {
                components.push(InputWithPosition { input, position });
            }
        }

        // handling events
        match game_state {
            GameState::MainMenu | GameState::Settings | GameState::_Pause => {
                menu::handle_mouse_events(&self.mouse_inputs, &components, event_proxy);
                menu::handle_key_events(
                    &self.keyboard_pressed_inputs,
                    active_entity.expect("At least one active entity expected!"),
                    component_manager,
                    event_proxy,
                );
            }
            GameState::Game => {
                game::handle_player_events(
                    &self.keyboard_pressed_inputs,
                    &self.active_keyboard_inputs,
                    &self.mouse_inputs,
                    component_manager,
                    resource_system,
                );
                game::handle_mouse_events(&self.mouse_inputs);
                game::handle_key_events(&self.keyboard_pressed_inputs);
            }
        }

        // clear each frame
        self.mouse_inputs.clear();
        self.keyboard_pressed_inputs.clear();
    }
}
