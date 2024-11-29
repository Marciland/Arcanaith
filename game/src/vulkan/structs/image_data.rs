use ash::vk::{DeviceMemory, Image, ImageView};

pub struct ImageData {
    image: Image,
    memory: DeviceMemory,
    view: ImageView,
}

impl ImageData {
    pub fn create(image: Image, memory: DeviceMemory, view: ImageView) -> Self {
        Self {
            image,
            memory,
            view,
        }
    }
}
