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
