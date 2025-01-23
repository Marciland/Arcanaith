mod input;
mod render;
mod resource;

use rendering::Renderer;
use resource::ResourceSystem;

pub use input::{InputHandler, InputSystem, MouseEvent, MouseHandler, MousePosition};
pub use render::RenderSystem;

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
        R: Renderer,
    {
        self.resource_system.initialize(renderer);
    }

    pub fn destroy(&self) {
        self.resource_system.destroy();
    }
}
