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
        let mut loader = EntityLoader {
            component_manager,
            resource_system,
        };

        match game_state {
            GameState::MainMenu => {
                loader.load_main_menu(self);
            }
            GameState::Game => {
                loader.load_game(self);
            }
            GameState::_Pause => {
                todo!("load pause menu")
            }
            GameState::Settings => {
                loader.load_settings_menu(self);
            }
        }
    }

    fn create_entity(&mut self) -> Entity {
        let entity = self.next_id;
        self.entities.insert(entity);
        self.next_id += 1;
        entity
    }

    pub fn clear(&mut self, component_manager: &mut ComponentManager) {
        for entity in &self.entities {
            component_manager.clear_entity(*entity);
        }
        self.entities.clear();
    }
}
