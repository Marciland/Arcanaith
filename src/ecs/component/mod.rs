mod position;
mod visual;
use crate::ecs::entity::Entity;
pub use position::PositionComponent;
use std::collections::HashMap;
pub use visual::{Layer, VisualComponent};

pub struct ComponentStorage<T> {
    components: HashMap<Entity, T>,
}

impl<T> ComponentStorage<T> {
    fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    pub fn add(&mut self, entity: Entity, component: T) {
        self.components.insert(entity, component);
    }

    /*
    fn remove(&mut self, entity: Entity) {
        self.components.remove(&entity);
    }
    */

    pub fn get(&self, entity: Entity) -> Option<&T> {
        self.components.get(&entity)
    }

    pub fn size(&self) -> usize {
        self.components.len()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Entity, &mut T)> {
        self.components
            .iter_mut()
            .map(|(entity, component)| (*entity, component))
    }
}

pub struct ComponentManager {
    pub visual_storage: ComponentStorage<VisualComponent>,
    pub position_storage: ComponentStorage<PositionComponent>,
}

impl ComponentManager {
    pub fn new() -> Self {
        Self {
            visual_storage: ComponentStorage::new(),
            position_storage: ComponentStorage::new(),
        }
    }
}
