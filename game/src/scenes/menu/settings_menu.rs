use crate::{scenes::Menu, GameEvent};

use ecs::{Entity, TextContent, ECS};
use glam::Vec2;
use objects::{Content, Factory};
use winit::event_loop::EventLoopProxy;

pub struct SettingsMenu {
    pub objects: Vec<Entity>,
}

impl Menu {
    fn create_back_button(ecs: &mut ECS<GameEvent>) -> Entity {
        Factory::button(
            ecs,
            Vec2 { x: 0.0, y: 0.5 },
            Vec2 { x: 0.5, y: 0.5 },
            Content::Text(TextContent {
                text: "Back".to_owned(),
                font: "test".to_owned(), // TODO adjust font
                font_size: 50.0,         // TODO adjust font size
            }),
            true,
            back_fn,
        )
    }
}

impl SettingsMenu {
    pub fn create(ecs: &mut ECS<GameEvent>) -> Self {
        let mut objects: Vec<Entity> = Vec::with_capacity(3);

        objects.push(Menu::create_background(ecs));
        objects.push(Menu::create_title(ecs));
        objects.push(Menu::create_back_button(ecs));

        Self { objects }
    }
}

fn back_fn(event_proxy: &EventLoopProxy<GameEvent>) {
    event_proxy
        .send_event(GameEvent::MainMenu)
        .expect("Failed to send MainMenu by pressing back button in settings!");
}
