use crate::ecs::{
    component::{player::PlayerState, Player, PositionComponent},
    entity::{EntityLoader, EntityManager},
};
use glam::Vec3;

pub fn create(loader: &mut EntityLoader, entity_manager: &mut EntityManager) -> Player {
    let id = entity_manager.create_entity();
    let state = PlayerState::Idle;
    let start_position = PositionComponent {
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
    };

    loader
        .component_manager
        .position_storage
        .add(id, start_position);
    loader
        .component_manager
        .visual_storage
        .add(id, state.get_visual(loader.resource_system));

    Player { id, state }
}
