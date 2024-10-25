use crate::{
    constants::{TEXTURE_NOT_FOUND, TEXTURE_TABLE},
    read_bytes_from_file,
};
use image::DynamicImage;
use serde::Deserialize;
use serde_json::from_slice;
use std::{collections::HashMap, io::Result};

#[derive(Deserialize)]
pub struct Texture {
    pub path: String,
    pub sprite_size: u32,
}

impl Texture {
    pub fn parse_table_from_json() -> Result<HashMap<String, Texture>> {
        let table_bytes = read_bytes_from_file(TEXTURE_TABLE)?;
        let table: HashMap<String, Texture> = from_slice(&table_bytes)?;

        Ok(table)
    }

    pub fn load_images(
        texture_table: &HashMap<String, Texture>,
    ) -> (Vec<DynamicImage>, HashMap<String, usize>) {
        let mut images: Vec<DynamicImage> = Vec::new();
        let mut texture_indices: HashMap<String, usize> = HashMap::new();

        let mut next_id = 0;
        for (key, texture) in texture_table {
            if texture.sprite_size == 1 {
                images.push(Texture::open_image(&texture.path));
                texture_indices.insert(key.to_owned(), next_id);
                next_id += 1;
                continue;
            }

            let sprite_amount = texture.sprite_size * texture.sprite_size;
            images.extend(load_sprite_from_file(&texture.path, texture.sprite_size));
            for index in 0..sprite_amount {
                texture_indices.insert(key.to_owned() + &index.to_string(), next_id);
                next_id += 1;
            }
        }

        (images, texture_indices)
    }

    fn open_image(path: &str) -> DynamicImage {
        match image::open(path) {
            Ok(image) => image,
            Err(e) => {
                println!("{e}",);
                image::open(TEXTURE_NOT_FOUND).expect("Failed to open texture not found image!")
            }
        }
    }
}

fn load_sprite_from_file(texture_path: &str, sprite_size: u32) -> Vec<DynamicImage> {
    let texture_image = Texture::open_image(texture_path);
    let sprite_width = texture_image.width() / sprite_size;
    let sprite_height = texture_image.height() / sprite_size;

    crop_sprite(&texture_image, sprite_size, sprite_width, sprite_height)
}

fn crop_sprite(
    sprite_image: &DynamicImage,
    sprite_size: u32,
    sprite_width: u32,
    sprite_height: u32,
) -> Vec<DynamicImage> {
    let mut sprite_textures: Vec<DynamicImage> =
        Vec::with_capacity((sprite_size * sprite_size) as usize);

    // from left to right and then top to bottom
    for row in 0..sprite_size {
        for col in 0..sprite_size {
            sprite_textures.push(sprite_image.crop_imm(
                col * sprite_width,
                row * sprite_height,
                sprite_width,
                sprite_height,
            ));
        }
    }

    sprite_textures
}
