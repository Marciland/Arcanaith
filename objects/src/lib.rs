mod factory;

use ecs::{Layer, TextContent};

pub use factory::Factory;

pub enum Content<'a> {
    Text(TextContent),
    Image { name: &'a str, layer: Layer },
}
