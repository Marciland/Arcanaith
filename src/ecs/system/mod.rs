use ash::Device;
mod input;
mod render;
mod resource;

pub use input::{mouse, InputSystem};
pub use render::RenderSystem;
pub use resource::ResourceSystem;

pub struct SystemManager {
    pub render: RenderSystem,
    pub resource: ResourceSystem,
    pub input: InputSystem,
}

impl SystemManager {
    pub fn create() -> Self {
        Self {
            render: RenderSystem::create(),
            resource: ResourceSystem::create(),
            input: InputSystem::new(),
        }
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn destroy(&self, device: &Device) {
        self.render.destroy(device);
        self.resource.destroy(device);
    }
}
