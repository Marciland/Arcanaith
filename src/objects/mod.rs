mod button;
mod label;
mod quad;

use crate::ecs::{component::ComponentManager, entity::EntityManager, system::SystemManager};
pub use button::Button;
pub use label::{Label, LabelContent};
pub use quad::Quad;

pub struct TextContent<'a> {
    pub text: &'a str,
    pub font: &'a str,
    pub font_size: f32,
}

pub struct ObjectFactory<'building> {
    pub entity_manager: &'building mut EntityManager,
    pub component_manager: &'building mut ComponentManager,
    pub system_manager: &'building SystemManager,
}
