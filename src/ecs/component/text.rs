// TODO remove crate dependencies
use crate::{objects::TextContent, structs::ImageData};

use super::{ComponentStorage, Entity, Layer};
use ash::Device;

pub struct TextComponent {
    pub content: TextContent,
    pub bitmap: Option<ImageData>,
    pub layer: Layer,
}

impl ComponentStorage<TextComponent> {
    pub fn destroy(&mut self, device: &Device) {
        for component in self.components.values_mut() {
            component.destroy(device);
        }
    }

    pub fn destroy_entity(&mut self, entity: Entity, device: &Device) {
        if let Some(mut component) = self.components.remove(&entity) {
            component.destroy(device);
        }
    }
}

impl TextComponent {
    pub fn create(content: TextContent) -> Self {
        Self {
            content,
            bitmap: None,
            layer: Layer::Interface,
        }
    }

    pub fn destroy(&mut self, device: &Device) {
        if let Some(image_data) = self.bitmap.take() {
            unsafe { image_data.destroy(device) }
        }
    }
}
