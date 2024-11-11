mod game;
mod menu;
use crate::ecs::{component::ComponentManager, entity::EntityManager, system::ResourceSystem};

pub struct EntityLoader<'loading> {
    pub component_manager: &'loading mut ComponentManager,
    pub resource_system: &'loading ResourceSystem,
}

impl<'loading> EntityLoader<'loading> {
    pub fn load_main_menu(&mut self, entity_manager: &mut EntityManager) {
        menu::load_main_menu(self, entity_manager);
    }

    pub fn load_settings_menu(&mut self, entity_manager: &mut EntityManager) {
        menu::load_settings_menu(self, entity_manager);
    }

    pub fn load_game(&mut self, entity_manager: &mut EntityManager) {
        game::load_new_game(self, entity_manager);
    }
}
