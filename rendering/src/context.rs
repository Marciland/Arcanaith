use super::{RenderAPI, Renderer, WindowSize};

use ash::vk::ImageView;
use image::{ImageBuffer, Rgba};
use vulkan::structs::{ImageData, MVP};
use winit::window::Window;

pub struct RenderContext<API: RenderAPI> {
    api: API,
}

impl<API: RenderAPI> RenderContext<API> {
    pub fn create(
        window: &Window,
        max_texture_count: u32,
        title: &str,
        in_flight: usize,
        vertex_path: &str,
        frag_path: &str,
    ) -> Self {
        Self {
            api: API::create(
                window,
                max_texture_count,
                title,
                in_flight,
                vertex_path,
                frag_path,
            ),
        }
    }

    pub fn get_extent(&self) -> WindowSize {
        self.api.get_extent()
    }

    pub fn wait_idle(&self) {
        self.api.wait_idle()
    }

    pub fn destroy(&self) {
        unsafe { self.api.destroy() };
    }
}

impl<API: RenderAPI> Renderer for RenderContext<API> {
    fn draw(&mut self, textures: &[ImageView], positions: &[MVP]) {
        self.api.draw(textures, positions);
    }

    fn create_image_data(&self, image: ImageBuffer<Rgba<u8>, Vec<u8>>) -> ImageData {
        self.api.create_image_data(image)
    }
}
