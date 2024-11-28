use crate::ecs::component::PositionComponent;
use glam::{Mat4, Vec2, Vec3};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ModelViewProjection {
    pub model: Mat4,
    pub view: Mat4,
    pub projection: Mat4,
}

impl ModelViewProjection {
    pub fn _scale(mut self, scaling: Vec2) -> Self {
        //TODO use this
        self.model *= Mat4::from_scale(Vec3 {
            x: scaling.x,
            y: scaling.y,
            z: 1.0,
        });
        self
    }

    pub fn _translate(mut self, translation: Vec2) -> Self {
        //TODO use this
        self.model *= Mat4::from_translation(Vec3 {
            x: translation.x,
            y: translation.y,
            z: 0.0,
        });
        self
    }

    pub fn get_projection() -> Mat4 {
        Mat4::orthographic_rh(-1.0, 1.0, -1.0, 1.0, 0.0, -1.0)
    }

    pub fn get_model_matrix(position: &PositionComponent) -> Mat4 {
        Mat4::from_translation(position.xyz) * Mat4::from_scale(position.scale)
    }
}
