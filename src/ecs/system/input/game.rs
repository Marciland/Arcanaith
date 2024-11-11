use crate::ecs::{
    component::{player::PlayerState, ComponentManager},
    system::{mouse::MouseEvent, ResourceSystem},
};
use indexmap::IndexSet;
use std::collections::HashSet;
use winit::keyboard::{Key, NamedKey};

pub fn handle_player_events(
    keyboard_pressed_inputs: &IndexSet<Key>,
    active_keyboard_inputs: &HashSet<Key>,
    mouse_inputs: &[MouseEvent],
    component_manager: &mut ComponentManager,
    resource_system: &ResourceSystem,
) {
    // player movement
    let player = component_manager
        .player_entity
        .as_mut()
        .expect("Missing player entity!");

    if active_keyboard_inputs.contains(&Key::Named(NamedKey::ArrowDown))
        || active_keyboard_inputs.contains(&Key::Character("s".into()))
    {
        player.change_state(
            &mut component_manager.visual_storage,
            resource_system,
            PlayerState::WalkingDown,
        );
    } else if active_keyboard_inputs.contains(&Key::Named(NamedKey::ArrowUp))
        || active_keyboard_inputs.contains(&Key::Character("w".into()))
    {
        player.change_state(
            &mut component_manager.visual_storage,
            resource_system,
            PlayerState::WalkingUp,
        );
    } else if active_keyboard_inputs.contains(&Key::Named(NamedKey::ArrowLeft))
        || active_keyboard_inputs.contains(&Key::Character("a".into()))
    {
        player.change_state(
            &mut component_manager.visual_storage,
            resource_system,
            PlayerState::WalkingLeft,
        );
    } else if active_keyboard_inputs.contains(&Key::Named(NamedKey::ArrowRight))
        || active_keyboard_inputs.contains(&Key::Character("d".into()))
    {
        player.change_state(
            &mut component_manager.visual_storage,
            resource_system,
            PlayerState::WalkingRight,
        );
    } else {
        player.change_state(
            &mut component_manager.visual_storage,
            resource_system,
            PlayerState::Idle,
        );
    }

    // skills
    for _key in keyboard_pressed_inputs {}

    // skills / movement?
    for _event in mouse_inputs {}
}

pub fn handle_mouse_events(mouse_inputs: &[MouseEvent]) {
    for _event in mouse_inputs {
        // todo!("mouse event in game")
    }
}

pub fn handle_key_events(keyboard_pressed_inputs: &IndexSet<Key>) {
    for _key in keyboard_pressed_inputs {
        //todo!("key event in game")
    }
}
