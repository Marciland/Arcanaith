mod constants;
mod ecs;
mod game;
mod vulkan;
mod window;
pub use game::Game;
use std::{fs::File, io::Read};
use vulkan::structs;
use window::Window;

fn read_bytes_from_file(path: &str) -> Vec<u8> {
    let mut file = File::open(path).expect(&("Failed to open file: ".to_string() + path));
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .expect(&("Could not read file: ".to_string() + path));
    buffer
}
