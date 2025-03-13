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
use glam::Vec2;
use image::{ImageBuffer, Rgba};
use std::rc::Rc;
use vulkan::{
    structs::{ImageData, StorageBufferObject, Vertex, MVP},
    Vulkan,
};
use winit::window::Window;

use crate::{RenderAPI, WindowSize};

pub struct VulkanAPI {
    vk_instance: Instance,
    surface: SurfaceKHR,
    surface_loader: surface::Instance,
    physical_device: PhysicalDevice,
    device: Rc<Device>,
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
    frames_in_flight: usize,
}

impl VulkanAPI {
    unsafe fn destroy_sync_elements(&self) {
        for semaphore in &self.image_available {
            self.device.destroy_semaphore(*semaphore, None);
        }
        for semaphore in &self.render_finished {
            self.device.destroy_semaphore(*semaphore, None);
        }
        for fence in &self.in_flight {
            self.device.destroy_fence(*fence, None);
        }
    }

    unsafe fn destroy_swapchain_elements(&self) {
        self.depth.destroy();

        for framebuffer in &self.swapchain_framebuffers {
            self.device.destroy_framebuffer(*framebuffer, None);
        }

        for image_view in &self.image_views {
            self.device.destroy_image_view(*image_view, None);
        }

        self.swapchain_loader
            .destroy_swapchain(self.swapchain, None);
    }

    fn recreate_swapchain(&mut self) {
        self.wait_idle();

        unsafe { self.destroy_swapchain_elements() };

        let (swapchain, swapchain_loader, format, extent) = Vulkan::create_swapchain(
            &self.vk_instance,
            self.surface,
            &self.device,
            self.physical_device,
            &self.surface_loader,
        );
        let (images, image_views) =
            Vulkan::create_image_views(&swapchain_loader, swapchain, format, &self.device);
        let depth = Vulkan::create_depth(
            &self.vk_instance,
            self.physical_device,
            self.device.clone(),
            extent,
        );
        let framebuffers = Vulkan::create_framebuffers(
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
}

impl RenderAPI for VulkanAPI {
    fn create(
        window: &Window,
        max_texture_count: u32,
        title: &str,
        frames_in_flight: usize,
        vertex_path: &str,
        frag_path: &str,
    ) -> Self {
        let entry = Entry::linked();
        let vk_instance = Vulkan::create_vulkan_instance(&entry, window, title);
        let (surface, surface_loader) = Vulkan::create_surface(window, &entry, &vk_instance);
        let (physical_device, queue_family_index) =
            Vulkan::find_physical_device(&vk_instance, surface, &surface_loader);
        let device = Rc::new(Vulkan::create_logical_device(
            &vk_instance,
            physical_device,
            queue_family_index,
        ));
        let (swapchain, swapchain_loader, format, extent) = Vulkan::create_swapchain(
            &vk_instance,
            surface,
            &device,
            physical_device,
            &surface_loader,
        );
        let (command_pools, command_buffers) =
            Vulkan::create_command_buffers(&device, queue_family_index, frames_in_flight);
        let (images, image_views) =
            Vulkan::create_image_views(&swapchain_loader, swapchain, format, &device);
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
        let (vertex_buffer, vertex_buffer_memory) = Vulkan::create_vertex_buffer(
            &vk_instance,
            physical_device,
            &device,
            &vertices,
            command_pools[0],
            graphics_queue,
        );
        let indices: Vec<u32> = vec![0, 1, 2, 2, 3, 0];
        let (index_buffer, index_buffer_memory) = Vulkan::create_index_buffer(
            &vk_instance,
            physical_device,
            &device,
            &indices,
            graphics_queue,
            command_pools[0],
        );
        let render_pass =
            Vulkan::create_render_pass(&vk_instance, physical_device, &device, format);
        let texture_sampler =
            Vulkan::create_texture_sampler(&vk_instance, physical_device, &device);
        let (descriptor_set_layout, descriptor_pool) =
            Vulkan::create_descriptors(&device, frames_in_flight as u32, max_texture_count);
        let descriptor_sets = Vulkan::create_descriptor_sets(
            &device,
            descriptor_pool,
            descriptor_set_layout,
            frames_in_flight,
        );
        let (pipeline_layout, pipeline) = Vulkan::create_graphics_pipeline(
            &device,
            extent,
            render_pass,
            &[descriptor_set_layout],
            vertex_path,
            frag_path,
        );
        let depth = Vulkan::create_depth(&vk_instance, physical_device, device.clone(), extent);
        let mut mvp_buffers: Vec<StorageBufferObject> = Vec::with_capacity(frames_in_flight);
        let initial_capacity = 100;
        for descriptor_set in &descriptor_sets {
            let ssbo = StorageBufferObject::create(
                &vk_instance,
                physical_device,
                &device,
                initial_capacity,
            );
            Vulkan::update_mvp_descriptors(
                &device,
                *descriptor_set,
                initial_capacity,
                ssbo.get_buffer(),
            );
            mvp_buffers.push(ssbo);
        }
        let (image_available, render_finished, in_flight) =
            Vulkan::create_sync(&device, frames_in_flight);

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
            frames_in_flight,
        }
    }

    fn draw(&mut self, textures: &[ImageView], positions: &[MVP]) {
        unsafe {
            self.device
                .wait_for_fences(&[self.in_flight[self.current_frame]], true, u64::MAX)
        }
        .expect("Failed to wait for fences!");

        let Ok((image_index, _suboptimal /* ignore suboptimal for performance */)) = (unsafe {
            self.swapchain_loader.acquire_next_image(
                self.swapchain,
                u64::MAX,
                self.image_available[self.current_frame],
                Fence::null(),
            )
        }) else {
            return self.recreate_swapchain(); // skip frame as swapchain is invalid
        };

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

        if self.swapchain_framebuffers.is_empty() {
            self.swapchain_framebuffers = Vulkan::create_framebuffers(
                &self.device,
                self.render_pass,
                &self.image_views,
                self.depth.get_view(),
                self.extent,
            );
        }

        if !textures.is_empty() {
            Vulkan::update_texture_descriptors(
                &self.device,
                self.descriptor_sets[self.current_frame],
                textures,
                self.texture_sampler,
            );
        }

        Vulkan::begin_render_pass(
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
            positions.len(),
            self.descriptor_sets[self.current_frame],
        );
        Vulkan::bind_buffers(
            &self.device,
            self.command_buffers[self.current_frame],
            self.vertex_buffer,
            self.index_buffer,
        );
        Vulkan::draw_indexed_instanced(
            &self.device,
            self.command_buffers[self.current_frame],
            self.pipeline_layout,
            self.descriptor_sets[self.current_frame],
            self.index_count,
            positions,
            &self.mvp_buffers[self.current_frame],
        );
        Vulkan::end_render_pass(&self.device, self.command_buffers[self.current_frame]);

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

        self.current_frame = (self.current_frame + 1) % self.frames_in_flight;
    }

    fn create_image_data(&self, image: ImageBuffer<Rgba<u8>, Vec<u8>>) -> ImageData {
        Vulkan::create_image_data(
            &self.vk_instance,
            self.physical_device,
            self.device.clone(),
            self.graphics_queue,
            self.command_pools[self.current_frame],
            Extent2D {
                width: image.width(),
                height: image.height(),
            },
            &image.into_raw(),
        )
    }

    fn get_extent(&self) -> WindowSize {
        WindowSize {
            width: self.extent.width,
            height: self.extent.height,
        }
    }

    fn wait_idle(&self) {
        unsafe { self.device.device_wait_idle() }.expect("Failed to wait for device idle!");
    }

    unsafe fn destroy(&self) {
        self.device.destroy_buffer(self.index_buffer, None);
        self.device.free_memory(self.index_buffer_memory, None);

        self.device.destroy_buffer(self.vertex_buffer, None);
        self.device.free_memory(self.vertex_buffer_memory, None);

        self.destroy_sync_elements();
        self.destroy_swapchain_elements();

        for command_pool in &self.command_pools {
            self.device.destroy_command_pool(*command_pool, None);
        }

        self.device.destroy_pipeline(self.pipeline, None);
        self.device
            .destroy_pipeline_layout(self.pipeline_layout, None);

        self.device
            .destroy_descriptor_pool(self.descriptor_pool, None);
        self.device
            .destroy_descriptor_set_layout(self.descriptor_set_layout, None);

        for buffer in &self.mvp_buffers {
            buffer.destroy(&self.device);
        }

        self.device.destroy_sampler(self.texture_sampler, None);
        self.device.destroy_render_pass(self.render_pass, None);

        self.surface_loader.destroy_surface(self.surface, None);
        self.device.destroy_device(None);
        self.vk_instance.destroy_instance(None);
    }
}
