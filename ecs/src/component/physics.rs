use glam::Vec3;

pub(crate) struct PhysicsComponent {
    velocity: Vec3,
}

impl Default for PhysicsComponent {
    fn default() -> Self {
        Self {
            velocity: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        }
    }
}
