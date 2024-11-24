use std::collections::HashSet;

pub type Entity = u32;

pub struct EntityManager {
    next_id: Entity,
    entities: HashSet<Entity>,
}

impl Default for EntityManager {
    fn default() -> Self {
        Self::new()
    }
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
}
