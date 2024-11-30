use crate::Content;

use super::Factory;

use ecs::{
    Component, Entity, InputComponent, PositionComponent, TextComponent, VisualComponent, ECS,
};
use glam::Vec2;
use winit::event_loop::EventLoopProxy;

impl Factory {
    pub fn button<E>(
        ecs: &mut ECS<E>,
        position: Vec2,
        size: Vec2,
        content: Content,
        is_focused: bool,
        callback: fn(&EventLoopProxy<E>) -> (),
    ) -> Entity {
        let button = ecs.create_entity();

        ecs.add_component(
            button,
            Component::Position(PositionComponent {
                xyz: position.extend(0.0),
                scale: size.extend(1.0),
            }),
        );

        match content {
            Content::Image { name, layer } => {
                ecs.add_component(
                    button,
                    Component::Visual(VisualComponent::new(
                        vec![ecs.get_texture_index(name)],
                        layer,
                        0,
                    )),
                );
            }
            Content::Text(content) => {
                ecs.add_component(button, Component::Text(TextComponent::create(content)));
            }
        }

        ecs.add_component(
            button,
            Component::Input(InputComponent {
                is_active: is_focused,
                activate: callback,
                next: None,
                previous: None,
            }),
        );

        button
    }
}
