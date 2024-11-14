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

pub struct OverlayLoader<'overlay, 'loader> {
    pub loader: &'overlay mut EntityLoader<'loader>,
    pub entity_manager: &'overlay mut EntityManager,
}

impl<'overlay, 'loader> OverlayLoader<'overlay, 'loader> {
    pub fn load(&mut self) {
        self.create_health_bar();
        self.create_mana_bar();
        self.create_exp_bar();
        self.create_money();
        self.create_inventory();
        self.create_highscore();
        self.create_pause();
    }

    fn create_health_bar(&mut self) {
        let health_bar = self.entity_manager.create_entity();
        self.loader.create_empty_bar(
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
    }

    fn create_mana_bar(&mut self) {
        let mana_bar = self.entity_manager.create_entity();
        self.loader.create_empty_bar(
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
    }

    fn create_exp_bar(&mut self) {
        let exp_bar = self.entity_manager.create_entity();
        self.loader.create_empty_bar(
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
    }

    fn create_money(&mut self) {
        let money = self.entity_manager.create_entity();
        self.loader.component_manager.visual_storage.add(
            money,
            VisualComponent::new(
                vec![self.loader.resource_system.get_texture_index("money_bag")],
                Layer::Interface,
                0,
            ),
        );
        self.loader.component_manager.position_storage.add(
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
    }

    fn create_inventory(&mut self) {
        let inventory = self.entity_manager.create_entity();
        self.loader.create_clickable(
            inventory,
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
                previous: inventory,
                next: inventory,
                activate: open_inventory,
            },
        );
    }

    fn create_highscore(&mut self) {
        todo!("highscore display")
    }
    fn create_pause(&mut self) {
        todo!("pause button")
    }
}

fn open_inventory(_event_proxy: &EventLoopProxy<GameEvent>) {
    todo!("open inventory")
}
