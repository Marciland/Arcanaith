use crate::{
    ecs::{
        component::{InputComponent, Layer, PositionComponent, VisualComponent},
        entity::{Entity, EntityLoader, EntityManager},
    },
    GameEvent,
};
use glam::Vec3;
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
        input: InputComponent,
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

        self.component_manager.input_storage.add(entity, input);
    }
}

pub fn load(loader: &mut EntityLoader, entity_manager: &mut EntityManager) {
    let health_bar = entity_manager.create_entity();
    loader.create_empty_bar(
        health_bar,
        Vec3 {
            x: -0.925,
            y: 0.925,
            z: 0.0,
        },
        Vec3 {
            x: 0.15,
            y: 0.05,
            z: 1.0,
        },
    );

    let mana_bar = entity_manager.create_entity();
    loader.create_empty_bar(
        mana_bar,
        Vec3 {
            x: -0.925,
            y: 0.975,
            z: 0.0,
        },
        Vec3 {
            x: 0.15,
            y: 0.05,
            z: 1.0,
        },
    );

    let exp_bar = entity_manager.create_entity();
    loader.create_empty_bar(
        exp_bar,
        Vec3 {
            x: 0.0,
            y: 0.975,
            z: 0.0,
        },
        Vec3 {
            x: 0.6,
            y: 0.05,
            z: 1.0,
        },
    );

    let money = entity_manager.create_entity();
    loader.component_manager.visual_storage.add(
        money,
        VisualComponent::new(
            vec![loader.resource_system.get_texture_index("money_bag")],
            Layer::Interface,
            0,
        ),
    );
    loader.component_manager.position_storage.add(
        money,
        PositionComponent {
            xyz: Vec3 {
                x: 0.7,
                y: 0.975,
                z: 0.0,
            },
            scale: Vec3 {
                x: 0.1,
                y: 0.05,
                z: 1.0,
            },
        },
    );

    let bag = entity_manager.create_entity();
    loader.create_clickable(
        bag,
        "bag",
        Vec3 {
            x: 0.85,
            y: 0.975,
            z: 0.0,
        },
        Vec3 {
            x: 0.1,
            y: 0.1,
            z: 1.0,
        },
        InputComponent {
            is_active: true,
            previous: bag,
            next: bag,
            activate: open_inventory,
        },
    );
    /*
    let pause = entity_manager.create_entity();
    loader.create_clickable(
        pause,
        Vec3 {
            x: (),
            y: (),
            z: (),
        },
        Vec3 {
            x: (),
            y: (),
            z: (),
        },
    );
    */
}

fn open_inventory(event_proxy: &EventLoopProxy<GameEvent>) {}
