use ecs::{Component, Entity, Layer, PositionComponent, VisualComponent, ECS};
use glam::Vec2;

use super::Factory;

impl Factory {
    pub fn status_bar<E>(ecs: &mut ECS<E>, position: Vec2, size: Vec2) -> Entity {
        let status_bar = ecs.create_entity();

        ecs.add_component(
            status_bar,
            Component::Position(PositionComponent {
                xyz: position.extend(0.0),
                scale: size.extend(1.0),
            }),
        );

        ecs.add_component(
            status_bar,
            Component::Visual(VisualComponent::new(
                vec![ecs.get_texture_index("empty_bar")],
                Layer::Interface,
                0,
            )),
        );

        status_bar
    }
}
