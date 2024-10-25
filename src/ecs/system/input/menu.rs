use crate::{
    ecs::{component::ComponentManager, entity::Entity},
    GameEvent,
};
use winit::{
    event_loop::EventLoopProxy,
    keyboard::{Key, NamedKey},
};

pub fn handle_key_event(
    key: &Key,
    active: Entity,
    component_manager: &mut ComponentManager,
    event_proxy: &EventLoopProxy<GameEvent>,
) {
    match key {
        Key::Named(NamedKey::Tab) => {
            {
                let active_input = component_manager
                    .input_storage
                    .get_mut(active)
                    .expect("Failed to get mut ref on active entity!");
                active_input.is_active = false;
            }
            let active_input = component_manager
                .input_storage
                .get(active)
                .expect("Failed to get ref on active entity!");
            let next = component_manager
                .input_storage
                .get_mut(active_input.next)
                .expect("Input component has no valid next entity!");
            next.is_active = true;
        }
        Key::Named(NamedKey::Space | NamedKey::Enter) => {
            let active_input = component_manager
                .input_storage
                .get(active)
                .expect("Failed to get ref on active entity!");

            (active_input.activate)(event_proxy);
        }
        _ => (),
    }
}
