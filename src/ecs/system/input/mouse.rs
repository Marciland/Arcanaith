use crate::{ecs::component::composition::InputWithPosition, GameEvent};
use glam::Vec2;
use std::collections::HashMap;
use std::hash::BuildHasher;
use winit::{event::DeviceId, event_loop::EventLoopProxy};

#[derive(Eq, Hash, PartialEq)]
pub struct MouseButton {
    pub mouse_button: winit::event::MouseButton,
    pub device_id: DeviceId,
}

pub struct MousePosition {
    pub pressed: Vec2,
    pub released: Option<Vec2>,
}

pub struct MouseEvent {
    pub button: MouseButton,
    pub position: MousePosition,
}

pub fn handle_pressed<Hasher>(
    partial_mouse_inputs: &mut HashMap<MouseButton, MousePosition, Hasher>,
    cursor_positions: &HashMap<DeviceId, Vec2, Hasher>,
    mouse_button: winit::event::MouseButton,
    device_id: DeviceId,
) where
    Hasher: BuildHasher,
{
    let Some(pressed) = cursor_positions.get(&device_id) else {
        return;
    };

    partial_mouse_inputs.insert(
        MouseButton {
            mouse_button,
            device_id,
        },
        MousePosition {
            pressed: *pressed,
            released: None,
        },
    );
}

pub fn handle_released<Hasher>(
    partial_mouse_inputs: &mut HashMap<MouseButton, MousePosition, Hasher>,
    mouse_inputs: &mut Vec<MouseEvent>,
    cursor_positions: &HashMap<DeviceId, Vec2, Hasher>,
    mouse_button: winit::event::MouseButton,
    device_id: DeviceId,
) where
    Hasher: BuildHasher,
{
    let Some(position) = partial_mouse_inputs.remove(&MouseButton {
        mouse_button,
        device_id,
    }) else {
        return;
    };

    let Some(released) = cursor_positions.get(&device_id) else {
        return;
    };

    mouse_inputs.push(MouseEvent {
        button: MouseButton {
            mouse_button,
            device_id,
        },
        position: MousePosition {
            pressed: position.pressed,
            released: Some(*released),
        },
    });
}

#[must_use]
pub fn any_component_was_clicked(
    components: &[InputWithPosition],
    mouse_position: &MousePosition,
) -> Option<fn(event_proxy: &EventLoopProxy<GameEvent>) -> ()> {
    for component in components {
        if component.position.was_clicked(mouse_position) {
            return Some(component.input.activate);
        }
    }

    None
}
