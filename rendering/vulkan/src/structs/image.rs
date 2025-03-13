use ash::{
    vk::{DeviceMemory, Image, ImageView},
    Device,
};
use std::rc::Rc;

pub struct ImageData {
    image: Image,
    memory: DeviceMemory,
    view: ImageView,
    device: Rc<Device>,
}

impl ImageData {
    #[must_use]
    pub fn create(image: Image, memory: DeviceMemory, view: ImageView, device: Rc<Device>) -> Self {
        Self {
            image,
            memory,
            view,
            device,
        }
    }

    #[must_use]
    pub fn get_view(&self) -> ImageView {
        self.view
    }

    pub fn destroy(&self) {
        unsafe {
            self.device.destroy_image_view(self.view, None);
            self.device.destroy_image(self.image, None);
            self.device.free_memory(self.memory, None);
        }
    }
}
