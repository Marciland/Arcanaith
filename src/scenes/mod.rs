mod game;
mod menu;

use crate::{
    ecs::{component::ComponentManager, system::input::InputHandler},
    objects::Object,
    GameEvent, MouseEvent, ECS,
};
use ash::Device;
use indexmap::IndexSet;
use winit::{event_loop::EventLoopProxy, keyboard::Key};

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

impl InputHandler for Scene {
    fn handle_mouse_events(
        &self,
        events: &[MouseEvent],
        component_manager: &mut ComponentManager,
        event_proxy: &EventLoopProxy<GameEvent>,
    ) {
        match self {
            Scene::Menu(menu) => menu.handle_mouse_events(events, component_manager, event_proxy),
            Scene::Game(game) => game.handle_mouse_events(events, component_manager, event_proxy),
        }
    }

    fn handle_key_events(
        &self,
        pressed_keys: &IndexSet<Key>,
        component_manager: &mut ComponentManager,
        event_proxy: &EventLoopProxy<GameEvent>,
    ) {
        match self {
            Scene::Menu(menu) => {
                menu.handle_key_events(pressed_keys, component_manager, event_proxy);
            }

            Scene::Game(game) => {
                game.handle_key_events(pressed_keys, component_manager, event_proxy);
            }
        }
    }
}
