mod font;
mod text;
mod texture;

use crate::{constants::TEXTURE_TABLE, ecs::component::TextComponent, structs::ImageData, Window};
use ab_glyph::FontVec;
use ash::{vk::ImageView, Device};
use image::DynamicImage;
use std::{collections::HashMap, path::PathBuf};
use texture::TextureTable;

pub struct ResourceSystem {
    font_base_path: PathBuf,
    images: Vec<DynamicImage>,
    fonts: HashMap<String, FontVec>,
    textures: Vec<ImageData>,
    texture_indices: HashMap<String, usize>, // combine?
}

impl ResourceSystem {
    pub fn create(font_path: &str) -> Self {
        let font_base_path = PathBuf::from(font_path);

        let texture_table = TextureTable::from_json(TEXTURE_TABLE);
        let (images, texture_indices) = texture_table.load_images();
        let textures = Vec::with_capacity(images.len());

        Self {
            font_base_path,
            images,
            fonts: HashMap::with_capacity(5),
            textures,
            texture_indices,
        }
    }

    pub fn initialize(&mut self, window: &Window) {
        for image in &self.images {
            self.textures
                .push(window.create_image_data(image.clone().into_rgba8()));
        }

        self.fonts = self.create_font_map();
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

    pub fn get_bitmap(&mut self, window: &Window, component: &mut TextComponent) -> ImageView {
        let Some(bitmap) = &component.bitmap else {
            return self.create_bitmap(window, component);
        };

        bitmap.get_view()
    }

    fn create_bitmap(&mut self, window: &Window, component: &mut TextComponent) -> ImageView {
        let image = text::to_image(
            &component.content.text,
            self.get_font(&component.content.font),
            component.content.font_size,
        );

        let bitmap = window.create_image_data(image);
        let view = bitmap.get_view();

        component.bitmap = Some(bitmap);

        view
    }

    pub fn destroy(&self, device: &Device) {
        for texture in &self.textures {
            unsafe {
                texture.destroy(device);
            }
        }
    }
}
