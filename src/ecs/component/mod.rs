mod input;
mod physics;
mod position;
mod text;
mod visual;

use super::Entity;
use ash::Device;
use std::collections::HashMap;

pub mod composition;

pub use input::InputComponent;
pub use physics::PhysicsComponent;
pub use position::PositionComponent;
pub use text::TextComponent;
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

    fn remove(&mut self, entity: Entity) {
        self.components.remove(&entity);
    }

    pub fn get(&self, entity: Entity) -> Option<&T> {
        self.components.get(&entity)
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        self.components.get_mut(&entity)
    }

    pub fn size(&self) -> usize {
        self.components.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = (Entity, &T)> {
        self.components
            .iter()
            .map(|(entity, component)| (*entity, component))
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
    pub input_storage: ComponentStorage<InputComponent>,
    pub text_storage: ComponentStorage<TextComponent>,
    pub physics_storage: ComponentStorage<PhysicsComponent>,
}

impl Default for ComponentManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ComponentManager {
    pub fn new() -> Self {
        Self {
            visual_storage: ComponentStorage::new(),
            position_storage: ComponentStorage::new(),
            input_storage: ComponentStorage::new(),
            text_storage: ComponentStorage::new(),
            physics_storage: ComponentStorage::new(),
        }
    }

    pub fn clear_entity(&mut self, entity: Entity, device: &Device) {
        self.visual_storage.remove(entity);
        self.position_storage.remove(entity);
        self.input_storage.remove(entity);
        self.physics_storage.remove(entity);

        self.text_storage.destroy_entity(entity, device);
    }
}
