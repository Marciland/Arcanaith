use crate::ecs::{component::ComponentManager, system::ResourceSystem};
use ash::Device;
use std::collections::HashSet;

pub type Entity = u32;

pub struct EntityManager {
    next_id: Entity,
    entities: HashSet<Entity>,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            entities: HashSet::new(),
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        let entity = self.next_id;
        self.entities.insert(entity);
        self.next_id += 1;
        entity
    }

    pub fn destroy_entity(&mut self, entity: Entity) {
        self.entities.remove(&entity);
    }

    pub fn clear(&mut self, component_manager: &mut ComponentManager, device: &Device) {
        for entity in &self.entities {
            component_manager.clear_entity(*entity, device);
        }
        self.entities.clear();
    }
}

pub struct EntityLoader<'loading> {
    pub component_manager: &'loading mut ComponentManager,
    pub resource_system: &'loading ResourceSystem,
}
