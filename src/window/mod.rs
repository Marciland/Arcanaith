mod vulkan;

use ash::{
    khr::{surface, swapchain},
    vk::{
        CommandBuffer, CommandBufferResetFlags, CommandPool, Extent2D, Fence, Format, Framebuffer,
        Image, ImageView, PhysicalDevice, Pipeline, PipelineLayout, PipelineStageFlags,
        PresentInfoKHR, Queue, RenderPass, Semaphore, SubmitInfo, SurfaceKHR, SwapchainKHR,
    },
    Device, Entry, Instance,
};
use vulkan::{VulkanInterface, VulkanWrapper};

pub struct Window {
    pub window: winit::window::Window,
    vk_instance: Instance,
    surface: SurfaceKHR,
    surface_loader: surface::Instance,
    _physical_device: PhysicalDevice,
    device: Device,
    graphics_queue: Queue,
    swapchain: SwapchainKHR,
    swapchain_loader: swapchain::Device,
    swapchain_framebuffers: Vec<Framebuffer>,
    _images: Vec<Image>,
    image_views: Vec<ImageView>,
    _format: Format,
    extent: Extent2D,
    render_pass: RenderPass,
    pipeline_layout: PipelineLayout,
    graphics_pipeline: Pipeline,
    command_pool: CommandPool,
    command_buffer: CommandBuffer,
    image_available: Semaphore,
    render_finished: Semaphore,
    in_flight: Fence,
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
        let (image_available, render_finished, in_flight) = VulkanWrapper::create_sync(&device);

        window.set_visible(true);
        Self {
            window,
            vk_instance,
            surface,
            surface_loader,
            _physical_device: physical_device,
            device,
            graphics_queue,
            swapchain,
            swapchain_loader,
            swapchain_framebuffers,
            _images: images,
            image_views,
            _format: format,
            extent,
            render_pass,
            pipeline_layout,
            graphics_pipeline,
            command_pool,
            command_buffer,
            image_available,
            render_finished,
            in_flight,
        }
    }

    pub unsafe fn draw_frame(&mut self) {
        // https://docs.vulkan.org/tutorial/latest/03_Drawing_a_triangle/03_Drawing/02_Rendering_and_presentation.html#_waiting_for_the_previous_frame
        self.device
            .wait_for_fences(&[self.in_flight], true, u64::MAX)
            .unwrap();
        self.device.reset_fences(&[self.in_flight]).unwrap();

        // https://docs.vulkan.org/tutorial/latest/03_Drawing_a_triangle/03_Drawing/02_Rendering_and_presentation.html#_acquiring_an_image_from_the_swap_chain
        let image_index = self
            .swapchain_loader
            .acquire_next_image(
                self.swapchain,
                u64::MAX,
                self.image_available,
                Fence::null(),
            )
            .unwrap()
            .0;

        // https://docs.vulkan.org/tutorial/latest/03_Drawing_a_triangle/03_Drawing/02_Rendering_and_presentation.html#_recording_the_command_buffer
        self.device
            .reset_command_buffer(self.command_buffer, CommandBufferResetFlags::empty())
            .unwrap();

        VulkanWrapper::begin_render_pass(
            &self.device,
            self.render_pass,
            &self.swapchain_framebuffers,
            image_index as usize,
            self.command_buffer,
            self.graphics_pipeline,
            self.extent,
        );

        // https://docs.vulkan.org/tutorial/latest/03_Drawing_a_triangle/03_Drawing/02_Rendering_and_presentation.html#_submitting_the_command_buffer
        let wait_semaphores = [self.image_available];
        let command_buffers = [self.command_buffer];
        let signal_semaphores = [self.render_finished];

        let submit_info = SubmitInfo::default()
            .wait_semaphores(&wait_semaphores)
            .wait_dst_stage_mask(&[PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT])
            .command_buffers(&command_buffers)
            .signal_semaphores(&signal_semaphores);

        self.device
            .queue_submit(self.graphics_queue, &[submit_info], self.in_flight)
            .unwrap();

        // https://docs.vulkan.org/tutorial/latest/03_Drawing_a_triangle/03_Drawing/02_Rendering_and_presentation.html#_presentation
        let swapchains = [self.swapchain];
        let image_indices = [image_index];

        let present_info = PresentInfoKHR::default()
            .wait_semaphores(&signal_semaphores)
            .swapchains(&swapchains)
            .image_indices(&image_indices);

        self.swapchain_loader
            .queue_present(self.graphics_queue, &present_info)
            .unwrap();
    }

    pub unsafe fn destroy(&mut self) {
        self.device.device_wait_idle().unwrap();
        self.device.destroy_semaphore(self.image_available, None);
        self.device.destroy_semaphore(self.render_finished, None);
        self.device.destroy_fence(self.in_flight, None);
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
