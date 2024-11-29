use crate::component::{Quad, MVP};

use super::{
    super::super::component::{composition::InputWithPosition, PositionComponent},
    InputSystem,
};
use glam::{Vec2, Vec3, Vec3Swizzles};
use winit::{event::DeviceId, event_loop::EventLoopProxy};

#[derive(Eq, Hash, PartialEq)]
pub struct MouseButton {
    mouse_button: winit::event::MouseButton,
    device_id: DeviceId,
}

pub struct MousePosition {
    pressed: Vec2,
    released: Option<Vec2>,
}

pub struct MouseEvent {
    button: MouseButton,
    position: MousePosition,
}

pub trait MouseHandler {
    fn handle_pressed(&mut self, mouse_button: winit::event::MouseButton, device_id: DeviceId);

    fn handle_released(&mut self, mouse_button: winit::event::MouseButton, device_id: DeviceId);

    fn any_object_was_clicked<E>(
        objects: &[InputWithPosition<E>],
        mouse_position: &MousePosition,
    ) -> Option<fn(event_proxy: &EventLoopProxy<E>) -> ()>;
}

impl MouseHandler for InputSystem {
    fn handle_pressed(&mut self, mouse_button: winit::event::MouseButton, device_id: DeviceId) {
        let Some(pressed) = self.cursor_positions.get(&device_id) else {
            return;
        };

        self.partial_mouse_inputs.insert(
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

    fn handle_released(&mut self, mouse_button: winit::event::MouseButton, device_id: DeviceId) {
        let Some(position) = self.partial_mouse_inputs.remove(&MouseButton {
            mouse_button,
            device_id,
        }) else {
            return;
        };

        let Some(released) = self.cursor_positions.get(&device_id) else {
            return;
        };

        self.mouse_inputs.push(MouseEvent {
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
    fn any_object_was_clicked<E>(
        objects: &[InputWithPosition<E>],
        mouse_position: &MousePosition,
    ) -> Option<fn(event_proxy: &EventLoopProxy<E>) -> ()> {
        for obj in objects {
            if component_was_clicked(obj.position, mouse_position) {
                return Some(obj.input.activate);
            }
        }

        None
    }
}

fn component_was_clicked(
    component_position: &PositionComponent,
    mouse_position: &MousePosition,
) -> bool {
    let current_geometry = quad_from_model(component_position);

    let Some(release_position) = mouse_position.released else {
        return false;
    };

    current_geometry.position_is_inside(mouse_position.pressed)
        && current_geometry.position_is_inside(release_position)
}

fn quad_from_model(position: &PositionComponent) -> Quad {
    let model_matrix = MVP::get_model_matrix(position);
    let mut geometry = Quad::new();

    let bottom_left = model_matrix.transform_point3(Vec3 {
        x: geometry.bottom_left.x,
        y: geometry.bottom_left.y,
        z: 1.0,
    });

    let bottom_right = model_matrix.transform_point3(Vec3 {
        x: geometry.bottom_right.x,
        y: geometry.bottom_right.y,
        z: 1.0,
    });

    let top_left = model_matrix.transform_point3(Vec3 {
        x: geometry.top_left.x,
        y: geometry.top_left.y,
        z: 1.0,
    });

    let top_right = model_matrix.transform_point3(Vec3 {
        x: geometry.top_right.x,
        y: geometry.top_right.y,
        z: 1.0,
    });

    geometry.bottom_left = bottom_left.xy();
    geometry.bottom_right = bottom_right.xy();
    geometry.top_left = top_left.xy();
    geometry.top_right = top_right.xy();

    geometry
}
