use std::collections::HashSet;

pub type Entity = u32;

#[derive(Default)]
pub(crate) struct EntityManager {
    next_id: Entity,
    entities: HashSet<Entity>,
}

impl EntityManager {
    pub fn create_entity(&mut self) -> Entity {
        let entity = self.next_id;
        self.entities.insert(entity);
        self.next_id += 1;
        entity
    }

    pub fn destroy_entity(&mut self, entity: Entity) {
        self.entities.remove(&entity);
    }
}

pub trait EntityProvider {
    fn get_entities(&self) -> &[Entity];
    fn get_player(&self) -> Option<Entity>;
}
