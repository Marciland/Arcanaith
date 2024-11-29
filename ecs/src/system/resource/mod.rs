mod font;
mod text;
mod texture;

use ab_glyph::FontVec;
use ash::{vk::ImageView, Device};
use image::DynamicImage;
use std::{collections::HashMap, path::PathBuf};
use texture::TextureTable;

use crate::component::{ImageData, TextComponent};

use super::render::RenderContext;

pub(crate) struct ResourceSystem {
    font_base_path: PathBuf,
    images: Vec<DynamicImage>,
    fonts: HashMap<String, FontVec>,
    textures: Vec<ImageData>,
    texture_indices: HashMap<String, usize>, // combine?
}

impl ResourceSystem {
    pub fn create(texture_path: &str, font_path: &str) -> Self {
        let font_base_path = PathBuf::from(font_path);

        let texture_table = TextureTable::from_json(texture_path);
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

    pub fn initialize<R>(&mut self, renderer: &R)
    where
        R: RenderContext,
    {
        for image in &self.images {
            self.textures
                .push(renderer.create_image_data(image.clone().into_rgba8()));
        }

        self.fonts = self.create_font_map();
    }

    fn get_texture_count(&self) -> u32 {
        self.images.len() as u32
    }

    fn get_texture_index(&self, key: &str) -> usize {
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

    fn get_font(&self, font: &str) -> &FontVec {
        self.fonts
            .get(font)
            .expect(&("Failed to get font: ".to_string() + font))
    }

    pub fn get_bitmap<R>(&mut self, renderer: &R, component: &mut TextComponent) -> ImageView
    where
        R: RenderContext,
    {
        let Some(bitmap) = &component.bitmap else {
            return self.create_bitmap(renderer, component);
        };

        bitmap.get_view()
    }

    fn create_bitmap<R>(&mut self, renderer: &R, component: &mut TextComponent) -> ImageView
    where
        R: RenderContext,
    {
        let image = self.text_to_image(&component.content);

        let bitmap = renderer.create_image_data(image);
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
