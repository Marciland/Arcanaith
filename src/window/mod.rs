mod vulkan;

use ash::{vk::PhysicalDevice, Device, Entry, Instance};
use vulkan::{VulkanInterface, VulkanWrapper};

pub struct Window {
    _window: winit::window::Window,
    vk_instance: Instance,
    _physical_device: PhysicalDevice,
    device: Device,
}

impl Window {
    pub unsafe fn new(window: winit::window::Window) -> Self {
        let entry = Entry::linked();
        let vk_instance = VulkanWrapper::create_vulkan_instance(&entry, &window);
        let (physical_device, index) = VulkanWrapper::find_physical_device(&vk_instance);
        let device =
            VulkanWrapper::create_logical_device(&vk_instance, physical_device, index as u32);

        Self {
            _window: window,
            vk_instance,
            _physical_device: physical_device,
            device,
        }
    }

    pub unsafe fn destroy(&mut self) {
        self.device.destroy_device(None);
        self.vk_instance.destroy_instance(None);
    }
}
