mod input;
mod render;
mod resource;

use ash::Device;
use resource::ResourceSystem;

pub use input::{InputHandler, InputSystem, MouseEvent, MouseHandler, MousePosition};
pub use render::{RenderContext, RenderSystem};

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
