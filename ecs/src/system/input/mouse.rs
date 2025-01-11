use super::super::super::{
    component::{ComponentManager, PositionComponent, Quad},
    entity::Entity,
    system::InputSystem,
};

use glam::{Vec2, Vec3, Vec3Swizzles};
use winit::event::DeviceId;

#[derive(Eq, Hash, PartialEq)]
pub struct MouseButton {
    pub mouse_button: winit::event::MouseButton,
    device_id: DeviceId,
}

pub struct MousePosition {
    pressed: Vec2,
    released: Option<Vec2>,
}

pub struct MouseEvent {
    pub button: MouseButton,
    pub position: MousePosition,
}

pub trait MouseHandler {
    fn handle_pressed(&mut self, mouse_button: winit::event::MouseButton, device_id: DeviceId);

    fn handle_released(&mut self, mouse_button: winit::event::MouseButton, device_id: DeviceId);

    fn entity_was_clicked<E>(
        &self,
        component_manager: &ComponentManager<E>,
        position: &MousePosition,
        entity: Entity,
    ) -> bool;
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

    fn entity_was_clicked<E>(
        &self,
        component_manager: &ComponentManager<E>,
        position: &MousePosition,
        entity: Entity,
    ) -> bool {
        let Some(component_position) = component_manager.position_storage.get(entity) else {
            return false;
        };

        component_was_clicked(component_position, position)
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
    let model_matrix = position.get_model_matrix();
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
