mod renderer;
use crate::{
    vulkan::{ImageData, Wrapper},
    Scene, UniformBufferObject, Vertex, VulkanWrapper,
};
use ash::{
    khr::surface,
    vk::{
        Buffer, DescriptorPool, DescriptorSet, DescriptorSetLayout, DeviceMemory, ImageView,
        PhysicalDevice, Pipeline, PipelineLayout, RenderPass, Sampler, SurfaceKHR,
    },
    Device, Entry, Instance,
};
use image::{ImageBuffer, Rgba};
use renderer::Renderer;
use std::time::{Duration, Instant};

pub struct Window {
    inner: winit::window::Window,
    renderer: Renderer,
    vk_instance: Instance,
    surface: SurfaceKHR,
    surface_loader: surface::Instance,
    physical_device: PhysicalDevice,
    device: Device,
}

impl Window {
    pub fn create(inner_window: winit::window::Window) -> Self {
        let entry = Entry::linked();
        let vk_instance = VulkanWrapper::create_vulkan_instance(&entry, &inner_window);
        let (surface, surface_loader) =
            VulkanWrapper::create_surface(&inner_window, &entry, &vk_instance);
        let (physical_device, queue_family_index) =
            VulkanWrapper::find_physical_device(&vk_instance, &surface, &surface_loader);
        let device =
            VulkanWrapper::create_logical_device(&vk_instance, physical_device, queue_family_index);
        let renderer = Renderer::new(
            &vk_instance,
            physical_device,
            &device,
            surface,
            &surface_loader,
            queue_family_index,
        );

        inner_window.set_visible(true);
        Self {
            inner: inner_window,
            renderer,
            vk_instance,
            surface,
            surface_loader,
            physical_device,
            device,
        }
    }

    pub fn render(&mut self, scene: &Scene) -> Duration {
        let start_time = Instant::now();

        if self.inner.is_minimized().unwrap() {
            return Instant::now() - start_time;
        }

        self.renderer.draw_frame(
            &self.vk_instance,
            self.physical_device,
            &self.device,
            self.surface,
            &self.surface_loader,
            scene,
        );
        let end_time = Instant::now();

        end_time - start_time
    }

    pub fn request_render(&self) {
        self.inner.request_redraw();
    }

    pub fn create_vertex_buffer(&self, vertices: &[Vertex]) -> (Buffer, DeviceMemory) {
        self.renderer.create_vertex_buffer(
            &self.vk_instance,
            self.physical_device,
            &self.device,
            vertices,
        )
    }

    pub fn create_index_buffer(&self, indices: &[u16]) -> (Buffer, DeviceMemory) {
        self.renderer.create_index_buffer(
            &self.vk_instance,
            self.physical_device,
            &self.device,
            indices,
        )
    }

    pub fn create_render_pass(&self) -> RenderPass {
        VulkanWrapper::create_render_pass(
            &self.vk_instance,
            self.physical_device,
            &self.device,
            self.renderer.get_format(),
        )
    }

    pub fn create_uniform_buffer(&self, buffer_size: u64) -> UniformBufferObject {
        VulkanWrapper::create_uniform_buffer(
            &self.vk_instance,
            self.physical_device,
            &self.device,
            buffer_size,
        )
    }

    pub fn create_texture(&self, image: ImageBuffer<Rgba<u8>, Vec<u8>>) -> ImageData {
        self.renderer
            .create_texture(&self.vk_instance, self.physical_device, &self.device, image)
    }

    pub fn create_texture_sampler(&self) -> Sampler {
        VulkanWrapper::create_texture_sampler(&self.vk_instance, self.physical_device, &self.device)
    }

    pub fn create_descriptor_pool(
        &self,
        texture_count: u32,
    ) -> (DescriptorSetLayout, DescriptorPool) {
        VulkanWrapper::create_descriptor_pool(&self.device, texture_count)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_descriptor_set(
        &self,
        descriptor_pool: DescriptorPool,
        descriptor_set_layout: DescriptorSetLayout,
        texture_image_views: &[ImageView],
        texture_sampler: Sampler,
        object_count: usize,
        object_count_buffer: Buffer,
        mvp_buffer: Buffer,
    ) -> DescriptorSet {
        VulkanWrapper::create_descriptor_set(
            &self.device,
            descriptor_pool,
            descriptor_set_layout,
            texture_image_views,
            texture_sampler,
            object_count,
            object_count_buffer,
            mvp_buffer,
        )
    }

    pub fn create_pipeline(
        &self,
        render_pass: RenderPass,
        descriptor_set_layouts: &[DescriptorSetLayout],
    ) -> (PipelineLayout, Pipeline) {
        VulkanWrapper::create_graphics_pipeline(
            &self.device,
            self.renderer.get_extent(),
            render_pass,
            descriptor_set_layouts,
        )
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn destroy(&self, scene: &Scene) {
        self.renderer.destroy(&self.device);
        scene.destroy(&self.device);
        self.surface_loader.destroy_surface(self.surface, None);
        self.device.destroy_device(None);
        self.vk_instance.destroy_instance(None);
    }
}
