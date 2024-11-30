use image::DynamicImage;
use serde::Deserialize;
use serde_json::from_slice;
use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Result},
};

#[derive(Deserialize)]
struct Texture {
    name: String,
    path: String,
}

#[derive(Deserialize)]
struct Sprite {
    name: String,
    path: String,
    sprite_size: u32,
}

#[derive(Deserialize)]
pub(crate) struct TextureTable {
    textures: Vec<Texture>,
    sprites: Vec<Sprite>,
}

impl TextureTable {
    pub fn from_json(file_path: &str) -> Self {
        let table_bytes = read_bytes_from_file(file_path)
            .expect(&("Failed to read bytes from ".to_string() + file_path));
        from_slice(&table_bytes).expect(&("Failed to parse ".to_string() + file_path))
    }

    pub fn load_images(&self) -> (Vec<DynamicImage>, HashMap<String, usize>) {
        // estimating that sprites consist of ~4 images, therefore reducing allocations
        let estimated_amount = self.textures.len() + self.sprites.len() * 4;
        let mut images: Vec<DynamicImage> = Vec::with_capacity(estimated_amount);
        let mut texture_indices: HashMap<String, usize> = HashMap::with_capacity(estimated_amount);

        let mut next_id = 0;

        for texture in &self.textures {
            images.push(open_image(&texture.path));
            texture_indices.insert(texture.name.clone(), next_id);
            next_id += 1;
        }

        for sprite in &self.sprites {
            images.extend(load_sprite_from_file(&sprite.path, sprite.sprite_size));
            for index in 0..sprite.sprite_size * sprite.sprite_size {
                texture_indices.insert(sprite.name.clone() + "_" + &index.to_string(), next_id);
                next_id += 1;
            }
        }

        (images, texture_indices)
    }
}

fn load_sprite_from_file(texture_path: &str, sprite_size: u32) -> Vec<DynamicImage> {
    let texture_image = open_image(texture_path);
    let sprite_width = texture_image.width() / sprite_size;
    let sprite_height = texture_image.height() / sprite_size;

    crop_sprite(&texture_image, sprite_size, sprite_width, sprite_height)
}

fn open_image(path: &str) -> DynamicImage {
    match image::open(path) {
        Ok(image) => image,
        Err(e) => {
            println!("{e}",);
            image::open("res/404.png").expect("Failed to open texture not found image!")
        }
    }
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

fn read_bytes_from_file(path: &str) -> Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}
