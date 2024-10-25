mod texture;
use crate::{structs::ImageData, Window};
use ash::Device;
use image::DynamicImage;
use std::collections::HashMap;
use texture::Texture;

pub struct ResourceSystem {
    images: Vec<DynamicImage>,
    textures: Vec<ImageData>,
    texture_indices: HashMap<String, usize>,
}

impl ResourceSystem {
    pub fn create() -> Self {
        let (images, texture_indices) = {
            let texture_table: HashMap<String, Texture> =
                Texture::parse_table_from_json().expect("Failed to parse texture table!");
            Texture::load_images(&texture_table)
        };
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
