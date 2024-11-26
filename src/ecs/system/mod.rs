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
    pub physics_system: PhysicsSystem,
}

impl SystemManager {
    pub fn create() -> Self {
        Self {
            render_system: RenderSystem::create(),
            resource_system: ResourceSystem::create(),
            input_system: InputSystem::new(),
            physics_system: PhysicsSystem::new(),
        }
    }

    pub fn destroy(&self, device: &Device) {
        self.render_system.destroy(device);
        self.resource_system.destroy(device);
    }
}
