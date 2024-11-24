use crate::{
    ecs::{
        component::{PositionComponent, TextComponent, VisualComponent},
        entity::Entity,
    },
    objects::Content,
    ECS,
};
use glam::{Vec2, Vec3};

pub struct Label {
    pub id: Entity,
}

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

impl ECS {
    pub fn new_label<VecT: VecTExtend>(
        &mut self,
        position: VecT,
        size: Vec2,
        content: Content,
    ) -> Label {
        let id = self.entity_manager.create_entity();

        self.component_manager.position_storage.add(
            id,
            PositionComponent {
                xyz: position.extend(0.0),
                scale: size.extend(1.0),
            },
        );

        match content {
            Content::Text(content) => {
                self.component_manager
                    .text_storage
                    .add(id, TextComponent::create(content));
            }
            Content::Image { name, layer } => {
                self.component_manager.visual_storage.add(
                    id,
                    VisualComponent::new(
                        vec![self.system_manager.resource.get_texture_index(name)],
                        layer,
                        0,
                    ),
                );
            }
        }

        Label { id }
    }
}
