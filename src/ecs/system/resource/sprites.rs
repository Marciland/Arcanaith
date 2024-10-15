use image::DynamicImage;

pub fn load_sprite_from_file(sprite_file: &str, cols: u32, rows: u32) -> Vec<DynamicImage> {
    let sprite = image::open(sprite_file)
        .expect(&("Failed to open sprite image: ".to_string() + sprite_file));
    let sprite_width = sprite.width() / cols;
    let sprite_height = sprite.height() / rows;

    load_sprite(sprite, cols, rows, sprite_width, sprite_height)
}

fn load_sprite(
    sprite_image: DynamicImage,
    cols: u32,
    rows: u32,
    sprite_width: u32,
    sprite_height: u32,
) -> Vec<DynamicImage> {
    let mut sprite_textures: Vec<DynamicImage> = Vec::with_capacity((cols * rows) as usize);

    // from left to right and then top to bottom
    for row in 0..rows {
        for col in 0..cols {
            sprite_textures.push(sprite_image.crop_imm(
                col * sprite_width,
                row * sprite_height,
                sprite_width,
                sprite_height,
            ))
        }
    }

    sprite_textures
}
