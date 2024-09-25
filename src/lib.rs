mod constants;
mod game;
mod vulkan;
mod window;
pub use game::Game;
use game::Scene;
use std::{fs::File, io::Read};
use vulkan::{ModelViewProjection, UniformBufferObject, Vertex, VulkanWrapper};
use window::Window;

fn read_bytes_from_file(path: &str) -> Vec<u8> {
    let mut file = File::open(path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    buffer
}
