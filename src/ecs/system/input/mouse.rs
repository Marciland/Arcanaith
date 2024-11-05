use crate::{ecs::component::composition::InputWithPosition, GameEvent};
use glam::Vec2;
use std::collections::HashMap;
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

pub fn handle_pressed(
    partial_mouse_inputs: &mut HashMap<MouseButton, MousePosition>,
    cursor_positions: &HashMap<DeviceId, Vec2>,
    mouse_button: winit::event::MouseButton,
    device_id: DeviceId,
) {
    partial_mouse_inputs.insert(
        MouseButton {
            mouse_button,
            device_id,
        },
        MousePosition {
            pressed: *cursor_positions
                .get(&device_id)
                .expect("Mouse event for unknown device!"),
            released: None,
        },
    );
}

pub fn handle_released(
    partial_mouse_inputs: &mut HashMap<MouseButton, MousePosition>,
    mouse_inputs: &mut Vec<MouseEvent>,
    cursor_positions: &HashMap<DeviceId, Vec2>,
    mouse_button: winit::event::MouseButton,
    device_id: DeviceId,
) {
    if let Some(position) = partial_mouse_inputs.remove(&MouseButton {
        mouse_button,
        device_id,
    }) {
        let release_position = *cursor_positions
            .get(&device_id)
            .expect("Mouse event for unknown device!");

        mouse_inputs.push(MouseEvent {
            button: MouseButton {
                mouse_button,
                device_id,
            },
            position: MousePosition {
                pressed: position.pressed,
                released: Some(release_position),
            },
        });
    }
}

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
