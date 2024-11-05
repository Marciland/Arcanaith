use super::{mouse::any_component_was_clicked, MouseEvent};
use crate::{
    ecs::{
        component::{composition::InputWithPosition, ComponentManager},
        entity::Entity,
    },
    GameEvent,
};
use indexmap::IndexSet;
use winit::{
    event::MouseButton,
    event_loop::EventLoopProxy,
    keyboard::{Key, NamedKey},
};

pub fn handle_key_events(
    pressed_keys: &IndexSet<Key>,
    active_entity: Entity,
    component_manager: &mut ComponentManager,
    event_proxy: &EventLoopProxy<GameEvent>,
) {
    for key in pressed_keys {
        match key {
            Key::Named(NamedKey::Tab | NamedKey::ArrowDown | NamedKey::ArrowRight) => {
                set_next_entity_to_active(active_entity, component_manager);
            }

            Key::Named(NamedKey::ArrowLeft | NamedKey::ArrowUp) => {
                set_previous_entity_to_active(active_entity, component_manager);
            }

            Key::Named(NamedKey::Space | NamedKey::Enter) => {
                let active_input = component_manager
                    .input_storage
                    .get(active_entity)
                    .expect("Failed to get ref on active entity!");

                (active_input.activate)(event_proxy);
            }

            Key::Named(NamedKey::Escape) => {
                event_proxy
                    .send_event(GameEvent::Back)
                    .expect("Failed to send back event by pressing escape!");
            }
            _ => (),
        }
    }
}

pub fn handle_mouse_events(
    events: &[MouseEvent],
    components: &[InputWithPosition],
    event_proxy: &EventLoopProxy<GameEvent>,
) {
    for event in events {
        if event.button.mouse_button == MouseButton::Left {
            match any_component_was_clicked(components, &event.position) {
                Some(function) => (function)(event_proxy),
                None => continue,
            }
        }
    }
}

fn set_active_entity_to_inactive(active: Entity, component_manager: &mut ComponentManager) {
    let active_input = component_manager
        .input_storage
        .get_mut(active)
        .expect("Failed to get mut ref on active entity!");
    active_input.is_active = false;
}

fn set_next_entity_to_active(active: Entity, component_manager: &mut ComponentManager) {
    set_active_entity_to_inactive(active, component_manager);

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

fn set_previous_entity_to_active(active: Entity, component_manager: &mut ComponentManager) {
    set_active_entity_to_inactive(active, component_manager);

    let active_input = component_manager
        .input_storage
        .get(active)
        .expect("Failed to get ref on active entity!");
    let previous = component_manager
        .input_storage
        .get_mut(active_input.previous)
        .expect("Input component has no valid previous entity!");
    previous.is_active = true;
}
