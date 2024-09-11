mod constants;
mod game;
mod scene;
mod shader_structs;
mod vulkan;
mod window;

use std::{fs::File, io::Read};

pub use game::Game;

fn read_bytes_from_file(path: &str) -> Vec<u8> {
    let mut file = File::open(path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    buffer
}
