pub mod input;
mod physics;
mod render;
mod resource;

use ash::Device;
use input::InputSystem;

pub use physics::PhysicsSystem;
pub use render::RenderSystem;
pub use resource::ResourceSystem;

pub struct SystemManager {
    pub render_system: RenderSystem,
    pub resource_system: ResourceSystem,
    pub input_system: InputSystem,
}

impl SystemManager {
    pub fn create(font_path: &str) -> Self {
        Self {
            render_system: RenderSystem::create(),
            resource_system: ResourceSystem::create(font_path),
            input_system: InputSystem::new(),
        }
    }

    pub fn destroy(&self, device: &Device) {
        self.render_system.destroy(device);
        self.resource_system.destroy(device);
    }
}
