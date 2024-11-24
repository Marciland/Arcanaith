use crate::window::Window;
use ash::Device;
use component::ComponentManager;
use entity::{Entity, EntityManager};
use system::SystemManager;

pub mod component;
pub mod entity;
pub mod system;

pub struct ECS {
    pub entity_manager: EntityManager,
    pub component_manager: ComponentManager,
    pub system_manager: SystemManager,
}

impl ECS {
    #[must_use]
    pub fn create() -> Self {
        Self {
            entity_manager: EntityManager::new(),
            component_manager: ComponentManager::new(),
            system_manager: SystemManager::create(),
        }
    }

    pub fn initialize(&mut self, window: &Window) {
        self.system_manager.render.initialize(window);
        self.system_manager.resource.initialize(window);
    }

    pub fn destroy_entity(&mut self, entity: Entity, device: &Device) {
        self.component_manager.clear_entity(entity, device);
        self.entity_manager.destroy_entity(entity);
    }

    pub fn destroy(&mut self, device: &Device) {
        self.component_manager.text_storage.destroy(device);
        self.system_manager.destroy(device);
    }
}
