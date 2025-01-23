mod api;
mod context;
mod window;

pub use api::{RenderAPI, VulkanAPI};
pub use ash::vk::ImageView;
pub use context::RenderContext;
pub use vulkan::structs::{ImageData, MVP};
pub use window::WindowSize;

use image::{ImageBuffer, Rgba};

pub trait Renderer {
    fn draw(&mut self, textures: &[ImageView], positions: &[MVP]);
    fn create_image_data(&self, image: ImageBuffer<Rgba<u8>, Vec<u8>>) -> ImageData;
}
