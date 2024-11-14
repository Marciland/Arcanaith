mod overlay;
mod player;
use crate::ecs::{
    component::{Layer, PositionComponent, VisualComponent},
    entity::{EntityLoader, EntityManager},
};
use glam::Vec3;
use overlay::OverlayLoader;

pub fn load_new_game(loader: &mut EntityLoader, entity_manager: &mut EntityManager) {
    loader.component_manager.player_entity = Some(player::create(loader, entity_manager));

    // TODO animate background and never move it instead?
    let background = entity_manager.create_entity();
    loader.component_manager.position_storage.add(
        background,
        PositionComponent {
            xyz: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            scale: Vec3 {
                x: 2.0,
                y: 2.0,
                z: 1.0,
            },
        },
    );
    loader.component_manager.visual_storage.add(
        background,
        VisualComponent::new(
            vec![loader.resource_system.get_texture_index("game_background")],
            Layer::Background,
            0,
        ),
    );

    OverlayLoader {
        loader,
        entity_manager,
    }
    .load();

    todo!("spawn waves mechanic")
}
