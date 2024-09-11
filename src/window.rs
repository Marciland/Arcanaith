use std::ffi::c_void;

use crate::{
    scene::Scene,
    shader_structs::Vertex,
    vulkan::{VulkanInitializer, VulkanRenderer, VulkanWrapper},
};
use ash::{
    khr::{surface, swapchain},
    vk::{
        Buffer, CommandBuffer, CommandBufferResetFlags, CommandPool, DescriptorPool, DescriptorSet,
        DescriptorSetLayout, DeviceMemory, Extent2D, Fence, Format, Framebuffer, Image, ImageView,
        PhysicalDevice, Pipeline, PipelineLayout, PipelineStageFlags, PresentInfoKHR, Queue,
        RenderPass, Sampler, Semaphore, SubmitInfo, SurfaceKHR, SwapchainKHR,
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
    render_pass: RenderPass,                    // store in scene
    descriptor_set_layout: DescriptorSetLayout, // store in scene
    pipeline_layout: PipelineLayout,            // store in scene
    graphics_pipeline: Pipeline,                // store in scene
    command_pool: CommandPool,                  // profile, maybe in scene?
    command_buffer: CommandBuffer,              // profile...
    uniform_buffers: Vec<Buffer>,               // store per object
    uniform_buffers_memory: Vec<DeviceMemory>,  // store per object
    uniform_buffers_mapped: Vec<*mut c_void>,   // store per object
    descriptor_pool: DescriptorPool,            // store in scene
    descriptor_set: DescriptorSet,              // store in scene
    image_available: Semaphore,
    render_finished: Semaphore,
    in_flight: Fence,
    texture_image: Image,               // store per object
    texture_image_memory: DeviceMemory, // store per object
    texture_image_view: ImageView,      // store per object
    texture_sampler: Sampler,           // store in scene
    depth_image: Image,
    depth_image_memory: DeviceMemory,
    depth_image_view: ImageView,
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
        let render_pass =
            VulkanWrapper::create_render_pass(&vk_instance, physical_device, &device, format);
        let descriptor_set_layout = VulkanWrapper::create_descriptor_set_layout(&device);
        let (pipeline_layout, graphics_pipeline) = VulkanWrapper::create_graphics_pipeline(
            &device,
            extent,
            render_pass,
            descriptor_set_layout,
        );
        let command_pool = VulkanWrapper::create_command_pool(&device, queue_family_index);
        let (uniform_buffers, uniform_buffers_memory, uniform_buffers_mapped) =
            VulkanWrapper::create_uniform_buffers(&vk_instance, physical_device, &device);
        let command_buffer = VulkanWrapper::create_command_buffer(&device, command_pool); // combine this with pool creation
        let (depth_image, depth_image_memory, depth_image_view) =
            VulkanWrapper::create_depth_image_view(&vk_instance, physical_device, &device, extent);
        let swapchain_framebuffers = VulkanWrapper::create_framebuffers(
            &device,
            render_pass,
            &image_views,
            depth_image_view,
            extent,
        );
        let (texture_image, texture_image_memory, texture_image_view) =
            VulkanWrapper::create_texture_image(
                &vk_instance,
                physical_device,
                &device,
                graphics_queue,
                command_pool,
            );
        let texture_sampler =
            VulkanWrapper::create_texture_sampler(&vk_instance, physical_device, &device);
        let (descriptor_pool, descriptor_set) = VulkanWrapper::create_descriptors(
            &device,
            descriptor_set_layout,
            &uniform_buffers,
            texture_image_view,
            texture_sampler,
        );
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
            descriptor_set_layout,
            pipeline_layout,
            graphics_pipeline,
            command_pool,
            command_buffer,
            uniform_buffers,
            uniform_buffers_memory,
            uniform_buffers_mapped,
            descriptor_pool,
            descriptor_set,
            image_available,
            render_finished,
            in_flight,
            texture_image,
            texture_image_memory,
            texture_image_view,
            texture_sampler,
            depth_image,
            depth_image_memory,
            depth_image_view,
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

        let (index_buffer, vertex_buffer) = scene.get_buffers();

        VulkanWrapper::begin_render_pass(
            &self.device,
            self.render_pass,
            &self.swapchain_framebuffers,
            &[vertex_buffer],
            index_buffer,
            image_index as usize,
            self.command_buffer,
            self.graphics_pipeline,
            self.extent,
            scene.get_index_count(),
            self.pipeline_layout,
            &[self.descriptor_set],
        );

        VulkanWrapper::update_uniform_buffer(self.extent, self.uniform_buffers_mapped[0]);

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

    pub fn create_vertex_buffer(&self, vertices: Vec<Vertex>) -> (Buffer, DeviceMemory) {
        unsafe {
            VulkanWrapper::create_vertex_buffer(
                &self.vk_instance,
                self.physical_device,
                &self.device,
                &vertices,
                self.command_pool,
                self.graphics_queue,
            )
        }
    }

    pub fn create_index_buffer(&self, indices: &[u16]) -> (Buffer, DeviceMemory) {
        unsafe {
            VulkanWrapper::create_index_buffer(
                &self.vk_instance,
                self.physical_device,
                &self.device,
                indices,
                self.graphics_queue,
                self.command_pool,
            )
        }
    }

    unsafe fn recreate_swapchain(&mut self) {
        // https://docs.vulkan.org/tutorial/latest/03_Drawing_a_triangle/04_Swap_chain_recreation.html#_recreating_the_swap_chain
        if self.window.is_minimized().unwrap() {
            return;
        };

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
        let (depth_image, depth_image_memory, depth_image_view) =
            VulkanWrapper::create_depth_image_view(
                &self.vk_instance,
                self.physical_device,
                &self.device,
                extent,
            );
        let framebuffers = VulkanWrapper::create_framebuffers(
            &self.device,
            self.render_pass,
            &image_views,
            depth_image_view,
            extent,
        );

        self.swapchain = swapchain;
        self.swapchain_loader = swapchain_loader;
        self.format = format;
        self.extent = extent;
        self.images = images;
        self.image_views = image_views;
        self.depth_image = depth_image;
        self.depth_image_memory = depth_image_memory;
        self.depth_image_view = depth_image_view;
        self.swapchain_framebuffers = framebuffers;
    }

    pub unsafe fn destroy(&self, scene: &Scene) {
        self.destroy_sync_elements();
        self.device.destroy_command_pool(self.command_pool, None);
        self.device.destroy_pipeline(self.graphics_pipeline, None);
        self.device
            .destroy_pipeline_layout(self.pipeline_layout, None);
        self.device.destroy_render_pass(self.render_pass, None);
        self.destroy_swapchain_elements();
        for index in 0..self.uniform_buffers.len() {
            self.device
                .destroy_buffer(self.uniform_buffers[index], None);
            self.device
                .free_memory(self.uniform_buffers_memory[index], None);
        }
        self.device
            .destroy_descriptor_pool(self.descriptor_pool, None);
        self.device
            .destroy_descriptor_set_layout(self.descriptor_set_layout, None);
        self.device.destroy_sampler(self.texture_sampler, None);
        self.device
            .destroy_image_view(self.texture_image_view, None);
        self.device.destroy_image(self.texture_image, None);
        self.device.free_memory(self.texture_image_memory, None);
        scene.destroy_buffers(&self.device);
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
        self.device.destroy_image_view(self.depth_image_view, None);
        self.device.destroy_image(self.depth_image, None);
        self.device.free_memory(self.depth_image_memory, None);

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
