use glam::Mat4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ModelViewProjection {
    pub model: Mat4,
    pub view: Mat4,
    pub projection: Mat4,
}

impl ModelViewProjection {
    /*
    pub fn scale(mut self, scaling: Vec2) -> Self {
        self.model *= Mat4::from_scale(Vec3 {
            x: scaling.x,
            y: scaling.y,
            z: 1.0,
        });
        self
    }

    pub fn translate(mut self, translation: Vec2) -> Self {
        self.model *= Mat4::from_translation(Vec3 {
            x: translation.x,
            y: translation.y,
            z: 0.0,
        });
        self
    }
    */
}

impl Default for ModelViewProjection {
    fn default() -> Self {
        Self {
            model: Mat4::IDENTITY,
            view: Mat4::IDENTITY,
            projection: Mat4::orthographic_rh(-1.0, 1.0, -1.0, 1.0, 0.0, 1.0), // TODO is this OK?
        }
    }
}
