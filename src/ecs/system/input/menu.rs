use crate::{
    ecs::{
        component::{composition::InputWithPosition, ComponentManager},
        entity::Entity,
        system::input::{InputHandler, InputSystem, MouseHandler},
    },
    scenes::Menu,
    GameEvent, MouseEvent,
};
use indexmap::IndexSet;
use winit::{
    event::MouseButton,
    event_loop::EventLoopProxy,
    keyboard::{Key, NamedKey},
};

impl InputHandler for Menu {
    fn handle_mouse_events(
        &self,
        events: &[MouseEvent],
        component_manager: &mut ComponentManager,
        event_proxy: &EventLoopProxy<GameEvent>,
    ) {
        for event in events {
            if event.button.mouse_button != MouseButton::Left {
                continue;
            }

            let objects = self.get_objects();
            let mut clickables = Vec::with_capacity(objects.len());

            for obj in objects {
                let entity = obj.id();

                let Some(input) = component_manager.input_storage.get(entity) else {
                    continue;
                };

                let Some(position) = component_manager.position_storage.get(entity) else {
                    continue;
                };

                clickables.push(InputWithPosition { input, position });
            }

            match InputSystem::any_object_was_clicked(&clickables, &event.position) {
                Some(function) => (function)(event_proxy),
                None => continue,
            }
        }
    }

    fn handle_key_events(
        &self,
        pressed_keys: &IndexSet<Key>,
        component_manager: &mut ComponentManager,
        event_proxy: &EventLoopProxy<GameEvent>,
    ) {
        let active_entity = self.get_active(component_manager);

        for key in pressed_keys {
            match key {
                Key::Named(NamedKey::Tab | NamedKey::ArrowDown | NamedKey::ArrowRight) => {
                    if active_entity.is_none() {
                        continue;
                    }
                    set_next_entity_to_active(active_entity.unwrap(), component_manager);
                }

                Key::Named(NamedKey::ArrowLeft | NamedKey::ArrowUp) => {
                    if active_entity.is_none() {
                        continue;
                    }
                    set_previous_entity_to_active(active_entity.unwrap(), component_manager);
                }

                Key::Named(NamedKey::Space | NamedKey::Enter) => {
                    if active_entity.is_none() {
                        continue;
                    }
                    let active_input = component_manager
                        .input_storage
                        .get(active_entity.unwrap())
                        .expect("Failed to get ref on active entity!");

                    (active_input.activate)(event_proxy);
                }

                Key::Named(NamedKey::Escape) => {
                    if let Menu::SettingsMenu(_) = self {
                        event_proxy
                            .send_event(GameEvent::MainMenu)
                            .expect("Failed to send MainMenu by pressing escape!");
                    }
                }
                _ => (),
            }
        }
    }
}

fn set_next_entity_to_active(active: Entity, component_manager: &mut ComponentManager) {
    let active_input = component_manager
        .input_storage
        .get_mut(active)
        .expect("Failed to get ref on active entity!");

    if let Some(next_entity) = active_input.next {
        active_input.is_active = false;

        let next = component_manager
            .input_storage
            .get_mut(next_entity)
            .expect("Input component has no valid next entity!");

        next.is_active = true;
    }
}

fn set_previous_entity_to_active(active: Entity, component_manager: &mut ComponentManager) {
    let active_input = component_manager
        .input_storage
        .get_mut(active)
        .expect("Failed to get ref on active entity!");

    if let Some(previous_entity) = active_input.previous {
        active_input.is_active = false;

        let previous = component_manager
            .input_storage
            .get_mut(previous_entity)
            .expect("Input component has no valid previous entity!");

        previous.is_active = true;
    }
}
