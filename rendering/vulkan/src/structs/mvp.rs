use glam::Mat4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MVP {
    pub model: Mat4,
    pub view: Mat4,
    pub projection: Mat4,
}
