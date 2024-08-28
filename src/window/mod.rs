mod vulkan;

use ash::{
    khr::{surface, swapchain},
    vk::{
        CommandBuffer, CommandPool, Extent2D, Format, Framebuffer, Image, ImageView,
        PhysicalDevice, Pipeline, PipelineLayout, Queue, RenderPass, SurfaceKHR, SwapchainKHR,
    },
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
    swapchain_framebuffers: Vec<Framebuffer>,
    _images: Vec<Image>,
    image_views: Vec<ImageView>,
    _format: Format,
    _extent: Extent2D,
    render_pass: RenderPass,
    pipeline_layout: PipelineLayout,
    graphics_pipeline: Pipeline,
    command_pool: CommandPool,
    _command_buffer: CommandBuffer,
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
        let image_views = VulkanWrapper::create_image_views(&images, format, &device);
        let render_pass = VulkanWrapper::create_render_pass(&device, format);
        let (pipeline_layout, graphics_pipeline) =
            VulkanWrapper::create_graphics_pipeline(&device, extent, render_pass);
        let swapchain_framebuffers =
            VulkanWrapper::create_framebuffers(&device, render_pass, &image_views, extent);
        let command_pool = VulkanWrapper::create_command_pool(&device, queue_family_index);
        let command_buffer = VulkanWrapper::create_command_buffer(&device, command_pool);

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
            swapchain_framebuffers,
            _images: images,
            image_views,
            _format: format,
            _extent: extent,
            render_pass,
            pipeline_layout,
            graphics_pipeline,
            command_pool,
            _command_buffer: command_buffer,
        }
    }

    pub unsafe fn destroy(&mut self) {
        self.device.destroy_command_pool(self.command_pool, None);
        self.device.destroy_pipeline(self.graphics_pipeline, None);
        self.device
            .destroy_pipeline_layout(self.pipeline_layout, None);
        for framebuffer in &self.swapchain_framebuffers {
            self.device.destroy_framebuffer(*framebuffer, None);
        }
        self.device.destroy_render_pass(self.render_pass, None);
        for image_view in &self.image_views {
            self.device.destroy_image_view(*image_view, None);
        }
        self.swapchain_loader
            .destroy_swapchain(self.swapchain, None);
        self.surface_loader.destroy_surface(self.surface, None);
        self.device.destroy_device(None);
        self.vk_instance.destroy_instance(None);
    }
}
