mod main_menu;
mod settings_menu;

use crate::GameEvent;

use ash::Device;
use ecs::{Entity, InputHandler, Layer, MouseEvent, ECS};
use glam::{Vec2, Vec3};
use indexmap::IndexSet;
use objects::{Content, Factory};
use winit::{
    event::MouseButton,
    event_loop::EventLoopProxy,
    keyboard::{Key, NamedKey},
};

pub use main_menu::MainMenu;
pub use settings_menu::SettingsMenu;

pub enum Menu {
    MainMenu(MainMenu),
    SettingsMenu(SettingsMenu),
}

impl Menu {
    pub fn create_background(ecs: &mut ECS<GameEvent>) -> Entity {
        Factory::label(
            ecs,
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

    pub fn create_title(ecs: &mut ECS<GameEvent>) -> Entity {
        Factory::label(
            ecs,
            Vec2 { x: 0.0, y: -0.8 },
            Vec2 { x: 1.5, y: 0.5 },
            Content::Image {
                name: "menu_title",
                layer: Layer::Background,
            },
        )
    }

    pub fn get_objects(&self) -> &[Entity] {
        match self {
            Menu::MainMenu(main_menu) => &main_menu.objects,
            Menu::SettingsMenu(settings_menu) => &settings_menu.objects,
        }
    }

    pub fn destroy(&self, device: &Device, ecs: &mut ECS<GameEvent>) {
        let objects = match self {
            Menu::MainMenu(main_menu) => &main_menu.objects,
            Menu::SettingsMenu(settings_menu) => &settings_menu.objects,
        };

        for obj in objects {
            ecs.destroy_entity(*obj, device);
        }
    }
}

impl InputHandler<GameEvent> for Menu {
    fn handle_mouse_events(
        &self,
        ecs: &ECS<GameEvent>,
        events: &[MouseEvent],
        event_proxy: &EventLoopProxy<GameEvent>,
    ) {
        for event in events {
            if event.button.mouse_button != MouseButton::Left {
                continue;
            }

            for entity in self.get_objects() {
                if ecs.position_matches_entity(&event.position, *entity) {
                    return ecs.activate_entity(entity, event_proxy);
                }
            }
        }
    }

    fn handle_key_events(
        &self,
        ecs: &mut ECS<GameEvent>,
        pressed_keys: &IndexSet<Key>,
        event_proxy: &EventLoopProxy<GameEvent>,
    ) {
        for key in pressed_keys {
            match key {
                Key::Named(NamedKey::Tab | NamedKey::ArrowDown | NamedKey::ArrowRight) => {
                    if let Some(currently_active) = ecs.get_active_entity() {
                        ecs.set_next_active(*currently_active);
                    }
                }

                Key::Named(NamedKey::ArrowLeft | NamedKey::ArrowUp) => {
                    if let Some(currently_active) = ecs.get_active_entity() {
                        ecs.set_previous_active(*currently_active);
                    }
                }

                Key::Named(NamedKey::Space | NamedKey::Enter) => {
                    if let Some(active_entity) = ecs.get_active_entity() {
                        ecs.activate_entity(active_entity, event_proxy);
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
