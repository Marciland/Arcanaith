use crate::{
    vulkan::{ImageData, Wrapper},
    Scene, Vertex, VulkanWrapper,
};
use ash::{
    khr::{surface, swapchain},
    vk::{
        Buffer, CommandBuffer, CommandPool, CommandPoolResetFlags, DeviceMemory, Extent2D, Fence,
        Format, Framebuffer, Image, ImageView, PhysicalDevice, PipelineStageFlags, PresentInfoKHR,
        Queue, Semaphore, SubmitInfo, SurfaceKHR, SwapchainKHR,
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
    command_pool: CommandPool,
    command_buffer: CommandBuffer,
    graphics_queue: Queue,
    images: Vec<Image>,
    image_views: Vec<ImageView>,
    depth: ImageData,
    image_available: Semaphore,
    render_finished: Semaphore,
    in_flight: Fence,
}

impl Renderer {
    pub fn new(
        vk_instance: &Instance,
        physical_device: PhysicalDevice,
        device: &Device,
        surface: SurfaceKHR,
        surface_loader: &surface::Instance,
        queue_family_index: u32,
    ) -> Self {
        let (swapchain, swapchain_loader, format, extent) = VulkanWrapper::create_swapchain(
            vk_instance,
            surface,
            device,
            physical_device,
            surface_loader,
        );
        let (command_pool, command_buffer) =
            VulkanWrapper::create_command_buffer(device, queue_family_index);
        let (images, image_views) =
            VulkanWrapper::create_image_views(&swapchain_loader, swapchain, format, device);
        let graphics_queue = unsafe { device.get_device_queue(queue_family_index, 0) };
        let depth = VulkanWrapper::create_depth(vk_instance, physical_device, device, extent);
        let (image_available, render_finished, in_flight) = VulkanWrapper::create_sync(device);

        Self {
            swapchain,
            swapchain_loader,
            swapchain_framebuffers: Vec::new(),
            format,
            extent,
            command_pool,
            command_buffer,
            graphics_queue,
            images,
            image_views,
            depth,
            image_available,
            render_finished,
            in_flight,
        }
    }

    pub fn draw_frame(
        &mut self,
        vk_instance: &Instance,
        physical_device: PhysicalDevice,
        device: &Device,
        surface: SurfaceKHR,
        surface_loader: &surface::Instance,
        scene: &Scene,
    ) {
        unsafe {
            device
                .wait_for_fences(&[self.in_flight], true, u64::MAX)
                .unwrap();
        };

        let (image_index, _) = match unsafe {
            self.swapchain_loader.acquire_next_image(
                self.swapchain,
                u64::MAX,
                self.image_available,
                Fence::null(),
            )
        } {
            Ok((image_index, suboptimal)) => (image_index, suboptimal),
            Err(_) => {
                return self.recreate_swapchain(
                    vk_instance,
                    physical_device,
                    device,
                    surface,
                    surface_loader,
                    scene,
                )
            }
        };

        unsafe {
            device.reset_fences(&[self.in_flight]).unwrap();

            device
                .reset_command_pool(self.command_pool, CommandPoolResetFlags::empty())
                .unwrap();
        };

        if self.swapchain_framebuffers.is_empty() {
            self.swapchain_framebuffers = VulkanWrapper::create_framebuffers(
                device,
                scene.get_render_pass(),
                &self.image_views,
                self.depth.get_view(),
                self.extent,
            );
        }

        VulkanWrapper::begin_render_pass(
            device,
            &self.swapchain_framebuffers,
            image_index as usize,
            self.command_buffer,
            self.extent,
            scene,
        );
        VulkanWrapper::draw_indexed_instanced(device, self.command_buffer, scene);
        VulkanWrapper::end_render_pass(device, self.command_buffer);

        let wait_semaphores = [self.image_available];
        let command_buffers = [self.command_buffer];
        let signal_semaphores = [self.render_finished];

        let submit_info = SubmitInfo::default()
            .wait_semaphores(&wait_semaphores)
            .wait_dst_stage_mask(&[PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT])
            .command_buffers(&command_buffers)
            .signal_semaphores(&signal_semaphores);

        unsafe {
            device
                .queue_submit(self.graphics_queue, &[submit_info], self.in_flight)
                .unwrap();
        };

        let swapchains = [self.swapchain];
        let image_indices = [image_index];

        let present_info = PresentInfoKHR::default()
            .wait_semaphores(&signal_semaphores)
            .swapchains(&swapchains)
            .image_indices(&image_indices);

        if unsafe {
            self.swapchain_loader
                .queue_present(self.graphics_queue, &present_info)
                .is_err()
        } {
            self.recreate_swapchain(
                vk_instance,
                physical_device,
                device,
                surface,
                surface_loader,
                scene,
            )
        };
    }

    fn recreate_swapchain(
        &mut self,
        vk_instance: &Instance,
        physical_device: PhysicalDevice,
        device: &Device,
        surface: SurfaceKHR,
        surface_loader: &surface::Instance,
        scene: &Scene,
    ) {
        // https://docs.vulkan.org/tutorial/latest/03_Drawing_a_triangle/04_Swap_chain_recreation.html#_recreating_the_swap_chain
        unsafe {
            device.device_wait_idle().unwrap();

            self.destroy_swapchain_elements(device);
        };

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
            scene.get_render_pass(),
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
            self.command_pool,
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
            self.command_pool,
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
            self.command_pool,
            image,
        )
    }

    pub fn get_format(&self) -> Format {
        self.format
    }

    pub fn get_extent(&self) -> Extent2D {
        self.extent
    }

    pub unsafe fn destroy(&self, device: &Device) {
        self.destroy_sync_elements(device);
        self.destroy_swapchain_elements(device);
        device.destroy_command_pool(self.command_pool, None);
    }

    unsafe fn destroy_sync_elements(&self, device: &Device) {
        device.device_wait_idle().unwrap();

        device.destroy_semaphore(self.image_available, None);
        device.destroy_semaphore(self.render_finished, None);

        device.destroy_fence(self.in_flight, None);
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
