use crate::{
    scene::Scene,
    vertex::Vertex,
    vulkan::{VulkanInterface, VulkanWrapper},
};
use ash::{
    khr::{surface, swapchain},
    vk::{
        Buffer, CommandBuffer, CommandBufferResetFlags, CommandPool, DeviceMemory, Extent2D, Fence,
        Format, Framebuffer, Image, ImageView, PhysicalDevice, Pipeline, PipelineLayout,
        PipelineStageFlags, PresentInfoKHR, Queue, RenderPass, Semaphore, SubmitInfo, SurfaceKHR,
        SwapchainKHR,
    },
    Device, Entry, Instance,
};

pub struct Window {
    pub window: winit::window::Window,
    vk_instance: Instance,
    surface: SurfaceKHR,
    surface_loader: surface::Instance,
    physical_device: PhysicalDevice,
    device: Device,
    graphics_queue: Queue,
    swapchain: SwapchainKHR,
    swapchain_loader: swapchain::Device,
    swapchain_framebuffers: Vec<Framebuffer>,
    images: Vec<Image>,
    image_views: Vec<ImageView>,
    format: Format,
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
        let (images, image_views) =
            VulkanWrapper::create_image_views(&swapchain_loader, swapchain, format, &device);
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
            physical_device,
            device,
            graphics_queue,
            swapchain,
            swapchain_loader,
            swapchain_framebuffers,
            images,
            image_views,
            format,
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

    pub unsafe fn draw_frame(&mut self, scene: &Scene) {
        // https://docs.vulkan.org/tutorial/latest/03_Drawing_a_triangle/03_Drawing/02_Rendering_and_presentation.html#_waiting_for_the_previous_frame
        self.device
            .wait_for_fences(&[self.in_flight], true, u64::MAX)
            .unwrap();

        // https://docs.vulkan.org/tutorial/latest/03_Drawing_a_triangle/03_Drawing/02_Rendering_and_presentation.html#_acquiring_an_image_from_the_swap_chain
        let (image_index, _) = match self.swapchain_loader.acquire_next_image(
            self.swapchain,
            u64::MAX,
            self.image_available,
            Fence::null(),
        ) {
            Ok((image_index, suboptimal)) => (image_index, suboptimal),
            Err(_) => return self.recreate_swapchain(),
        };

        // https://docs.vulkan.org/tutorial/latest/03_Drawing_a_triangle/04_Swap_chain_recreation.html#_fixing_a_deadlock
        self.device.reset_fences(&[self.in_flight]).unwrap();

        // https://docs.vulkan.org/tutorial/latest/03_Drawing_a_triangle/03_Drawing/02_Rendering_and_presentation.html#_recording_the_command_buffer
        self.device
            .reset_command_buffer(self.command_buffer, CommandBufferResetFlags::empty())
            .unwrap();

        let (index_buffer, vertex_buffers) = scene.get_buffers();

        VulkanWrapper::begin_render_pass(
            &self.device,
            self.render_pass,
            &self.swapchain_framebuffers,
            &vertex_buffers,
            index_buffer,
            image_index as usize,
            self.command_buffer,
            self.graphics_pipeline,
            self.extent,
            0,
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

        if self
            .swapchain_loader
            .queue_present(self.graphics_queue, &present_info)
            .is_err()
        {
            self.recreate_swapchain()
        };
    }

    pub unsafe fn create_buffers(
        &self,
        all_vertices: Vec<Vec<Vertex>>,
        indices: Vec<u16>,
    ) -> (Buffer, DeviceMemory, Vec<(Buffer, DeviceMemory)>) {
        let (index_buffer, index_buffer_memory) = VulkanWrapper::create_index_buffer(
            &self.vk_instance,
            self.physical_device,
            &self.device,
            indices,
            self.graphics_queue,
            self.command_pool,
        );

        let mut vertex_buffers = Vec::new();
        for vertices in all_vertices {
            let (buffer, buffer_memory) = VulkanWrapper::create_vertex_buffer(
                &self.vk_instance,
                self.physical_device,
                &self.device,
                &vertices,
                self.command_pool,
                self.graphics_queue,
            );
            vertex_buffers.push((buffer, buffer_memory))
        }

        (index_buffer, index_buffer_memory, vertex_buffers)
    }

    unsafe fn recreate_swapchain(&mut self) {
        // https://docs.vulkan.org/tutorial/latest/03_Drawing_a_triangle/04_Swap_chain_recreation.html#_recreating_the_swap_chain
        self.device.device_wait_idle().unwrap();

        self.destroy_swapchain_elements();

        let (swapchain, swapchain_loader, format, extent) = VulkanWrapper::create_swapchain(
            &self.vk_instance,
            self.surface,
            &self.device,
            self.physical_device,
            &self.surface_loader,
        );
        let (images, image_views) =
            VulkanWrapper::create_image_views(&swapchain_loader, swapchain, format, &self.device);
        let framebuffers = VulkanWrapper::create_framebuffers(
            &self.device,
            self.render_pass,
            &image_views,
            extent,
        );

        self.swapchain = swapchain;
        self.swapchain_loader = swapchain_loader;
        self.format = format;
        self.extent = extent;
        self.images = images;
        self.image_views = image_views;
        self.swapchain_framebuffers = framebuffers;
    }

    pub unsafe fn destroy(&self, _scene: &Scene) {
        self.destroy_sync_elements();
        self.device.destroy_command_pool(self.command_pool, None);
        self.device.destroy_pipeline(self.graphics_pipeline, None);
        self.device
            .destroy_pipeline_layout(self.pipeline_layout, None);
        self.device.destroy_render_pass(self.render_pass, None);
        self.destroy_swapchain_elements();

        /* TODO all scene buffers need to be destroyed here

        for each object in scene:

        self.device.destroy_buffer(index_buffer, None);
        self.device.free_memory(index_buffer_memory, None);

        self.device.destroy_buffer(vertex_buffer, None);
        self.device.free_memory(vertex_buffer_memory, None);

        */

        self.surface_loader.destroy_surface(self.surface, None);
        self.device.destroy_device(None);
        self.vk_instance.destroy_instance(None);
    }

    unsafe fn destroy_sync_elements(&self) {
        self.device.device_wait_idle().unwrap();

        self.device.destroy_semaphore(self.image_available, None);
        self.device.destroy_semaphore(self.render_finished, None);

        self.device.destroy_fence(self.in_flight, None);
    }

    unsafe fn destroy_swapchain_elements(&self) {
        for framebuffer in &self.swapchain_framebuffers {
            self.device.destroy_framebuffer(*framebuffer, None);
        }

        for image_view in &self.image_views {
            self.device.destroy_image_view(*image_view, None);
        }

        self.swapchain_loader
            .destroy_swapchain(self.swapchain, None);
    }
}
