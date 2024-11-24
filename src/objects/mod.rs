mod button;
mod icon_text;
mod label;
mod player;
mod quad;
mod status_bar;

use crate::ecs::{component::Layer, entity::Entity};

pub use button::Button;
pub use icon_text::IconText;
pub use label::Label;
pub use player::Player;
pub use quad::Quad;
pub use status_bar::StatusBar;

pub enum Object {
    Button(Button),
    Label(Label),
    IconText(IconText),
    StatusBar(StatusBar),
    Player(Player),
}

impl Object {
    pub fn id(&self) -> Entity {
        match self {
            Object::Button(b) => b.id,
            Object::Label(l) => l.id,
            Object::IconText(it) => it.id,
            Object::StatusBar(sb) => sb.id,
            Object::Player(p) => p.id,
        }
    }
}

pub enum Content<'a> {
    Text(TextContent),
    Image { name: &'a str, layer: Layer },
}

pub struct TextContent {
    pub text: String,
    pub font: String,
    pub font_size: f32,
}
