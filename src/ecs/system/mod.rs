pub mod input;
mod position;
mod render;
mod resource;

use ash::Device;
use input::InputSystem;

pub use position::PositionSystem;
pub use render::RenderSystem;
pub use resource::ResourceSystem;

pub struct SystemManager {
    pub render: RenderSystem,
    pub resource: ResourceSystem,
    pub input: InputSystem,
    pub position: PositionSystem,
}

impl SystemManager {
    pub fn create() -> Self {
        Self {
            render: RenderSystem::create(),
            resource: ResourceSystem::create(),
            input: InputSystem::new(),
            position: PositionSystem::new(),
        }
    }

    pub fn destroy(&self, device: &Device) {
        self.render.destroy(device);
        self.resource.destroy(device);
    }
}
