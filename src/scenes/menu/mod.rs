mod main_menu;
mod settings_menu;

pub use main_menu::MainMenu;
pub use settings_menu::SettingsMenu;

use crate::{
    ecs::component::Layer,
    objects::{Label, LabelContent, ObjectFactory},
};
use glam::{Vec2, Vec3};

pub struct Menu;

impl Menu {
    pub fn create_background(factory: &mut ObjectFactory) -> Label {
        factory.new_label(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.1,
            },
            Vec2 { x: 2.0, y: 2.0 },
            LabelContent::Image {
                name: "menu_background",
                layer: Layer::Background,
            },
        )
    }

    pub fn create_title(factory: &mut ObjectFactory) -> Label {
        factory.new_label(
            Vec2 { x: 0.0, y: -0.8 },
            Vec2 { x: 1.5, y: 0.5 },
            LabelContent::Image {
                name: "menu_title",
                layer: Layer::Background,
            },
        )
    }
}
