use crate::Content;

use super::Factory;

use ecs::{Component, Entity, PositionComponent, TextComponent, VisualComponent, ECS};
use glam::{Vec2, Vec3};

pub trait VecTExtend {
    fn extend(self, z: f32) -> Vec3;
}

impl VecTExtend for Vec2 {
    fn extend(self, z: f32) -> Vec3 {
        self.extend(z)
    }
}

impl VecTExtend for Vec3 {
    fn extend(self, _: f32) -> Vec3 {
        self
    }
}

impl Factory {
    pub fn label<VecT: VecTExtend, E>(
        ecs: &mut ECS<E>,
        position: VecT,
        size: Vec2,
        content: Content,
    ) -> Entity {
        let label = ecs.create_entity();
        ecs.add_component(
            label,
            Component::Position(PositionComponent {
                xyz: position.extend(0.0),
                scale: size.extend(1.0),
            }),
        );

        match content {
            Content::Text(content) => {
                ecs.add_component(label, Component::Text(TextComponent::create(content)));
            }
            Content::Image { name, layer } => {
                ecs.add_component(
                    label,
                    Component::Visual(VisualComponent::new(
                        vec![ecs.get_texture_index(name)],
                        layer,
                        0,
                    )),
                );
            }
        }

        label
    }
}
