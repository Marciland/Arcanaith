mod input;
mod render;
mod resource;

use ash::Device;

use input::InputSystem;
pub use input::{InputHandler, MouseEvent, MouseHandler, MousePosition};
pub use render::{RenderContext, RenderSystem};
use resource::ResourceSystem;

pub(crate) struct SystemManager {
    pub resource_system: ResourceSystem,
    pub input_system: InputSystem,
}

impl SystemManager {
    pub fn create(texture_path: &str, font_path: &str) -> Self {
        Self {
            resource_system: ResourceSystem::create(texture_path, font_path),
            input_system: InputSystem::default(),
        }
    }

    pub fn initialize<R>(&mut self, renderer: &R)
    where
        R: RenderContext,
    {
        self.resource_system.initialize(renderer);
    }

    pub fn destroy(&self, device: &Device) {
        self.resource_system.destroy(device);
    }
}
