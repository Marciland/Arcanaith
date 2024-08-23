mod vulkan;

use ash::{Entry, Instance};
use vulkan::create_vulkan_instance;

pub struct Window {
    window: winit::window::Window,
    vk_instance: Instance,
}

impl Window {
    pub fn new(window: winit::window::Window) -> Self {
        let entry = Entry::linked();
        let vk_instance = unsafe { create_vulkan_instance(&entry, &window, false) };

        Self {
            window,
            vk_instance,
        }
    }

    pub fn destroy(&mut self) {
        unsafe { self.vk_instance.destroy_instance(None) }
    }
}
