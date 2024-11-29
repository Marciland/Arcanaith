mod game;
mod menu;

use crate::GameEvent;

use ash::Device;
use ecs::{Entity, InputHandler, MouseEvent, ECS};
use indexmap::IndexSet;
use winit::{event_loop::EventLoopProxy, keyboard::Key};

pub use game::Game;
pub use menu::{MainMenu, Menu, SettingsMenu};

pub enum Scene {
    Menu(Menu),
    Game(Game),
}

impl Scene {
    pub fn get_objects(&self) -> &[Entity] {
        match self {
            Scene::Menu(menu) => menu.get_objects(),
            Scene::Game(game) => game.get_objects(),
        }
    }

    pub fn destroy(&self, device: &Device, ecs: &mut ECS<GameEvent>) {
        match self {
            Scene::Menu(menu) => menu.destroy(device, ecs),
            Scene::Game(game) => game.destroy(device, ecs),
        }
    }
}

impl InputHandler<GameEvent> for Scene {
    fn handle_mouse_events(
        &self,
        ecs: &ECS<GameEvent>,
        events: &[MouseEvent],
        event_proxy: &EventLoopProxy<GameEvent>,
    ) {
        match self {
            Scene::Menu(menu) => menu.handle_mouse_events(ecs, events, event_proxy),
            Scene::Game(game) => game.handle_mouse_events(ecs, events, event_proxy),
        }
    }

    fn handle_key_events(
        &self,
        ecs: &ECS<GameEvent>,
        pressed_keys: &IndexSet<Key>,
        event_proxy: &EventLoopProxy<GameEvent>,
    ) {
        match self {
            Scene::Menu(menu) => {
                menu.handle_key_events(ecs, pressed_keys, event_proxy);
            }

            Scene::Game(game) => {
                game.handle_key_events(ecs, pressed_keys, event_proxy);
            }
        }
    }
}
