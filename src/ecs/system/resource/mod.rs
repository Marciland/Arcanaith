mod assets;
mod font;
use crate::{constants::TEXTURE_TABLE, structs::ImageData, Window};
use ab_glyph::FontVec;
use ash::Device;
use assets::TextureTable;
use font::create_font_map;
use image::DynamicImage;
use std::collections::HashMap;

pub struct ResourceSystem {
    images: Vec<DynamicImage>,
    fonts: HashMap<String, FontVec>,
    textures: Vec<ImageData>,
    texture_indices: HashMap<String, usize>,
}

impl ResourceSystem {
    pub fn create() -> Self {
        let texture_table = TextureTable::from_json(TEXTURE_TABLE);
        let (images, texture_indices) = texture_table.load_images();
        let textures = Vec::with_capacity(images.len());
        let fonts = create_font_map();

        Self {
            images,
            fonts,
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

    pub fn get_font(&self, font: &str) -> &FontVec {
        self.fonts
            .get(font)
            .expect(&("Failed to get font: ".to_string() + font))
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn destroy(&self, device: &Device) {
        for texture in &self.textures {
            texture.destroy(device);
        }
    }
}
