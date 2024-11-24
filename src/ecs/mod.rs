use crate::{scenes::Scene, window::Window, GameEvent};
use ash::Device;
use component::ComponentManager;
use entity::{Entity, EntityManager};
use std::time::Duration;
use system::SystemManager;
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

    pub fn process_inputs(
        &mut self,
        current_scene: &Scene,
        event_proxy: &EventLoopProxy<GameEvent>,
    ) {
        self.system_manager.input.process_inputs(
            current_scene,
            &mut self.component_manager,
            &self.system_manager.resource,
            event_proxy,
        );
    }

    pub fn render(&mut self, current_scene: &Scene, window: &mut Window) -> Duration {
        self.system_manager.render.draw(
            current_scene,
            &mut self.component_manager,
            &self.system_manager.resource,
            window,
        )
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
