mod loader;
use crate::{
    ecs::{component::ComponentManager, system::ResourceSystem},
    game::GameState,
};
use loader::EntityLoader;
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

    pub fn load(
        &mut self,
        game_state: &GameState,
        component_manager: &mut ComponentManager,
        resource_system: &ResourceSystem,
    ) {
        match game_state {
            GameState::Menu => {
                EntityLoader::load_main_menu(self, component_manager, resource_system);
            }
            GameState::_Game => {
                EntityLoader::load_game(self, component_manager, resource_system);
            }
            GameState::_Pause => {
                EntityLoader::load_pause_menu(self, component_manager, resource_system);
            }
        }
    }

    fn create_entity(&mut self) -> Entity {
        let entity = self.next_id;
        self.entities.insert(entity);
        self.next_id += 1;
        entity
    }

    /*
    fn remove_entity(&mut self, entity: Entity) {
        self.entities.remove(&entity);
    }

    fn is_valid(&self, entity: Entity) -> bool {
        self.entities.contains(&entity)
    }
    */
}
