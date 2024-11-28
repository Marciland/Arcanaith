// TODO remove dependencies
use crate::{window::Window, GameEvent};

use ash::Device;
use component::ComponentManager;
use entity::{Entity, EntityManager};
use system::{input::InputHandler, SystemManager};
use winit::event_loop::EventLoopProxy;

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
    pub fn create(font_path: &str) -> Self {
        Self {
            entity_manager: EntityManager::new(),
            component_manager: ComponentManager::default(),
            system_manager: SystemManager::create(font_path),
        }
    }

    pub fn initialize(&mut self, window: &Window) {
        self.system_manager.render_system.initialize(window);
        self.system_manager.resource_system.initialize(window);
    }

    pub fn process_inputs<T: InputHandler>(
        &mut self,
        handler: &T,
        event_proxy: &EventLoopProxy<GameEvent>, // TODO get rid of GameEvent
    ) {
        self.system_manager.input_system.process_inputs(
            handler,
            &mut self.component_manager,
            event_proxy,
        );
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
