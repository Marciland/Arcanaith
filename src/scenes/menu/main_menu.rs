use crate::{
    ecs::component::Layer,
    objects::{Button, Label, LabelContent, Object, ObjectFactory, TextContent},
    scenes::Menu,
    GameEvent,
};
use ash::Device;
use glam::Vec2;
use winit::event_loop::EventLoopProxy;

impl Menu {
    fn create_banner(factory: &mut ObjectFactory) -> Label {
        factory.new_label(
            Vec2 { x: 0.0, y: 0.5 },
            Vec2 { x: 1.5, y: 0.5 },
            LabelContent::Image {
                name: "main_menu_banner",
                layer: Layer::Background,
            },
        )
    }

    fn create_new_game_button(factory: &mut ObjectFactory) -> Button {
        factory.new_button(
            Vec2 { x: -0.5, y: 0.0 },
            Vec2 { x: 0.5, y: 0.5 },
            &TextContent {
                text: "New Game",
                font: "test",    // TODO adjust font
                font_size: 50.0, // TODO adjust font size
            },
            true,
            new_game_fn,
        )
    }

    fn create_settings_button(factory: &mut ObjectFactory) -> Button {
        factory.new_button(
            Vec2 { x: 0.0, y: 0.0 },
            Vec2 { x: 0.5, y: 0.5 },
            &TextContent {
                text: "Settings",
                font: "test",    // TODO adjust font
                font_size: 50.0, // TODO adjust font size
            },
            false,
            settings_fn,
        )
    }

    fn create_exit_button(factory: &mut ObjectFactory) -> Button {
        factory.new_button(
            Vec2 { x: 0.5, y: 0.0 },
            Vec2 { x: 0.5, y: 0.5 },
            &TextContent {
                text: "Exit",
                font: "test",    // TODO adjust font
                font_size: 50.0, // TODO adjust font size
            },
            false,
            exit_fn,
        )
    }
}

pub struct MainMenu {
    objects: Vec<Object>,
}

impl MainMenu {
    pub fn create(factory: &mut ObjectFactory) -> Self {
        let mut objects: Vec<Object> = Vec::with_capacity(6);

        let background = Menu::create_background(factory);
        objects.push(Object::Label(background));

        let title = Menu::create_title(factory);
        objects.push(Object::Label(title));

        let banner = Menu::create_banner(factory);
        objects.push(Object::Label(banner));

        let new_game = Menu::create_new_game_button(factory);
        objects.push(Object::Button(new_game));

        let settings = Menu::create_settings_button(factory);
        objects.push(Object::Button(settings));

        let exit = Menu::create_exit_button(factory);
        objects.push(Object::Button(exit));

        Self { objects }
    }

    pub fn destroy(&self, device: &Device, factory: &mut ObjectFactory) {
        for obj in &self.objects {
            let entity = obj.id();

            factory.component_manager.clear_entity(entity, device);
            factory.entity_manager.destroy_entity(entity);
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
