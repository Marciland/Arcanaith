mod vulkan;

use ash::{
    khr::surface,
    vk::{PhysicalDevice, Queue, SurfaceKHR},
    Device, Entry, Instance,
};
use ash_window::create_surface;
use vulkan::{VulkanInterface, VulkanWrapper};
use winit::raw_window_handle::{HasDisplayHandle, HasWindowHandle};

pub struct Window {
    _window: winit::window::Window,
    vk_instance: Instance,
    surface: SurfaceKHR,
    surface_loader: surface::Instance,
    _physical_device: PhysicalDevice,
    device: Device,
    _graphics_queue: Queue,
}

impl Window {
    pub unsafe fn new(window: winit::window::Window) -> Self {
        let entry = Entry::linked();
        let vk_instance = VulkanWrapper::create_vulkan_instance(&entry, &window);
        let surface = create_surface(
            &entry,
            &vk_instance,
            window.display_handle().unwrap().as_raw(),
            window.window_handle().unwrap().as_raw(),
            None,
        )
        .unwrap();
        let surface_loader = surface::Instance::new(&entry, &vk_instance);
        let (physical_device, index) =
            VulkanWrapper::find_physical_device(&vk_instance, &surface, &surface_loader);
        let queue_family_index = index as u32;
        let device =
            VulkanWrapper::create_logical_device(&vk_instance, physical_device, queue_family_index);
        let graphics_queue = device.get_device_queue(queue_family_index, 0);

        Self {
            _window: window,
            vk_instance,
            surface,
            surface_loader,
            _physical_device: physical_device,
            device,
            _graphics_queue: graphics_queue,
        }
    }

    pub unsafe fn destroy(&mut self) {
        self.surface_loader.destroy_surface(self.surface, None);
        self.device.destroy_device(None);
        self.vk_instance.destroy_instance(None);
    }
}
