use crate::{
    ecs::component::Layer,
    objects::{Button, Label, LabelContent, ObjectFactory, TextContent},
    GameEvent,
};
use glam::{Vec2, Vec3};
use winit::event_loop::EventLoopProxy;

pub struct MainMenu {
    pub background: Label,
    pub title: Label,
    pub banner: Label,
    pub new_game: Button,
    pub settings: Button,
    pub exit: Button,
}

impl MainMenu {
    pub fn create(factory: &mut ObjectFactory) -> Self {
        let background = factory.new_label(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.1,
            },
            Vec2 { x: 2.0, y: 2.0 },
            LabelContent::Image {
                name: "menu_background",
                layer: Layer::Background,
            },
        );

        let title = factory.new_label(
            Vec2 { x: 0.0, y: -0.8 },
            Vec2 { x: 1.5, y: 0.5 },
            LabelContent::Image {
                name: "menu_title",
                layer: Layer::Background,
            },
        );

        let banner = factory.new_label(
            Vec2 { x: 0.0, y: 0.5 },
            Vec2 { x: 1.5, y: 0.5 },
            LabelContent::Image {
                name: "main_menu_banner",
                layer: Layer::Background,
            },
        );

        let new_game = factory.new_button(
            Vec2 { x: -0.5, y: 0.0 },
            Vec2 { x: 0.5, y: 0.5 },
            &TextContent {
                text: "New Game",
                font: "test",    // TODO adjust font
                font_size: 50.0, // TODO adjust font size
            },
            true,
            new_game_fn,
        );

        let settings = factory.new_button(
            Vec2 { x: 0.0, y: 0.0 },
            Vec2 { x: 0.5, y: 0.5 },
            &TextContent {
                text: "Settings",
                font: "test",    // TODO adjust font
                font_size: 50.0, // TODO adjust font size
            },
            false,
            settings_fn,
        );

        let exit = factory.new_button(
            Vec2 { x: 0.5, y: 0.0 },
            Vec2 { x: 0.5, y: 0.5 },
            &TextContent {
                text: "Exit",
                font: "test",    // TODO adjust font
                font_size: 50.0, // TODO adjust font size
            },
            false,
            exit_fn,
        );

        Self {
            background,
            title,
            banner,
            new_game,
            settings,
            exit,
        }
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
