use crate::{scenes::Menu, GameEvent};

use ecs::{Entity, Layer, TextContent, ECS};
use glam::Vec2;
use objects::{Content, Factory};
use winit::event_loop::EventLoopProxy;

impl Menu {
    fn create_banner(ecs: &mut ECS<GameEvent>) -> Entity {
        Factory::label(
            ecs,
            Vec2 { x: 0.0, y: 0.5 },
            Vec2 { x: 1.5, y: 0.5 },
            Content::Image {
                name: "main_menu_banner",
                layer: Layer::Background,
            },
        )
    }

    fn create_new_game_button(ecs: &mut ECS<GameEvent>) -> Entity {
        Factory::button(
            ecs,
            Vec2 { x: -0.5, y: 0.0 },
            Vec2 { x: 0.5, y: 0.5 },
            Content::Text(TextContent {
                text: "New Game".to_owned(),
                font: "test".to_owned(), // TODO adjust font
                font_size: 50.0,         // TODO adjust font size
            }),
            true,
            new_game_fn,
        )
    }

    fn create_settings_button(ecs: &mut ECS<GameEvent>) -> Entity {
        Factory::button(
            ecs,
            Vec2 { x: 0.0, y: 0.0 },
            Vec2 { x: 0.5, y: 0.5 },
            Content::Text(TextContent {
                text: "Settings".to_owned(),
                font: "test".to_owned(), // TODO adjust font
                font_size: 50.0,         // TODO adjust font size
            }),
            false,
            settings_fn,
        )
    }

    fn create_exit_button(ecs: &mut ECS<GameEvent>) -> Entity {
        Factory::button(
            ecs,
            Vec2 { x: 0.5, y: 0.0 },
            Vec2 { x: 0.5, y: 0.5 },
            Content::Text(TextContent {
                text: "Exit".to_owned(),
                font: "test".to_owned(), // TODO adjust font
                font_size: 50.0,         // TODO adjust font size
            }),
            false,
            exit_fn,
        )
    }
}

pub struct MainMenu {
    pub objects: Vec<Entity>,
}

impl MainMenu {
    pub fn create(ecs: &mut ECS<GameEvent>) -> Self {
        let mut objects: Vec<Entity> = Vec::with_capacity(6);

        objects.push(Menu::create_background(ecs));
        objects.push(Menu::create_title(ecs));
        objects.push(Menu::create_banner(ecs));

        let new_game = Menu::create_new_game_button(ecs);
        let settings = Menu::create_settings_button(ecs);
        let exit = Menu::create_exit_button(ecs);

        ecs.set_next_of(&new_game, &settings);
        ecs.set_next_of(&settings, &exit);
        ecs.set_next_of(&exit, &new_game);

        ecs.set_previous_of(&new_game, &exit);
        ecs.set_previous_of(&settings, &new_game);
        ecs.set_previous_of(&exit, &settings);

        objects.push(new_game);
        objects.push(settings);
        objects.push(exit);

        Self { objects }
    }
}

fn new_game_fn(event_proxy: &EventLoopProxy<GameEvent>) {
    event_proxy
        .send_event(GameEvent::NewGame)
        .expect("Failed to send new game event!");
}

fn settings_fn(event_proxy: &EventLoopProxy<GameEvent>) {
    event_proxy
        .send_event(GameEvent::SettingsMenu)
        .expect("Failed to send settings event!");
}

fn exit_fn(event_proxy: &EventLoopProxy<GameEvent>) {
    event_proxy
        .send_event(GameEvent::ExitGame)
        .expect("Failed to send exit event!");
}
