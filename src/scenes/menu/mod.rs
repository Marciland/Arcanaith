mod main_menu;
mod settings_menu;

pub use main_menu::MainMenu;
pub use settings_menu::SettingsMenu;

use crate::{
    ecs::{
        component::{ComponentManager, Layer},
        entity::Entity,
    },
    objects::{Content, Label, Object},
    ECS,
};
use ash::Device;
use glam::{Vec2, Vec3};

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
}
