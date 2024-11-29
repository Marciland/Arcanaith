mod input;
mod physics;
mod position;
mod text;
mod visual;

use super::entity::Entity;
use ash::Device;
use std::collections::HashMap;

pub mod composition;

use input::InputComponent;
use physics::PhysicsComponent;
pub use position::{PositionComponent, Quad, MVP};
pub use text::{TextComponent, TextContent};
pub use visual::{ImageData, Layer, Vertex, VisualComponent};

pub struct ComponentStorage<T> {
    components: HashMap<Entity, T>,
}

impl<T> ComponentStorage<T> {
    fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    fn add(&mut self, entity: Entity, component: T) {
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

    fn size(&self) -> usize {
        self.components.len()
    }

    fn iter(&self) -> impl Iterator<Item = (Entity, &T)> {
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

pub(crate) struct ComponentManager<E>
where
    E: 'static,
{
    visual_storage: ComponentStorage<VisualComponent>,
    pub position_storage: ComponentStorage<PositionComponent>,
    input_storage: ComponentStorage<InputComponent<E>>,
    text_storage: ComponentStorage<TextComponent>,
    pub physics_storage: ComponentStorage<PhysicsComponent>,
}

impl<E> ComponentManager<E>
where
    E: 'static,
{
    pub fn create() -> Self {
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

    pub fn destroy(&mut self, device: &Device) {
        self.text_storage.destroy(device);
    }
}
