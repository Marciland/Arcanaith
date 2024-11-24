use crate::{
    objects::{Button, Content, Object, TextContent},
    scenes::Menu,
    GameEvent, ECS,
};
use glam::Vec2;
use winit::event_loop::EventLoopProxy;

pub struct SettingsMenu {
    pub objects: Vec<Object>,
}

impl Menu {
    fn create_back_button(ecs: &mut ECS) -> Button {
        ecs.new_button(
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
    pub fn create(ecs: &mut ECS) -> Self {
        let mut objects: Vec<Object> = Vec::with_capacity(3);

        let background = Menu::create_background(ecs);
        objects.push(Object::Label(background));

        let title = Menu::create_title(ecs);
        objects.push(Object::Label(title));

        let back = Menu::create_back_button(ecs);
        objects.push(Object::Button(back));

        Self { objects }
    }
}

fn back_fn(event_proxy: &EventLoopProxy<GameEvent>) {
    event_proxy
        .send_event(GameEvent::MainMenu)
        .expect("Failed to send MainMenu by pressing back button in settings!");
}
