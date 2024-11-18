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
                previous: entity,
                next: entity,
                activate: input,
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

mod overlay {
    use crate::{
        ecs::{
            component::{Layer, PositionComponent, VisualComponent},
            entity::{EntityLoader, EntityManager},
        },
        GameEvent,
    };
    use glam::Vec3;
    use winit::event_loop::EventLoopProxy;

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
            self.create_wave_counter();
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
                open_inventory,
            );
        }

        fn create_wave_counter(&mut self) {
            let wave_counter = self.entity_manager.create_entity();
            self.loader.component_manager.visual_storage.add(
                wave_counter,
                VisualComponent::new(
                    vec![self
                        .loader
                        .resource_system
                        .get_texture_index("wave_counter")],
                    Layer::Interface,
                    0,
                ),
            );
            self.loader.component_manager.position_storage.add(
                wave_counter,
                PositionComponent {
                    xyz: Vec3 {
                        x: 0.0,
                        y: -0.8,
                        z: 0.0,
                    },
                    scale: Vec3 {
                        x: 0.6,
                        y: 0.1,
                        z: 1.0,
                    },
                },
            );
        }

        fn create_highscore(&mut self) {
            let highscore = self.entity_manager.create_entity();
            self.loader.component_manager.visual_storage.add(
                highscore,
                VisualComponent::new(
                    vec![self.loader.resource_system.get_texture_index("highscore")],
                    Layer::Interface,
                    0,
                ),
            );
            self.loader.component_manager.position_storage.add(
                highscore,
                PositionComponent {
                    xyz: Vec3 {
                        x: 0.0,
                        y: -0.9,
                        z: 0.0,
                    },
                    scale: Vec3 {
                        x: 0.6,
                        y: 0.1,
                        z: 1.0,
                    },
                },
            );
        }
        fn create_pause(&mut self) {
            let pause_button = self.entity_manager.create_entity();
            self.loader.create_clickable(
                pause_button,
                "pause_button",
                Vec3 {
                    x: -0.925,
                    y: -0.925,
                    z: 0.0,
                },
                Vec3 {
                    x: 0.05,
                    y: 0.05,
                    z: 1.0,
                },
                pause_clicked,
            );
        }
    }

    fn pause_clicked(_event_proxy: &EventLoopProxy<GameEvent>) {
        todo!("pause clicked")
    }

    fn open_inventory(_event_proxy: &EventLoopProxy<GameEvent>) {
        todo!("open inventory")
    }
}
