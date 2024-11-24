use crate::{
    ecs::{
        component::{Layer, PositionComponent, VisualComponent},
        entity::Entity,
    },
    ECS,
};
use glam::Vec2;

pub struct StatusBar {
    pub id: Entity,
}

impl ECS {
    pub fn new_status_bar(&mut self, position: Vec2, size: Vec2) -> StatusBar {
        let id = self.entity_manager.create_entity();

        self.component_manager.visual_storage.add(
            id,
            VisualComponent::new(
                vec![self.system_manager.resource.get_texture_index("empty_bar")],
                Layer::Interface,
                0,
            ),
        );

        self.component_manager.position_storage.add(
            id,
            PositionComponent {
                xyz: position.extend(0.0),
                scale: size.extend(1.0),
            },
        );

        StatusBar { id }
    }
}
