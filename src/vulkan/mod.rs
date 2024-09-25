mod internal;
mod shader_structs;
mod wrapper;
use ash::{
    vk::{DeviceMemory, Image, ImageView},
    Device,
};
pub use shader_structs::{ModelViewProjection, UniformBufferObject, Vertex};
pub use wrapper::Wrapper;
pub struct VulkanWrapper;

pub struct ImageData {
    image: Image,
    memory: DeviceMemory,
    view: ImageView,
}

impl ImageData {
    pub fn get_view(&self) -> ImageView {
        self.view
    }

    pub unsafe fn destroy(&self, device: &Device) {
        device.destroy_image_view(self.view, None);
        device.destroy_image(self.image, None);
        device.free_memory(self.memory, None);
    }
}
