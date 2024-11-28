mod main_menu;
mod settings_menu;

pub use main_menu::MainMenu;
pub use settings_menu::SettingsMenu;

use crate::{
    ecs::{
        component::{composition::InputWithPosition, ComponentManager, InputComponent, Layer},
        entity::Entity,
        system::input::{InputHandler, InputSystem, MouseHandler},
    },
    objects::{Content, Label, Object},
    GameEvent, MouseEvent, ECS,
};
use ash::Device;
use glam::{Vec2, Vec3};
use indexmap::IndexSet;
use winit::{
    event::MouseButton,
    event_loop::EventLoopProxy,
    keyboard::{Key, NamedKey},
};

pub enum Menu {
    MainMenu(MainMenu),
    SettingsMenu(SettingsMenu),
}

impl Menu {
    pub fn create_background(ecs: &mut ECS) -> Label {
        ecs.new_label(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.1,
            },
            Vec2 { x: 2.0, y: 2.0 },
            Content::Image {
                name: "menu_background",
                layer: Layer::Background,
            },
        )
    }

    pub fn create_title(ecs: &mut ECS) -> Label {
        ecs.new_label(
            Vec2 { x: 0.0, y: -0.8 },
            Vec2 { x: 1.5, y: 0.5 },
            Content::Image {
                name: "menu_title",
                layer: Layer::Background,
            },
        )
    }

    pub fn get_active(&self, component_manager: &ComponentManager) -> Option<Entity> {
        let objects = match self {
            Menu::MainMenu(main_menu) => &main_menu.objects,
            Menu::SettingsMenu(settings_menu) => &settings_menu.objects,
        };

        for obj in objects {
            let Some(input) = component_manager.input_storage.get(obj.id()) else {
                continue;
            };

            if input.is_active {
                return Some(obj.id());
            }
        }

        None
    }

    pub fn get_objects(&self) -> &[Object] {
        match self {
            Menu::MainMenu(main_menu) => &main_menu.objects,
            Menu::SettingsMenu(settings_menu) => &settings_menu.objects,
        }
    }

    pub fn destroy(&self, device: &Device, ecs: &mut ECS) {
        let objects = match self {
            Menu::MainMenu(main_menu) => &main_menu.objects,
            Menu::SettingsMenu(settings_menu) => &settings_menu.objects,
        };

        for obj in objects {
            ecs.destroy_entity(obj.id(), device);
        }
    }

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
