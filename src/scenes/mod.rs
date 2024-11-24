mod game;
mod menu;

use crate::{objects::Object, ECS};
use ash::Device;

pub use game::Game;
pub use menu::{MainMenu, Menu, SettingsMenu};

pub enum Scene {
    Menu(Menu),
    Game(Game),
}

impl Scene {
    pub fn get_objects(&self) -> &[Object] {
        match self {
            Scene::Menu(menu) => menu.get_objects(),
            Scene::Game(game) => game.get_objects(),
        }
    }

    pub fn destroy(&self, device: &Device, ecs: &mut ECS) {
        match self {
            Scene::Menu(menu) => menu.destroy(device, ecs),
            Scene::Game(game) => game.destroy(device, ecs),
        }
    }
}
