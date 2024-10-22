mod renderer;
use crate::{
    ecs::system::RenderSystem,
    structs::{ImageData, ModelViewProjection, Vertex},
    vulkan::VulkanWrapper,
};
use ash::{
    khr::surface,
    vk::{Buffer, DeviceMemory, ImageView, PhysicalDevice, SurfaceKHR},
    Device, Entry, Instance,
};
use image::{ImageBuffer, Rgba};
use renderer::Renderer;

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
    pub fn create(inner_window: winit::window::Window, max_texture_count: u32) -> Self {
        let entry = Entry::linked();
        let vk_instance = VulkanWrapper::create_vulkan_instance(&entry, &inner_window);
        let (surface, surface_loader) =
            VulkanWrapper::create_surface(&inner_window, &entry, &vk_instance);
        let (physical_device, queue_family_index) =
            VulkanWrapper::find_physical_device(&vk_instance, surface, &surface_loader);
        let device =
            VulkanWrapper::create_logical_device(&vk_instance, physical_device, queue_family_index);
        let renderer = Renderer::create(
            &vk_instance,
            physical_device,
            &device,
            surface,
            &surface_loader,
            queue_family_index,
            max_texture_count,
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

    pub fn draw(
        &mut self,
        render_system: &RenderSystem,
        textures: &[ImageView],
        mvps: &[ModelViewProjection],
    ) {
        if self
            .inner
            .is_minimized()
            .expect("Failed to determine whether window is minimized!")
        {
            return;
        }

        self.renderer.draw_frame(
            &self.vk_instance,
            self.physical_device,
            &self.device,
            self.surface,
            &self.surface_loader,
            render_system,
            textures,
            mvps,
        );
    }

    pub fn request_render(&self) {
        self.inner.request_redraw();
    }

    pub fn wait_idle(&self) {
        unsafe { self.device.device_wait_idle() }.expect("Failed to wait for device idle!");
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

    pub fn create_texture(&self, image: ImageBuffer<Rgba<u8>, Vec<u8>>) -> ImageData {
        self.renderer
            .create_texture(&self.vk_instance, self.physical_device, &self.device, image)
    }

    pub fn get_device(&self) -> &Device {
        &self.device
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn destroy(&self) {
        self.renderer.destroy(&self.device);
        self.surface_loader.destroy_surface(self.surface, None);
        self.device.destroy_device(None);
        self.vk_instance.destroy_instance(None);
    }
}
