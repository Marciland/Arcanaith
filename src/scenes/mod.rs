mod game;
mod menu;

use crate::objects::ObjectFactory;
use ash::Device;

pub use game::create_new_game;
pub use menu::{MainMenu, Menu, SettingsMenu};

pub enum Scene {
    None,
    MainMenu(MainMenu),
    SettingsMenu(SettingsMenu),
}

impl Scene {
    pub fn destroy(&self, device: &Device, factory: &mut ObjectFactory) {
        match self {
            Scene::MainMenu(menu) => menu.destroy(device, factory),
            Scene::SettingsMenu(menu) => menu.destroy(device, factory),
            Scene::None => (),
        }
    }
}
