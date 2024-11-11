use crate::{
    ecs::{
        component::InputComponent,
        entity::{EntityLoader, EntityManager},
    },
    GameEvent,
};
use glam::Vec3;
use winit::event_loop::EventLoopProxy;

#[allow(clippy::too_many_lines)]
pub fn load(loader: &mut EntityLoader, entity_manager: &mut EntityManager) {
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
