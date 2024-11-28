mod component;
mod entity;
mod system;

use component::ComponentManager;
use entity::{Entity, EntityManager};
use system::SystemManager;

use ash::vk::Device;
use winit::event_loop::EventLoopProxy;

struct ECS {
    entity_manager: EntityManager,
    component_manager: ComponentManager,
    system_manager: SystemManager,
}

impl ECS {
    fn create(font_path: &str) -> Self {
        Self {
            entity_manager: EntityManager::default(),
            component_manager: ComponentManager::create(),
            system_manager: SystemManager::create(font_path),
        }
    }

    fn initialize(&mut self, window: &Window /*  TODO render context */) {
        self.system_manager.initialize(window);
    }

    fn process_inputs<T: InputHandler>(
        &mut self,
        handler: &T,
        event_proxy: &EventLoopProxy<GameEvent>, // TODO event type
    ) {
        self.system_manager
            .process_inputs(handler, &mut self.component_manager, event_proxy);
    }

    fn destroy_entity(&mut self, entity: Entity, device: &Device) {
        self.component_manager.clear_entity(entity, device);
        self.entity_manager.destroy_entity(entity);
    }

    fn destroy(&mut self, device: &Device) {
        self.component_manager.destroy(device);
        self.system_manager.destroy(device);
    }
}
