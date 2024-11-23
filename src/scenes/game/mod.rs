mod overlay;

use crate::{
    ecs::{
        component::{
            ComponentManager, InputComponent, Layer, Player, PositionComponent, VisualComponent,
        },
        entity::{Entity, EntityLoader, EntityManager},
        system::ResourceSystem,
    },
    GameEvent,
};
use glam::Vec3;
use overlay::OverlayLoader;
use winit::event_loop::EventLoopProxy;

impl<'loading> EntityLoader<'loading> {
    fn create_empty_bar(&mut self, entity: Entity, xyz: Vec3, scale: Vec3) {
        self.component_manager.visual_storage.add(
            entity,
            VisualComponent::new(
                vec![self.resource_system.get_texture_index("empty_bar")],
                Layer::Interface,
                0,
            ),
        );

        self.component_manager
            .position_storage
            .add(entity, PositionComponent { xyz, scale });
    }

    fn create_clickable(
        &mut self,
        entity: Entity,
        texture: &str,
        xyz: Vec3,
        scale: Vec3,
        input: fn(&EventLoopProxy<GameEvent>) -> (),
    ) {
        self.component_manager.visual_storage.add(
            entity,
            VisualComponent::new(
                vec![self.resource_system.get_texture_index(texture)],
                Layer::Interface,
                0,
            ),
        );

        self.component_manager
            .position_storage
            .add(entity, PositionComponent { xyz, scale });

        self.component_manager.input_storage.add(
            entity,
            InputComponent {
                is_active: false,
                activate: input,
                next: None,
                previous: None,
            },
        );
    }
}

pub fn create_new_game(
    component_manager: &mut ComponentManager,
    resource_system: &ResourceSystem,
    entity_manager: &mut EntityManager,
) {
    component_manager.player_entity = Some(Player::create(
        component_manager,
        resource_system,
        entity_manager,
    ));

    // TODO animate background and never move it instead?
    let background = entity_manager.create_entity();
    component_manager.position_storage.add(
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
    component_manager.visual_storage.add(
        background,
        VisualComponent::new(
            vec![resource_system.get_texture_index("game_background")],
            Layer::Background,
            0,
        ),
    );

    let mut loader = EntityLoader {
        component_manager,
        resource_system,
    };

    OverlayLoader {
        loader: &mut loader,
        entity_manager,
    }
    .load();

    // todo!("spawn waves mechanic")
}
