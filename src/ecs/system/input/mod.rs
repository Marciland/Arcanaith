mod menu;
pub mod mouse;
use crate::{
    ecs::{
        component::{composition::InputWithPosition, ComponentManager},
        entity::Entity,
    },
    game::{GameEvent, GameState},
};
use ash::vk::Extent2D;
use glam::Vec2;
use indexmap::IndexSet;
use mouse::{MouseButton, MouseEvent, MousePosition};
use std::collections::HashMap;
use winit::{
    dpi::PhysicalPosition,
    event::{DeviceId, ElementState},
    event_loop::EventLoopProxy,
    keyboard::Key,
};

pub struct InputSystem {
    cursor_positions: HashMap<DeviceId, Vec2>,
    // set -> only 1 key per frame
    keyboard_pressed_inputs: IndexSet<Key>,
    keyboard_released_inputs: IndexSet<Key>,
    // don't clear, keeps track over frames
    partial_mouse_inputs: HashMap<MouseButton, MousePosition>,
    mouse_inputs: Vec<MouseEvent>,
}

impl InputSystem {
    pub fn new() -> Self {
        Self {
            cursor_positions: HashMap::with_capacity(2),
            keyboard_pressed_inputs: IndexSet::with_capacity(10),
            keyboard_released_inputs: IndexSet::with_capacity(10),
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

    pub fn add_keyboard_input(&mut self, state: ElementState, key: Key) {
        match state {
            ElementState::Pressed => {
                self.keyboard_pressed_inputs.insert(key);
            }
            ElementState::Released => {
                self.keyboard_released_inputs.insert(key);
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
        event_proxy: &EventLoopProxy<GameEvent>,
    ) {
        // start from the front as pop removes the last entry
        self.keyboard_pressed_inputs.reverse();
        self.keyboard_released_inputs.reverse();
        self.mouse_inputs.reverse();

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
                todo!("event in game");
            }
        }

        // clear each frame
        self.mouse_inputs.clear();
        self.keyboard_pressed_inputs.clear();
        self.keyboard_released_inputs.clear();
    }
}
