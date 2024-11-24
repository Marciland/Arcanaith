use crate::{
    ecs::{
        component::{Layer, PositionComponent, VisualComponent},
        entity::Entity,
    },
    ECS,
};
use glam::Vec3;

pub struct Player {
    pub id: Entity,
}

impl Player {
    pub fn create(ecs: &mut ECS) -> Self {
        let id = ecs.entity_manager.create_entity();

        ecs.component_manager.position_storage.add(
            id,
            PositionComponent {
                xyz: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                scale: Vec3 {
                    x: 0.3,
                    y: 0.3,
                    z: 1.0,
                },
            },
        );

        ecs.component_manager.visual_storage.add(
            id,
            VisualComponent::new(
                vec![ecs.system_manager.resource.get_texture_index("player_0")],
                Layer::Game,
                0,
            ),
        );

        //ecs.component_manager.physics_storage.add(id, component);

        Self { id }
    }
}
