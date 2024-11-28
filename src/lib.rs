mod constants;
mod ecs;
mod game;
mod objects;
mod scenes;
mod vulkan;
mod window;

pub use ecs::system::input::MouseEvent; // TODO deconstruct
pub use ecs::ECS;
pub use game::{Game, GameEvent};

use std::{
    fs::File,
    io::{Read, Result},
};
use vulkan::structs;
use window::Window;

fn read_bytes_from_file(path: &str) -> Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}
