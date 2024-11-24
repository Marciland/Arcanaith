pub mod input;
mod render;
mod resource;

use ash::Device;
use input::InputSystem;

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

    pub fn destroy(&self, device: &Device) {
        self.render.destroy(device);
        self.resource.destroy(device);
    }
}
