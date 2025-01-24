mod game;
mod menu;

use crate::GameEvent;

use ecs::{Entity, EntityProvider, InputHandler, MouseEvent, ECS};
use indexmap::IndexSet;
use winit::{event_loop::EventLoopProxy, keyboard::Key};

pub use game::Game;
pub use menu::{MainMenu, Menu, SettingsMenu};

pub enum Scene {
    None,
    Menu(Menu),
    Game(Game),
}

impl Scene {
    pub fn get_objects(&self) -> &[Entity] {
        match self {
            Scene::None => &[],
            Scene::Menu(menu) => menu.get_objects(),
            Scene::Game(game) => game.get_objects(),
        }
    }

    pub fn destroy(&self, ecs: &mut ECS<GameEvent>) {
        match self {
            Scene::None => (),
            Scene::Menu(menu) => menu.destroy(ecs),
            Scene::Game(game) => game.destroy(ecs),
        }
    }
}

impl EntityProvider for Scene {
    fn get_entities(&self) -> &[Entity] {
        self.get_objects()
    }

    fn get_player(&self) -> Option<Entity> {
        match self {
            Scene::None | Scene::Menu(_) => None,
            Scene::Game(game) => Some(game.player_id),
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
            Scene::None => panic!("Should not handle mouse events in None scene"),
            Scene::Menu(menu) => menu.handle_mouse_events(ecs, events, event_proxy),
            Scene::Game(game) => game.handle_mouse_events(ecs, events, event_proxy),
        }
    }

    fn handle_key_events(
        &self,
        ecs: &mut ECS<GameEvent>,
        pressed_keys: &IndexSet<Key>,
        event_proxy: &EventLoopProxy<GameEvent>,
    ) {
        match self {
            Scene::None => panic!("Should not handle key events in None scene"),
            Scene::Menu(menu) => {
                menu.handle_key_events(ecs, pressed_keys, event_proxy);
            }

            Scene::Game(game) => {
                game.handle_key_events(ecs, pressed_keys, event_proxy);
            }
        }
    }
}
