use glam::{Mat4, Vec3};

pub struct PositionComponent {
    pub xyz: Vec3,
    pub scale: Vec3,
}

impl PositionComponent {
    pub fn to_model_matrix(&self) -> Mat4 {
        Mat4::from_translation(self.xyz) * Mat4::from_scale(self.scale)
    }
}
