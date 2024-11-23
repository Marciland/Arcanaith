use super::{ObjectFactory, TextContent};
use crate::{
    ecs::{
        component::{InputComponent, PositionComponent, TextComponent},
        entity::Entity,
    },
    GameEvent,
};
use glam::Vec2;
use winit::event_loop::EventLoopProxy;

pub struct Button {
    pub id: Entity,
}

impl<'building> ObjectFactory<'building> {
    pub fn new_button(
        &mut self,
        position: Vec2,
        size: Vec2,
        content: &TextContent,
        is_focused: bool,
        callback: fn(&EventLoopProxy<GameEvent>) -> (),
    ) -> Button {
        let id = self.entity_manager.create_entity();

        self.component_manager.position_storage.add(
            id,
            PositionComponent {
                xyz: position.extend(0.0),
                scale: size.extend(1.0),
            },
        );

        self.component_manager.text_storage.add(
            id,
            TextComponent::create(content.text, content.font, content.font_size),
        );

        self.component_manager.input_storage.add(
            id,
            InputComponent {
                is_active: is_focused,
                activate: callback,
                next: None,
                previous: None,
            },
        );

        Button { id }
    }
}
