use crate::{
    ecs::component::Layer,
    objects::{Button, Label, LabelContent, ObjectFactory, TextContent},
    GameEvent,
};
use glam::{Vec2, Vec3};
use winit::event_loop::EventLoopProxy;

pub struct SettingsMenu {
    pub background: Label,
    pub title: Label,
    pub back: Button,
}

impl SettingsMenu {
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

        let back = factory.new_button(
            Vec2 { x: 0.0, y: 0.5 },
            Vec2 { x: 0.5, y: 0.5 },
            &TextContent {
                text: "Back",
                font: "test",    // TODO adjust font
                font_size: 50.0, // TODO adjust font size
            },
            true,
            back_fn,
        );

        Self {
            background,
            title,
            back,
        }
    }
}

fn back_fn(event_proxy: &EventLoopProxy<GameEvent>) {
    event_proxy
        .send_event(GameEvent::Back)
        .expect("Failed to send back event by pressing back button in settings!");
}
