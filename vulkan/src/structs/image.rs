use ash::{
    vk::{DeviceMemory, Image, ImageView},
    Device,
};

pub struct ImageData {
    image: Image,
    memory: DeviceMemory,
    view: ImageView,
}

impl ImageData {
    #[must_use]
    pub fn create(image: Image, memory: DeviceMemory, view: ImageView) -> Self {
        Self {
            image,
            memory,
            view,
        }
    }

    #[must_use]
    pub fn get_view(&self) -> ImageView {
        self.view
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn destroy(&self, device: &Device) {
        device.destroy_image_view(self.view, None);
        device.destroy_image(self.image, None);
        device.free_memory(self.memory, None);
    }
}
