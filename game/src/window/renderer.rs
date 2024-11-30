use crate::{
    constants::FRAMES_IN_FLIGHT,
    structs::{StorageBufferObject, Vertex},
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
    Device, Entry, Instance,
};
use ecs::{ImageData, RenderContext, MVP};
use glam::Vec2;
use image::{ImageBuffer, Rgba};

pub struct Renderer {
    vk_instance: Instance,
    surface: SurfaceKHR,
    surface_loader: surface::Instance,
    physical_device: PhysicalDevice,
    device: Device,
    swapchain: SwapchainKHR,
    swapchain_loader: swapchain::Device,
    swapchain_framebuffers: Vec<Framebuffer>,
    format: Format,
    extent: Extent2D,
    command_pools: Vec<CommandPool>,
    command_buffers: Vec<CommandBuffer>,
    graphics_queue: Queue,
    vertex_buffer: Buffer,
    vertex_buffer_memory: DeviceMemory,
    index_count: u32,
    index_buffer: Buffer,
    index_buffer_memory: DeviceMemory,
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
    suboptimal_timer: usize,
}

impl Renderer {
    #[allow(clippy::too_many_lines)]
    pub fn create(inner_window: &winit::window::Window, max_texture_count: u32) -> Self {
        let entry = Entry::linked();
        let vk_instance = VulkanWrapper::create_vulkan_instance(&entry, inner_window);
        let (surface, surface_loader) =
            VulkanWrapper::create_surface(inner_window, &entry, &vk_instance);
        let (physical_device, queue_family_index) =
            VulkanWrapper::find_physical_device(&vk_instance, surface, &surface_loader);
        let device =
            VulkanWrapper::create_logical_device(&vk_instance, physical_device, queue_family_index);
        let (swapchain, swapchain_loader, format, extent) = VulkanWrapper::create_swapchain(
            &vk_instance,
            surface,
            &device,
            physical_device,
            &surface_loader,
        );
        let (command_pools, command_buffers) =
            VulkanWrapper::create_command_buffers(&device, queue_family_index, FRAMES_IN_FLIGHT);
        let (images, image_views) =
            VulkanWrapper::create_image_views(&swapchain_loader, swapchain, format, &device);
        let graphics_queue = unsafe { device.get_device_queue(queue_family_index, 0) };
        let bottom_left = Vertex {
            position: Vec2 { x: -0.5, y: -0.5 },
            texture_coordinates: Vec2 { x: 0.0, y: 0.0 },
        };
        let bottom_right = Vertex {
            position: Vec2 { x: 0.5, y: -0.5 },
            texture_coordinates: Vec2 { x: 1.0, y: 0.0 },
        };
        let top_right = Vertex {
            position: Vec2 { x: 0.5, y: 0.5 },
            texture_coordinates: Vec2 { x: 1.0, y: 1.0 },
        };
        let top_left = Vertex {
            position: Vec2 { x: -0.5, y: 0.5 },
            texture_coordinates: Vec2 { x: 0.0, y: 1.0 },
        };
        let vertices: Vec<Vertex> = vec![bottom_left, bottom_right, top_right, top_left];
        let (vertex_buffer, vertex_buffer_memory) = VulkanWrapper::create_vertex_buffer(
            &vk_instance,
            physical_device,
            &device,
            &vertices,
            command_pools[0],
            graphics_queue,
        );
        let indices: Vec<u32> = vec![0, 1, 2, 2, 3, 0];
        let (index_buffer, index_buffer_memory) = VulkanWrapper::create_index_buffer(
            &vk_instance,
            physical_device,
            &device,
            &indices,
            graphics_queue,
            command_pools[0],
        );
        let render_pass =
            VulkanWrapper::create_render_pass(&vk_instance, physical_device, &device, format);
        let texture_sampler =
            VulkanWrapper::create_texture_sampler(&vk_instance, physical_device, &device);
        let (descriptor_set_layout, descriptor_pool) =
            VulkanWrapper::create_descriptors(&device, FRAMES_IN_FLIGHT as u32, max_texture_count);
        let descriptor_sets = VulkanWrapper::create_descriptor_sets(
            &device,
            descriptor_pool,
            descriptor_set_layout,
            FRAMES_IN_FLIGHT,
        );
        let (pipeline_layout, pipeline) = VulkanWrapper::create_graphics_pipeline(
            &device,
            extent,
            render_pass,
            &[descriptor_set_layout],
        );
        let depth = VulkanWrapper::create_depth(&vk_instance, physical_device, &device, extent);
        let mut mvp_buffers: Vec<StorageBufferObject> = Vec::with_capacity(FRAMES_IN_FLIGHT);
        let initial_capacity = 100;
        for descriptor_set in &descriptor_sets {
            let ssbo = StorageBufferObject::create(
                &vk_instance,
                physical_device,
                &device,
                initial_capacity,
            );
            VulkanWrapper::update_mvp_descriptors(
                &device,
                *descriptor_set,
                initial_capacity,
                ssbo.get_buffer(),
            );
            mvp_buffers.push(ssbo);
        }
        let (image_available, render_finished, in_flight) =
            VulkanWrapper::create_sync(&device, FRAMES_IN_FLIGHT);

        Self {
            vk_instance,
            surface,
            surface_loader,
            physical_device,
            device,
            swapchain,
            swapchain_loader,
            swapchain_framebuffers: Vec::new(),
            format,
            extent,
            command_pools,
            command_buffers,
            graphics_queue,
            vertex_buffer,
            vertex_buffer_memory,
            index_count: indices.len() as u32,
            index_buffer,
            index_buffer_memory,
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
            suboptimal_timer: 0,
        }
    }

    fn prepare_draw(&mut self, textures: &[ImageView]) -> Option<u32> {
        unsafe {
            self.device
                .wait_for_fences(&[self.in_flight[self.current_frame]], true, u64::MAX)
        }
        .expect("Failed to wait for fences!");

        if !textures.is_empty() {
            VulkanWrapper::update_texture_descriptors(
                &self.device,
                self.descriptor_sets[self.current_frame],
                textures,
                self.texture_sampler,
            );
        }

        // after 30 suboptimal frames, recreate swapchain without return but before acquire image!
        if self.suboptimal_timer == 30 {
            self.recreate_swapchain();
            self.suboptimal_timer = 0;
        }

        let Ok((image_index, suboptimal)) = (unsafe {
            self.swapchain_loader.acquire_next_image(
                self.swapchain,
                u64::MAX,
                self.image_available[self.current_frame],
                Fence::null(),
            )
        }) else {
            self.recreate_swapchain();
            return None;
        };

        if suboptimal {
            self.suboptimal_timer += 1;
        }

        unsafe {
            self.device
                .reset_fences(&[self.in_flight[self.current_frame]])
        }
        .expect("Failed to reset fences!");

        unsafe {
            self.device.reset_command_pool(
                self.command_pools[self.current_frame],
                CommandPoolResetFlags::empty(),
            )
        }
        .expect("Failed to reset command pool!");

        Some(image_index)
    }

    fn end_draw(&mut self, image_index: u32) {
        let wait_semaphores = [self.image_available[self.current_frame]];
        let command_buffers = [self.command_buffers[self.current_frame]];
        let signal_semaphores = [self.render_finished[self.current_frame]];

        let submit_info = SubmitInfo::default()
            .wait_semaphores(&wait_semaphores)
            .wait_dst_stage_mask(&[PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT])
            .command_buffers(&command_buffers)
            .signal_semaphores(&signal_semaphores);

        unsafe {
            self.device.queue_submit(
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
            self.recreate_swapchain();
        };
    }

    fn recreate_swapchain(&mut self) {
        self.wait_idle();

        unsafe { self.destroy_swapchain_elements() };

        let (swapchain, swapchain_loader, format, extent) = VulkanWrapper::create_swapchain(
            &self.vk_instance,
            self.surface,
            &self.device,
            self.physical_device,
            &self.surface_loader,
        );
        let (images, image_views) =
            VulkanWrapper::create_image_views(&swapchain_loader, swapchain, format, &self.device);
        let depth = VulkanWrapper::create_depth(
            &self.vk_instance,
            self.physical_device,
            &self.device,
            extent,
        );
        let framebuffers = VulkanWrapper::create_framebuffers(
            &self.device,
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

    pub fn get_extent(&self) -> Extent2D {
        self.extent
    }

    pub fn get_device(&self) -> &Device {
        &self.device
    }

    pub fn wait_idle(&self) {
        unsafe { self.device.device_wait_idle() }.expect("Failed to wait for device idle!");
    }

    pub unsafe fn destroy(&self) {
        self.device.destroy_buffer(self.index_buffer, None);
        self.device.free_memory(self.index_buffer_memory, None);

        self.device.destroy_buffer(self.vertex_buffer, None);
        self.device.free_memory(self.vertex_buffer_memory, None);

        self.destroy_sync_elements();
        self.destroy_swapchain_elements();

        for index in 0..FRAMES_IN_FLIGHT {
            self.device
                .destroy_command_pool(self.command_pools[index], None);
        }

        self.device.destroy_pipeline(self.pipeline, None);
        self.device
            .destroy_pipeline_layout(self.pipeline_layout, None);

        self.device
            .destroy_descriptor_pool(self.descriptor_pool, None);
        self.device
            .destroy_descriptor_set_layout(self.descriptor_set_layout, None);

        for index in 0..FRAMES_IN_FLIGHT {
            self.mvp_buffers[index].destroy(&self.device);
        }

        self.device.destroy_sampler(self.texture_sampler, None);
        self.device.destroy_render_pass(self.render_pass, None);

        self.surface_loader.destroy_surface(self.surface, None);
        self.device.destroy_device(None);
        self.vk_instance.destroy_instance(None);
    }

    unsafe fn destroy_sync_elements(&self) {
        for index in 0..FRAMES_IN_FLIGHT {
            self.device
                .destroy_semaphore(self.image_available[index], None);
            self.device
                .destroy_semaphore(self.render_finished[index], None);
            self.device.destroy_fence(self.in_flight[index], None);
        }
    }

    unsafe fn destroy_swapchain_elements(&self) {
        self.depth.destroy(&self.device);

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

impl RenderContext for Renderer {
    fn draw(&mut self, textures: &[ImageView], mvps: &[MVP]) {
        let Some(image_index) = self.prepare_draw(textures) else {
            return; // swapchain was recreated, skip frame
        };

        if self.swapchain_framebuffers.is_empty() {
            self.swapchain_framebuffers = VulkanWrapper::create_framebuffers(
                &self.device,
                self.render_pass,
                &self.image_views,
                self.depth.get_view(),
                self.extent,
            );
        }

        VulkanWrapper::begin_render_pass(
            &self.device,
            &self.swapchain_framebuffers,
            image_index as usize,
            self.command_buffers[self.current_frame],
            self.extent,
            self.render_pass,
            self.pipeline,
        );
        self.mvp_buffers[self.current_frame].resize_if_needed(
            &self.vk_instance,
            self.physical_device,
            &self.device,
            mvps.len(),
            self.descriptor_sets[self.current_frame],
        );
        VulkanWrapper::bind_buffers(
            &self.device,
            self.command_buffers[self.current_frame],
            self.vertex_buffer,
            self.index_buffer,
        );
        VulkanWrapper::draw_indexed_instanced(
            &self.device,
            self.command_buffers[self.current_frame],
            self.pipeline_layout,
            self.descriptor_sets[self.current_frame],
            self.index_count,
            mvps,
            &self.mvp_buffers[self.current_frame],
        );
        VulkanWrapper::end_render_pass(&self.device, self.command_buffers[self.current_frame]);

        self.end_draw(image_index);

        self.current_frame = (self.current_frame + 1) % FRAMES_IN_FLIGHT;
    }

    fn create_image_data(&self, image: ImageBuffer<Rgba<u8>, Vec<u8>>) -> ImageData {
        VulkanWrapper::create_image_data(
            &self.vk_instance,
            self.physical_device,
            &self.device,
            self.graphics_queue,
            self.command_pools[self.current_frame],
            Extent2D {
                width: image.width(),
                height: image.height(),
            },
            &image.into_raw(),
        )
    }
}
