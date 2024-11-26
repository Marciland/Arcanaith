use crate::{
    ecs::{
        component::{composition::InputWithPosition, ComponentManager, InputComponent},
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
        for key in pressed_keys {
            match key {
                Key::Named(NamedKey::Tab | NamedKey::ArrowDown | NamedKey::ArrowRight) => {
                    self.set_next_entity_to_active(component_manager);
                }

                Key::Named(NamedKey::ArrowLeft | NamedKey::ArrowUp) => {
                    self.set_previous_entity_to_active(component_manager);
                }

                Key::Named(NamedKey::Space | NamedKey::Enter) => {
                    if let Some(active_input) = self.get_active_input(component_manager) {
                        (active_input.activate)(event_proxy);
                    }
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

impl Menu {
    fn get_active_input<'input>(
        &'input self,
        component_manager: &'input ComponentManager,
    ) -> Option<&InputComponent> {
        let active_entity = self.get_active(component_manager)?;
        let active_input = component_manager.input_storage.get(active_entity)?;

        Some(active_input)
    }

    fn get_active_input_mut<'input>(
        &'input self,
        component_manager: &'input mut ComponentManager,
    ) -> Option<&mut InputComponent> {
        let active_entity = self.get_active(component_manager)?;
        let active_input = component_manager.input_storage.get_mut(active_entity)?;

        Some(active_input)
    }

    fn set_next_entity_to_active(&self, component_manager: &mut ComponentManager) {
        let Some(active_input) = self.get_active_input_mut(component_manager) else {
            return;
        };

        let Some(next_entity) = active_input.next else {
            return;
        };

        active_input.is_active = false;

        let Some(next_input) = component_manager.input_storage.get_mut(next_entity) else {
            return;
        };

        next_input.is_active = true;
    }

    fn set_previous_entity_to_active(&self, component_manager: &mut ComponentManager) {
        let Some(active_input) = self.get_active_input_mut(component_manager) else {
            return;
        };

        let Some(previous_entity) = active_input.previous else {
            return;
        };

        active_input.is_active = false;

        let Some(previous_input) = component_manager.input_storage.get_mut(previous_entity) else {
            return;
        };

        previous_input.is_active = true;
    }
}
