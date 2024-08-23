mod vulkan;

use ash::{vk::PhysicalDevice, Entry, Instance};
use vulkan::{create_vulkan_instance, find_physical_device};

pub struct Window {
    _window: winit::window::Window,
    vk_instance: Instance,
    _physical_device: PhysicalDevice,
}

impl Window {
    pub fn new(window: winit::window::Window) -> Self {
        let entry = Entry::linked();
        let vk_instance = unsafe { create_vulkan_instance(&entry, &window) };
        let physical_device = unsafe { find_physical_device(&vk_instance) };

        // https://docs.vulkan.org/tutorial/latest/03_Drawing_a_triangle/00_Setup/04_Logical_device_and_queues.html
        Self {
            _window: window,
            vk_instance,
            _physical_device: physical_device,
        }
    }

    pub fn destroy(&mut self) {
        unsafe { self.vk_instance.destroy_instance(None) }
    }
}
