mod loading;
mod sprites;
use crate::{structs::ImageData, Window};
use ash::Device;
use image::DynamicImage;
use loading::ResourceLoader;
use std::collections::HashMap;

pub struct ResourceSystem {
    images: Vec<DynamicImage>,
    textures: Vec<ImageData>,
    texture_indices: HashMap<String, usize>,
}

impl ResourceSystem {
    pub fn create() -> Self {
        let mut texture_indices = HashMap::new();
        let images = ResourceLoader::load_all_images(&mut texture_indices);
        let textures = Vec::with_capacity(images.len());

        Self {
            images,
            textures,
            texture_indices,
        }
    }

    pub fn initialize(&mut self, window: &Window) {
        for image in &self.images {
            self.textures
                .push(window.create_texture(image.clone().into_rgba8()));
        }
    }

    pub fn get_texture_count(&self) -> u32 {
        self.images.len() as u32
    }

    pub fn get_texture_index(&self, key: &str) -> usize {
        *self
            .texture_indices
            .get(key)
            .expect(&("Failed to get texture index: ".to_string() + key))
    }

    pub fn get_texture(&self, texture_index: usize) -> &ImageData {
        self.textures
            .get(texture_index)
            .expect(&("Failed to get texture: ".to_string() + &texture_index.to_string()))
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn destroy(&self, device: &Device) {
        for texture in &self.textures {
            texture.destroy(device);
        }
    }
}
