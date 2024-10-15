use super::sprites::load_sprite_from_file;
use crate::constants::{MAIN_MENU, SPRITES};
use image::DynamicImage;
use std::{collections::HashMap, path::Path};

pub struct ResourceLoader;

impl ResourceLoader {
    pub fn load_all_images(texture_indices: &mut HashMap<String, usize>) -> Vec<DynamicImage> {
        let sprite_path = Path::new(SPRITES);
        let mut all_images: Vec<DynamicImage> = Vec::new();

        let main_menu_path = Path::new(MAIN_MENU);
        let main_menu_images = ResourceLoader::load_main_menu(main_menu_path, texture_indices);
        all_images.extend(main_menu_images);

        let player_images = ResourceLoader::load_player_images(sprite_path, texture_indices);
        all_images.extend(player_images);

        all_images
    }

    fn load_main_menu(
        base_path: &Path,
        texture_indices: &mut HashMap<String, usize>,
    ) -> Vec<DynamicImage> {
        let background = image::open(
            base_path
                .join("background.png")
                .to_str()
                .expect("Path is not valid unicode!"),
        )
        .expect("Failed to open main menu background image!");
        texture_indices.insert("main_menu_background".to_string(), 0);

        vec![background]
    }

    fn load_player_images(
        base_path: &Path,
        texture_indices: &mut HashMap<String, usize>,
    ) -> Vec<DynamicImage> {
        let mut player_images: Vec<DynamicImage> = Vec::new();

        let player_walking = load_sprite_from_file(
            base_path
                .join("player_walking.png")
                .to_str()
                .expect("Path is not valid unicode!"),
            4,
            4,
        );
        texture_indices.insert("player_front".to_string(), 50);
        texture_indices.insert("player_front_walking_1".to_string(), 51);
        texture_indices.insert("player_front_walking_2".to_string(), 52);
        texture_indices.insert("player_front_walking_3".to_string(), 53);

        texture_indices.insert("player_back".to_string(), 54);
        texture_indices.insert("player_back_walking_1".to_string(), 55);
        texture_indices.insert("player_back_walking_2".to_string(), 56);
        texture_indices.insert("player_back_walking_3".to_string(), 57);

        texture_indices.insert("player_left".to_string(), 58);
        texture_indices.insert("player_left_walking_1".to_string(), 59);
        texture_indices.insert("player_left_walking_2".to_string(), 60);
        texture_indices.insert("player_left_walking_3".to_string(), 61);

        texture_indices.insert("player_right".to_string(), 62);
        texture_indices.insert("player_right_walking_1".to_string(), 63);
        texture_indices.insert("player_right_walking_2".to_string(), 64);
        texture_indices.insert("player_right_walking_3".to_string(), 65);

        player_images.extend(player_walking);

        player_images
    }
}
