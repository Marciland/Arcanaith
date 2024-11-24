use crate::{
    ecs::{
        component::{Layer, PositionComponent, TextComponent, VisualComponent},
        entity::Entity,
    },
    objects::TextContent,
    ECS,
};
use glam::Vec2;

pub struct IconText {
    pub id: Entity,
}

impl ECS {
    pub fn new_icon_text(
        &mut self,
        position: Vec2,
        size: Vec2,
        icon: &str,
        text: TextContent,
    ) -> IconText {
        let id = self.entity_manager.create_entity();

        self.component_manager.position_storage.add(
            id,
            PositionComponent {
                xyz: position.extend(0.0),
                scale: size.extend(1.0),
            },
        );

        self.component_manager.visual_storage.add(
            id,
            VisualComponent::new(
                vec![self.system_manager.resource.get_texture_index(icon)],
                Layer::Interface,
                0,
            ),
        );

        self.component_manager
            .text_storage
            .add(id, TextComponent::create(text));

        IconText { id }
    }
}
