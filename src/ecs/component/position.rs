use crate::{mouse::MousePosition, objects::Quad};
use glam::{Mat4, Vec3, Vec3Swizzles};

pub struct PositionComponent {
    pub xyz: Vec3,
    pub scale: Vec3,
}

impl PositionComponent {
    pub fn to_model_matrix(&self) -> Mat4 {
        Mat4::from_translation(self.xyz) * Mat4::from_scale(self.scale)
    }

    pub fn was_clicked(&self, mouse_position: &MousePosition) -> bool {
        let current_geometry = self.quad_from_model();

        current_geometry.position_is_inside(mouse_position.pressed)
            && current_geometry.position_is_inside(
                mouse_position
                    .released
                    .expect("Mouse position had no release position!"),
            )
    }

    fn quad_from_model(&self) -> Quad {
        let model_matrix = self.to_model_matrix();
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
}
