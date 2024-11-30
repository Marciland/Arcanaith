use ecs::{
    Component, Entity, Layer, PositionComponent, TextComponent, TextContent, VisualComponent, ECS,
};
use glam::Vec2;

use super::Factory;

impl Factory {
    pub fn icon_with_text<E>(
        ecs: &mut ECS<E>,
        position: Vec2,
        size: Vec2,
        icon: &str,
        text: TextContent,
    ) -> Entity {
        let icon_with_text: u32 = ecs.create_entity();

        ecs.add_component(
            icon_with_text,
            Component::Position(PositionComponent {
                xyz: position.extend(0.0),
                scale: size.extend(1.0),
            }),
        );

        ecs.add_component(
            icon_with_text,
            Component::Visual(VisualComponent::new(
                vec![ecs.get_texture_index(icon)],
                Layer::Interface,
                0,
            )),
        );

        ecs.add_component(icon_with_text, Component::Text(TextComponent::create(text)));

        icon_with_text
    }
}
