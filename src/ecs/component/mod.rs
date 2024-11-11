pub mod composition;
mod hover;
mod input;
pub mod player;
mod position;
mod visual;
use crate::ecs::entity::Entity;
pub use hover::HoverComponent;
pub use input::InputComponent;
pub use player::Player;
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

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Entity, &mut T)> {
        self.components
            .iter_mut()
            .map(|(entity, component)| (*entity, component))
    }
}

pub struct ComponentManager {
    pub player_entity: Option<Player>,
    pub visual_storage: ComponentStorage<VisualComponent>,
    pub position_storage: ComponentStorage<PositionComponent>,
    pub input_storage: ComponentStorage<InputComponent>,
    pub hover_storage: ComponentStorage<HoverComponent>,
}

impl ComponentManager {
    pub fn new() -> Self {
        Self {
            player_entity: None,
            visual_storage: ComponentStorage::new(),
            position_storage: ComponentStorage::new(),
            input_storage: ComponentStorage::new(),
            hover_storage: ComponentStorage::new(),
        }
    }

    pub fn clear_entity(&mut self, entity: Entity) {
        self.visual_storage.remove(entity);
        self.position_storage.remove(entity);
        self.input_storage.remove(entity);
        self.hover_storage.remove(entity);
    }
}
