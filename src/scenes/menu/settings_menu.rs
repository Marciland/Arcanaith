use crate::{
    objects::{Button, Object, ObjectFactory, TextContent},
    scenes::Menu,
    GameEvent,
};
use ash::Device;
use glam::Vec2;
use winit::event_loop::EventLoopProxy;

pub struct SettingsMenu {
    objects: Vec<Object>,
}

impl Menu {
    fn create_back_button(factory: &mut ObjectFactory) -> Button {
        factory.new_button(
            Vec2 { x: 0.0, y: 0.5 },
            Vec2 { x: 0.5, y: 0.5 },
            &TextContent {
                text: "Back",
                font: "test",    // TODO adjust font
                font_size: 50.0, // TODO adjust font size
            },
            true,
            back_fn,
        )
    }
}

impl SettingsMenu {
    pub fn create(factory: &mut ObjectFactory) -> Self {
        let mut objects: Vec<Object> = Vec::with_capacity(3);

        let background = Menu::create_background(factory);
        objects.push(Object::Label(background));

        let title = Menu::create_title(factory);
        objects.push(Object::Label(title));

        let back = Menu::create_back_button(factory);
        objects.push(Object::Button(back));

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

fn back_fn(event_proxy: &EventLoopProxy<GameEvent>) {
    event_proxy
        .send_event(GameEvent::Back)
        .expect("Failed to send back event by pressing back button in settings!");
}
