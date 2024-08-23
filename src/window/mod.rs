mod vulkan;

use ash::{
    khr::{surface, swapchain},
    vk::{Extent2D, Format, Image, PhysicalDevice, Queue, SurfaceKHR, SwapchainKHR},
    Device, Entry, Instance,
};
use vulkan::{VulkanInterface, VulkanWrapper};

pub struct Window {
    _window: winit::window::Window,
    vk_instance: Instance,
    surface: SurfaceKHR,
    surface_loader: surface::Instance,
    _physical_device: PhysicalDevice,
    device: Device,
    _graphics_queue: Queue,
    swapchain: SwapchainKHR,
    swapchain_loader: swapchain::Device,
    _images: Vec<Image>,
    _format: Format,
    _extent: Extent2D,
}

impl Window {
    pub unsafe fn new(window: winit::window::Window) -> Self {
        let entry = Entry::linked();
        let vk_instance = VulkanWrapper::create_vulkan_instance(&entry, &window);
        let (surface, surface_loader) =
            VulkanWrapper::create_surface(&window, &entry, &vk_instance);
        let (physical_device, queue_family_index) =
            VulkanWrapper::find_physical_device(&vk_instance, &surface, &surface_loader);
        let device =
            VulkanWrapper::create_logical_device(&vk_instance, physical_device, queue_family_index);
        let graphics_queue = device.get_device_queue(queue_family_index, 0);
        let (swapchain, swapchain_loader, format, extent) = VulkanWrapper::create_swapchain(
            &vk_instance,
            surface,
            &device,
            physical_device,
            &surface_loader,
        );
        let images = swapchain_loader.get_swapchain_images(swapchain).unwrap();

        Self {
            _window: window,
            vk_instance,
            surface,
            surface_loader,
            _physical_device: physical_device,
            device,
            _graphics_queue: graphics_queue,
            swapchain,
            swapchain_loader,
            _images: images,
            _format: format,
            _extent: extent,
        }
    }

    pub unsafe fn destroy(&mut self) {
        self.swapchain_loader
            .destroy_swapchain(self.swapchain, None);
        self.surface_loader.destroy_surface(self.surface, None);
        self.device.destroy_device(None);
        self.vk_instance.destroy_instance(None);
    }
}
