use crate::{
    ecs::component::Layer,
    objects::{Button, Content, Label, Object, TextContent},
    scenes::Menu,
    GameEvent, ECS,
};
use glam::Vec2;
use winit::event_loop::EventLoopProxy;

impl Menu {
    fn create_banner(ecs: &mut ECS) -> Label {
        ecs.new_label(
            Vec2 { x: 0.0, y: 0.5 },
            Vec2 { x: 1.5, y: 0.5 },
            Content::Image {
                name: "main_menu_banner",
                layer: Layer::Background,
            },
        )
    }

    fn create_new_game_button(ecs: &mut ECS) -> Button {
        ecs.new_button(
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

    fn create_settings_button(ecs: &mut ECS) -> Button {
        ecs.new_button(
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

    fn create_exit_button(ecs: &mut ECS) -> Button {
        ecs.new_button(
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
    pub objects: Vec<Object>,
}

impl MainMenu {
    pub fn create(ecs: &mut ECS) -> Self {
        let mut objects: Vec<Object> = Vec::with_capacity(6);

        let background = Menu::create_background(ecs);
        objects.push(Object::Label(background));

        let title = Menu::create_title(ecs);
        objects.push(Object::Label(title));

        let banner = Menu::create_banner(ecs);
        objects.push(Object::Label(banner));

        let new_game = Menu::create_new_game_button(ecs);
        let settings = Menu::create_settings_button(ecs);
        let exit = Menu::create_exit_button(ecs);

        new_game.set_next(&settings, &mut ecs.component_manager);
        settings.set_next(&exit, &mut ecs.component_manager);
        exit.set_next(&new_game, &mut ecs.component_manager);

        new_game.set_previous(&exit, &mut ecs.component_manager);
        settings.set_previous(&new_game, &mut ecs.component_manager);
        exit.set_previous(&settings, &mut ecs.component_manager);

        objects.push(Object::Button(new_game));
        objects.push(Object::Button(settings));
        objects.push(Object::Button(exit));

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
