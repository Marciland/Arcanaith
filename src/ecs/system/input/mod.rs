mod menu;
use crate::{
    ecs::{
        component::{ComponentManager, PositionComponent},
        entity::Entity,
    },
    game::{GameEvent, GameState},
};
use std::collections::HashMap;
use winit::{
    dpi::PhysicalPosition,
    event::{DeviceId, ElementState, MouseButton},
    event_loop::EventLoopProxy,
    keyboard::Key,
};

struct MouseEvent {
    _button: MouseButton,
    _state: ElementState,
}

pub struct InputSystem {
    cursor_positions: HashMap<DeviceId, PhysicalPosition<f64>>,
    keyboard_inputs: Vec<Key>,
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

    pub fn add_keyboard_input(&mut self, key: Key) {
        self.keyboard_inputs.push(key);
    }

    pub fn add_mouse_input(&mut self, button: MouseButton, state: ElementState) {
        self.mouse_inputs.push(MouseEvent {
            _button: button,
            _state: state,
        });
    }

    pub fn process_inputs(
        &mut self,
        game_state: &GameState,
        component_manager: &mut ComponentManager,
        event_proxy: &EventLoopProxy<GameEvent>,
    ) {
        self.keyboard_inputs.reverse();
        self.mouse_inputs.reverse();
        let mut components: Vec<&PositionComponent> =
            Vec::with_capacity(component_manager.input_storage.size());

        let mut active_entity: Option<Entity> = None;
        // get all positions of components that have both an input and a position
        for (entity, input) in component_manager.input_storage.iter_mut() {
            if input.is_active {
                active_entity = Some(entity);
            }
            if let Some(position) = component_manager.position_storage.get(entity) {
                components.push(position);
            }
        }

        for _ in 0..self.mouse_inputs.len() {
            if let Some(event) = self.mouse_inputs.pop() {
                handle_mouse_event(&event, game_state, &components);
            }
        }

        for _ in 0..self.keyboard_inputs.len() {
            if let Some(key) = self.keyboard_inputs.pop() {
                handle_key_event(
                    &key,
                    game_state,
                    active_entity.expect("At least one active entity expected!"),
                    component_manager,
                    event_proxy,
                );
            }
        }
    }
}

fn handle_key_event(
    key: &Key,
    game_state: &GameState,
    active: Entity,
    component_manager: &mut ComponentManager,
    event_proxy: &EventLoopProxy<GameEvent>,
) {
    match game_state {
        GameState::MainMenu | GameState::Settings => {
            menu::handle_key_event(key, active, component_manager, event_proxy);
        }
        GameState::Game => {
            todo!("key event in game")
        }
        GameState::_Pause => {
            todo!("key event in pause")
        }
    }
}

fn handle_mouse_event(
    _event: &MouseEvent,
    game_state: &GameState,
    _components: &[&PositionComponent],
) {
    match game_state {
        GameState::MainMenu => {
            todo!("mouse event in main menu")
        }
        GameState::Game => {
            todo!("mouse event in game")
        }
        GameState::_Pause => {
            todo!("mouse event in pause")
        }
        GameState::Settings => {
            todo!("mouse event in settings")
        }
    }
}
