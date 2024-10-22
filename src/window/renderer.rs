use crate::{
    constants::FRAMES_IN_FLIGHT,
    ecs::system::RenderSystem,
    structs::{ImageData, ModelViewProjection, StorageBufferObject, Vertex},
    vulkan::VulkanWrapper,
};
use ash::{
    khr::{surface, swapchain},
    vk::{
        Buffer, CommandBuffer, CommandPool, CommandPoolResetFlags, DescriptorPool, DescriptorSet,
        DescriptorSetLayout, DeviceMemory, Extent2D, Fence, Format, Framebuffer, Image, ImageView,
        PhysicalDevice, Pipeline, PipelineLayout, PipelineStageFlags, PresentInfoKHR, Queue,
        RenderPass, Sampler, Semaphore, SubmitInfo, SurfaceKHR, SwapchainKHR,
    },
    Device, Instance,
};
use image::{ImageBuffer, Rgba};

pub struct Renderer {
    swapchain: SwapchainKHR,
    swapchain_loader: swapchain::Device,
    swapchain_framebuffers: Vec<Framebuffer>,
    format: Format,
    extent: Extent2D,
    command_pools: Vec<CommandPool>,
    command_buffers: Vec<CommandBuffer>,
    graphics_queue: Queue,
    render_pass: RenderPass,
    texture_sampler: Sampler,
    descriptor_set_layout: DescriptorSetLayout,
    descriptor_pool: DescriptorPool,
    descriptor_sets: Vec<DescriptorSet>,
    pipeline: Pipeline,
    pipeline_layout: PipelineLayout,
    images: Vec<Image>,
    image_views: Vec<ImageView>,
    depth: ImageData,
    mvp_buffers: Vec<StorageBufferObject>,
    image_available: Vec<Semaphore>,
    render_finished: Vec<Semaphore>,
    in_flight: Vec<Fence>,
    current_frame: usize,
}

impl Renderer {
    pub fn create(
        vk_instance: &Instance,
        physical_device: PhysicalDevice,
        device: &Device,
        surface: SurfaceKHR,
        surface_loader: &surface::Instance,
        queue_family_index: u32,
        max_texture_count: u32,
    ) -> Self {
        let (swapchain, swapchain_loader, format, extent) = VulkanWrapper::create_swapchain(
            vk_instance,
            surface,
            device,
            physical_device,
            surface_loader,
        );
        let (command_pools, command_buffers) =
            VulkanWrapper::create_command_buffers(device, queue_family_index, FRAMES_IN_FLIGHT);
        let (images, image_views) =
            VulkanWrapper::create_image_views(&swapchain_loader, swapchain, format, device);
        let graphics_queue = unsafe { device.get_device_queue(queue_family_index, 0) };
        let render_pass = VulkanWrapper::create_render_pass(vk_instance, physical_device, device, format);
        let texture_sampler = VulkanWrapper::create_texture_sampler(vk_instance, physical_device, device);
        let (descriptor_set_layout, descriptor_pool) =
            VulkanWrapper::create_descriptors(device, FRAMES_IN_FLIGHT as u32, max_texture_count);
        let descriptor_sets = VulkanWrapper::create_descriptor_sets(
            device,
            descriptor_pool,
            descriptor_set_layout,
            FRAMES_IN_FLIGHT,
        );
        let (pipeline_layout, pipeline) =
            VulkanWrapper::create_graphics_pipeline(device, extent, render_pass, &[descriptor_set_layout]);
        let depth = VulkanWrapper::create_depth(vk_instance, physical_device, device, extent);
        let mut mvp_buffers: Vec<StorageBufferObject> = Vec::with_capacity(FRAMES_IN_FLIGHT);
        let initial_capacity = 100;
        for descriptor_set in &descriptor_sets {
            let ssbo =
                StorageBufferObject::create(vk_instance, physical_device, device, initial_capacity);
            VulkanWrapper::update_mvp_descriptors(
                device,
                *descriptor_set,
                initial_capacity,
                ssbo.get_buffer(),
            );
            mvp_buffers.push(ssbo);
        }
        let (image_available, render_finished, in_flight) =
            VulkanWrapper::create_sync(device, FRAMES_IN_FLIGHT);

        Self {
            swapchain,
            swapchain_loader,
            swapchain_framebuffers: Vec::new(),
            format,
            extent,
            command_pools,
            command_buffers,
            graphics_queue,
            render_pass,
            texture_sampler,
            descriptor_set_layout,
            descriptor_pool,
            descriptor_sets,
            pipeline,
            pipeline_layout,
            images,
            image_views,
            depth,
            mvp_buffers,
            image_available,
            render_finished,
            in_flight,
            current_frame: 0,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn draw_frame(
        &mut self,
        vk_instance: &Instance,
        physical_device: PhysicalDevice,
        device: &Device,
        surface: SurfaceKHR,
        surface_loader: &surface::Instance,
        render_system: &RenderSystem,
        textures: &[ImageView],
        mvps: &[ModelViewProjection],
    ) {
        let Some(image_index) = self.prepare_draw(
            vk_instance,
            physical_device,
            device,
            surface,
            surface_loader,
            textures,
        ) else {
            return; // swapchain was recreated, skip frame
        };

        if self.swapchain_framebuffers.is_empty() {
            self.swapchain_framebuffers = VulkanWrapper::create_framebuffers(
                device,
                self.render_pass,
                &self.image_views,
                self.depth.get_view(),
                self.extent,
            );
        }

        VulkanWrapper::begin_render_pass(
            device,
            &self.swapchain_framebuffers,
            image_index as usize,
            self.command_buffers[self.current_frame],
            self.extent,
            self.render_pass,
            self.pipeline,
        );
        self.mvp_buffers[self.current_frame].resize_if_needed(
            vk_instance,
            physical_device,
            device,
            mvps.len(),
            self.descriptor_sets[self.current_frame],
        );
        VulkanWrapper::draw_indexed_instanced(
            device,
            self.command_buffers[self.current_frame],
            self.pipeline_layout,
            self.descriptor_sets[self.current_frame],
            render_system,
            mvps,
            &self.mvp_buffers[self.current_frame],
        );
        VulkanWrapper::end_render_pass(device, self.command_buffers[self.current_frame]);

        self.end_draw(
            vk_instance,
            physical_device,
            device,
            surface,
            surface_loader,
            image_index,
        );

        self.current_frame = (self.current_frame + 1) % FRAMES_IN_FLIGHT;
    }

    fn prepare_draw(
        &mut self,
        vk_instance: &Instance,
        physical_device: PhysicalDevice,
        device: &Device,
        surface: SurfaceKHR,
        surface_loader: &surface::Instance,
        textures: &[ImageView],
    ) -> Option<u32> {
        unsafe { device.wait_for_fences(&[self.in_flight[self.current_frame]], true, u64::MAX) }
            .expect("Failed to wait for fences!");

        if !textures.is_empty() {
            VulkanWrapper::update_texture_descriptors(
                device,
                self.descriptor_sets[self.current_frame],
                textures,
                self.texture_sampler,
            );
        }

        let Ok((image_index, _suboptimal)) = (unsafe {
            self.swapchain_loader.acquire_next_image(
                self.swapchain,
                u64::MAX,
                self.image_available[self.current_frame],
                Fence::null(),
            )
        }) else {
            self.recreate_swapchain(
                vk_instance,
                physical_device,
                device,
                surface,
                surface_loader,
            );
            return None;
        };

        unsafe { device.reset_fences(&[self.in_flight[self.current_frame]]) }
            .expect("Failed to reset fences!");

        unsafe {
            device.reset_command_pool(
                self.command_pools[self.current_frame],
                CommandPoolResetFlags::empty(),
            )
        }
        .expect("Failed to reset command pool!");

        Some(image_index)
    }

    fn end_draw(
        &mut self,
        vk_instance: &Instance,
        physical_device: PhysicalDevice,
        device: &Device,
        surface: SurfaceKHR,
        surface_loader: &surface::Instance,
        image_index: u32,
    ) {
        let wait_semaphores = [self.image_available[self.current_frame]];
        let command_buffers = [self.command_buffers[self.current_frame]];
        let signal_semaphores = [self.render_finished[self.current_frame]];

        let submit_info = SubmitInfo::default()
            .wait_semaphores(&wait_semaphores)
            .wait_dst_stage_mask(&[PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT])
            .command_buffers(&command_buffers)
            .signal_semaphores(&signal_semaphores);

        unsafe {
            device.queue_submit(
                self.graphics_queue,
                &[submit_info],
                self.in_flight[self.current_frame],
            )
        }
        .expect("Failed to submit queue!");

        let swapchains = [self.swapchain];
        let image_indices = [image_index];

        let present_info = PresentInfoKHR::default()
            .wait_semaphores(&signal_semaphores)
            .swapchains(&swapchains)
            .image_indices(&image_indices);

        if unsafe {
            self.swapchain_loader
                .queue_present(self.graphics_queue, &present_info)
        }
        .is_err()
        {
            self.recreate_swapchain(
                vk_instance,
                physical_device,
                device,
                surface,
                surface_loader,
            );
        };
    }

    fn recreate_swapchain(
        &mut self,
        vk_instance: &Instance,
        physical_device: PhysicalDevice,
        device: &Device,
        surface: SurfaceKHR,
        surface_loader: &surface::Instance,
    ) {
        unsafe { device.device_wait_idle() }.expect("Failed to wait for device idle!");

        unsafe { self.destroy_swapchain_elements(device) };

        let (swapchain, swapchain_loader, format, extent) = VulkanWrapper::create_swapchain(
            vk_instance,
            surface,
            device,
            physical_device,
            surface_loader,
        );
        let (images, image_views) =
            VulkanWrapper::create_image_views(&swapchain_loader, swapchain, format, device);
        let depth = VulkanWrapper::create_depth(vk_instance, physical_device, device, extent);
        let framebuffers = VulkanWrapper::create_framebuffers(
            device,
            self.render_pass,
            &image_views,
            depth.get_view(),
            extent,
        );

        self.swapchain = swapchain;
        self.swapchain_loader = swapchain_loader;
        self.format = format;
        self.extent = extent;
        self.images = images;
        self.image_views = image_views;
        self.depth = depth;
        self.swapchain_framebuffers = framebuffers;
    }

    pub fn create_vertex_buffer(
        &self,
        instance: &Instance,
        physical_device: PhysicalDevice,
        device: &Device,
        vertices: &[Vertex],
    ) -> (Buffer, DeviceMemory) {
        VulkanWrapper::create_vertex_buffer(
            instance,
            physical_device,
            device,
            vertices,
            self.command_pools[self.current_frame],
            self.graphics_queue,
        )
    }

    pub fn create_index_buffer(
        &self,
        instance: &Instance,
        physical_device: PhysicalDevice,
        device: &Device,
        indices: &[u16],
    ) -> (Buffer, DeviceMemory) {
        VulkanWrapper::create_index_buffer(
            instance,
            physical_device,
            device,
            indices,
            self.graphics_queue,
            self.command_pools[self.current_frame],
        )
    }

    pub fn create_texture(
        &self,
        instance: &Instance,
        physical_device: PhysicalDevice,
        device: &Device,
        image: ImageBuffer<Rgba<u8>, Vec<u8>>,
    ) -> ImageData {
        VulkanWrapper::create_texture(
            instance,
            physical_device,
            device,
            self.graphics_queue,
            self.command_pools[self.current_frame],
            image,
        )
    }

    pub unsafe fn destroy(&self, device: &Device) {
        self.destroy_sync_elements(device);
        self.destroy_swapchain_elements(device);

        for index in 0..FRAMES_IN_FLIGHT {
            device.destroy_command_pool(self.command_pools[index], None);
        }

        device.destroy_pipeline(self.pipeline, None);
        device.destroy_pipeline_layout(self.pipeline_layout, None);

        device.destroy_descriptor_pool(self.descriptor_pool, None);
        device.destroy_descriptor_set_layout(self.descriptor_set_layout, None);

        for index in 0..FRAMES_IN_FLIGHT {
            self.mvp_buffers[index].destroy(device);
        }

        device.destroy_sampler(self.texture_sampler, None);
        device.destroy_render_pass(self.render_pass, None);
    }

    unsafe fn destroy_sync_elements(&self, device: &Device) {
        for index in 0..FRAMES_IN_FLIGHT {
            device.destroy_semaphore(self.image_available[index], None);
            device.destroy_semaphore(self.render_finished[index], None);
            device.destroy_fence(self.in_flight[index], None);
        }
    }

    unsafe fn destroy_swapchain_elements(&self, device: &Device) {
        self.depth.destroy(device);

        for framebuffer in &self.swapchain_framebuffers {
            device.destroy_framebuffer(*framebuffer, None);
        }

        for image_view in &self.image_views {
            device.destroy_image_view(*image_view, None);
        }

        self.swapchain_loader
            .destroy_swapchain(self.swapchain, None);
    }
}
