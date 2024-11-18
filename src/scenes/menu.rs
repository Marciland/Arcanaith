use crate::ecs::{
    component::{InputComponent, Layer, PositionComponent, VisualComponent},
    entity::{Entity, EntityLoader, EntityManager},
};
use glam::Vec3;
pub use main_menu::create as create_main_menu;
pub use settings_menu::create as create_settings_menu;

impl<'loading> EntityLoader<'loading> {
    fn create_menu_entity(
        &mut self,
        entity: Entity,
        texture: &str,
        xyz: Vec3,
        scale: Vec3,
        input: Option<InputComponent>,
    ) {
        let layer = match input {
            Some(component) => {
                self.component_manager.input_storage.add(entity, component);
                Layer::Interface
            }
            None => Layer::Background,
        };

        self.component_manager.visual_storage.add(
            entity,
            VisualComponent::new(
                vec![self.resource_system.get_texture_index(texture)],
                layer,
                0,
            ),
        );

        self.component_manager
            .position_storage
            .add(entity, PositionComponent { xyz, scale });
    }

    fn create_background(&mut self, entity_manager: &mut EntityManager) {
        let background = entity_manager.create_entity();
        self.create_menu_entity(
            background,
            "menu_background",
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.1,
            },
            Vec3 {
                x: 2.0,
                y: 2.0,
                z: 1.0,
            },
            None,
        );
    }

    fn create_title(&mut self, entity_manager: &mut EntityManager) {
        let title = entity_manager.create_entity();
        self.create_menu_entity(
            title,
            "menu_title",
            Vec3 {
                x: 0.0,
                y: -0.8,
                z: 0.0,
            },
            Vec3 {
                x: 1.5,
                y: 0.5,
                z: 1.0,
            },
            None,
        );
    }
}

mod main_menu {
    use crate::{
        ecs::{
            component::{ComponentManager, InputComponent},
            entity::{EntityLoader, EntityManager},
            system::ResourceSystem,
        },
        GameEvent,
    };
    use glam::Vec3;
    use winit::event_loop::EventLoopProxy;

    #[allow(clippy::too_many_lines)]
    pub fn create(
        component_manager: &mut ComponentManager,
        resource_system: &ResourceSystem,
        entity_manager: &mut EntityManager,
    ) {
        let mut loader = EntityLoader {
            component_manager,
            resource_system,
        };

        let start_game = entity_manager.create_entity();
        let settings = entity_manager.create_entity();
        let exit = entity_manager.create_entity();
        let banner = entity_manager.create_entity();

        loader.create_background(entity_manager);
        loader.create_title(entity_manager);

        loader.create_menu_entity(
            start_game,
            "main_menu_start_game",
            Vec3 {
                x: -0.5,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 0.5,
                y: 0.5,
                z: 1.0,
            },
            Some(InputComponent {
                is_active: true,
                previous: exit,
                next: settings,
                activate: start_game_fn,
            }),
        );

        loader.create_menu_entity(
            settings,
            "main_menu_settings",
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 0.5,
                y: 0.5,
                z: 1.0,
            },
            Some(InputComponent {
                is_active: false,
                previous: start_game,
                next: exit,
                activate: settings_fn,
            }),
        );

        loader.create_menu_entity(
            exit,
            "main_menu_exit",
            Vec3 {
                x: 0.5,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 0.5,
                y: 0.5,
                z: 1.0,
            },
            Some(InputComponent {
                is_active: false,
                previous: settings,
                next: start_game,
                activate: exit_fn,
            }),
        );

        loader.create_menu_entity(
            banner,
            "main_menu_banner",
            Vec3 {
                x: 0.0,
                y: 0.5,
                z: 0.0,
            },
            Vec3 {
                x: 1.5,
                y: 0.5,
                z: 1.0,
            },
            None,
        );
    }

    fn start_game_fn(event_proxy: &EventLoopProxy<GameEvent>) {
        event_proxy
            .send_event(GameEvent::NewGame)
            .expect("Failed to send new game event!");
    }

    fn settings_fn(event_proxy: &EventLoopProxy<GameEvent>) {
        event_proxy
            .send_event(GameEvent::SettingsMenu)
            .expect("Failed to send settings event!");
    }

    fn exit_fn(event_proxy: &EventLoopProxy<GameEvent>) {
        event_proxy
            .send_event(GameEvent::ExitGame)
            .expect("Failed to send exit event!");
    }
}

mod settings_menu {
    use crate::{
        ecs::{
            component::{ComponentManager, InputComponent},
            entity::{EntityLoader, EntityManager},
            system::ResourceSystem,
        },
        GameEvent,
    };
    use glam::Vec3;
    use winit::event_loop::EventLoopProxy;

    pub fn create(
        component_manager: &mut ComponentManager,
        resource_system: &ResourceSystem,
        entity_manager: &mut EntityManager,
    ) {
        let mut loader = EntityLoader {
            component_manager,
            resource_system,
        };

        let back = entity_manager.create_entity();

        loader.create_background(entity_manager);
        loader.create_title(entity_manager);

        loader.create_menu_entity(
            back,
            "settings_back",
            Vec3 {
                x: 0.0,
                y: 0.5,
                z: 0.0,
            },
            Vec3 {
                x: 0.5,
                y: 0.5,
                z: 1.0,
            },
            Some(InputComponent {
                is_active: true,
                previous: back,
                next: back,
                activate: back_fn,
            }),
        );
    }

    fn back_fn(event_proxy: &EventLoopProxy<GameEvent>) {
        event_proxy
            .send_event(GameEvent::Back)
            .expect("Failed to send back event by pressing back button in settings!");
    }
}
