use crate::{vulkan::ImageData, Window};
use image::DynamicImage;
use std::cell::Ref;

pub fn load_sprite_from_file(
    window: Ref<Window>,
    sprite_file: &str,
    cols: u32,
    rows: u32,
) -> Vec<ImageData> {
    let sprite = image::open(sprite_file).unwrap();
    let sprite_width = sprite.width() / cols;
    let sprite_height = sprite.height() / rows;

    load_sprite(window, sprite, cols, rows, sprite_width, sprite_height)
}

fn load_sprite(
    window: Ref<Window>,
    sprite_image: DynamicImage,
    cols: u32,
    rows: u32,
    sprite_width: u32,
    sprite_height: u32,
) -> Vec<ImageData> {
    let mut sprite_textures: Vec<ImageData> = Vec::with_capacity((cols * rows) as usize);

    for col in 0..cols {
        for row in 0..rows {
            sprite_textures.push(create_sprite_texture(
                &window,
                &sprite_image,
                col,
                row,
                sprite_width,
                sprite_height,
            ))
        }
    }

    sprite_textures
}

fn create_sprite_texture(
    window: &Ref<Window>,
    sprite_image: &DynamicImage,
    col: u32,
    row: u32,
    sprite_width: u32,
    sprite_height: u32,
) -> ImageData {
    window.create_texture(
        sprite_image
            .crop_imm(
                col * sprite_width,
                row * sprite_height,
                sprite_width,
                sprite_height,
            )
            .into_rgba8(),
    )
}
