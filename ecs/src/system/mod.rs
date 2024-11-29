mod input;
mod render;
mod resource;

use ash::Device;

pub use input::InputHandler;
use input::InputSystem;
pub use render::RenderContext;
use render::RenderSystem;
use resource::ResourceSystem;

pub(crate) struct SystemManager {
    pub render_system: RenderSystem,
    pub resource_system: ResourceSystem,
    pub input_system: InputSystem,
}

impl SystemManager {
    pub fn create(texture_path: &str, font_path: &str) -> Self {
        Self {
            render_system: RenderSystem::create(),
            resource_system: ResourceSystem::create(texture_path, font_path),
            input_system: InputSystem::default(),
        }
    }

    pub fn initialize<R>(&mut self, renderer: &R)
    where
        R: RenderContext,
    {
        self.render_system.initialize(renderer);
        self.resource_system.initialize(renderer);
    }

    pub fn destroy(&self, device: &Device) {
        self.render_system.destroy(device);
        self.resource_system.destroy(device);
    }
}
