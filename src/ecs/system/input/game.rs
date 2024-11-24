use crate::{
    ecs::{component::ComponentManager, system::ResourceSystem},
    scenes, MouseEvent,
};
use indexmap::IndexSet;
use std::collections::HashSet;
use winit::keyboard::Key;

pub fn handle_player_events(
    game: &scenes::Game,
    keyboard_pressed_inputs: &IndexSet<Key>,
    _active_keyboard_inputs: &HashSet<Key>,
    mouse_inputs: &[MouseEvent],
    _component_manager: &mut ComponentManager,
    _resource_system: &ResourceSystem,
) {
    // TODO player movement
    let _player = game.get_player().expect("Missing player entity!");

    // TODO skills
    for _key in keyboard_pressed_inputs {}

    // TODO skills / movement?
    for _event in mouse_inputs {}
}

pub fn handle_mouse_events(mouse_inputs: &[MouseEvent]) {
    for _event in mouse_inputs {
        // TODO mouse event in game
    }
}

pub fn handle_key_events(keyboard_pressed_inputs: &IndexSet<Key>) {
    for _key in keyboard_pressed_inputs {
        // TODO key event in game
    }
}
