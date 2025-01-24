mod vulkan;

use crate::{ImageData, WindowSize};

use ::vulkan::structs::MVP;
use ash::vk::ImageView;
use winit::window::Window;

pub use vulkan::VulkanAPI;

pub trait RenderAPI {
    fn create(
        window: &Window,
        max_texture_count: u32,
        title: &str,
        frames_in_flight: usize,
        vertex_path: &str,
        frag_path: &str,
    ) -> Self;
    fn draw(&mut self, textures: &[ImageView], positions: &[MVP]);
    fn create_image_data(&self, image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>) -> ImageData;
    fn get_extent(&self) -> WindowSize;
    fn wait_idle(&self);
    #[allow(clippy::missing_safety_doc)]
    unsafe fn destroy(&self);
}
