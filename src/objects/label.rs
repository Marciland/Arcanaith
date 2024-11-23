use super::ObjectFactory;
use crate::ecs::{
    component::{Layer, PositionComponent, VisualComponent},
    entity::Entity,
};
use glam::{Vec2, Vec3};

pub enum LabelContent<'a> {
    Image { name: &'a str, layer: Layer },
}

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

impl<'building> ObjectFactory<'building> {
    pub fn new_label<VecT: VecTExtend>(
        &mut self,
        position: VecT,
        size: Vec2,
        content: LabelContent,
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
            LabelContent::Image { name, layer } => {
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
