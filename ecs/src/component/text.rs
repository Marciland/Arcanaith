use super::{ComponentStorage, Entity, Layer};

use rendering::ImageData;

pub struct TextContent {
    pub text: String,
    pub font: String,
    pub font_size: f32,
}

pub struct TextComponent {
    pub content: TextContent,
    pub bitmap: Option<ImageData>,
    pub layer: Layer,
}

impl ComponentStorage<TextComponent> {
    pub fn destroy(&mut self) {
        for component in self.components.values_mut() {
            component.destroy();
        }
    }

    pub fn destroy_entity(&mut self, entity: Entity) {
        if let Some(mut component) = self.components.remove(&entity) {
            component.destroy();
        }
    }
}

impl TextComponent {
    #[must_use]
    pub fn create(content: TextContent) -> Self {
        Self {
            content,
            bitmap: None,
            layer: Layer::Interface,
        }
    }

    fn destroy(&mut self) {
        if let Some(image_data) = self.bitmap.take() {
            image_data.destroy();
        }
    }
}
