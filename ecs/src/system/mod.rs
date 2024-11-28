mod input;
mod physics;
mod render;
mod resource;

use ash::Device;

use input::InputSystem;
use render::RenderSystem;
use resource::ResourceSystem;
use winit::event_loop::EventLoopProxy;

use crate::component::ComponentManager;

pub(crate) struct SystemManager {
    render_system: RenderSystem,
    resource_system: ResourceSystem,
    input_system: InputSystem,
}

impl SystemManager {
    pub fn create(font_path: &str) -> Self {
        Self {
            render_system: RenderSystem::create(),
            resource_system: ResourceSystem::create(font_path),
            input_system: InputSystem::new(),
        }
    }

    pub fn initialize(&mut self, window: &Window) {
        self.render_system.initialize(window);
        self.resource_system.initialize(window);
    }

    pub fn process_inputs<T: InputHandler>(
        &self,
        handler: &T,
        component_manager: &mut ComponentManager,
        event_proxy: &EventLoopProxy<GameEvent>, // TODO event type
    ) {
        self.input_system
            .process_inputs(handler, component_manager, event_proxy);
    }

    pub fn destroy(&self, device: &Device) {
        self.render_system.destroy(device);
        self.resource_system.destroy(device);
    }
}
