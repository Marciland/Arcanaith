use crate::{objects::Quad, Window};
use ash::{
    vk::{Buffer, DeviceMemory},
    Device,
};

pub struct RenderSystem {
    geometry: Quad,
    vertex_buffer: Buffer,
    vertex_buffer_memory: DeviceMemory,
    index_buffer: Buffer,
    index_buffer_memory: DeviceMemory,
}

impl RenderSystem {
    pub fn create() -> Self {
        Self {
            geometry: Quad::new(),
            vertex_buffer: Buffer::null(),
            vertex_buffer_memory: DeviceMemory::null(),
            index_buffer: Buffer::null(),
            index_buffer_memory: DeviceMemory::null(),
        }
    }

    pub fn initialize(&mut self, window: &Window) {
        (self.index_buffer, self.index_buffer_memory) =
            window.create_index_buffer(self.geometry.get_indices());
        (self.vertex_buffer, self.vertex_buffer_memory) =
            window.create_vertex_buffer(self.geometry.get_vertices());
    }

    pub fn get_index_buffer(&self) -> Buffer {
        self.index_buffer
    }

    pub fn get_vertex_buffer(&self) -> Buffer {
        self.vertex_buffer
    }

    pub fn get_index_count(&self) -> u32 {
        self.geometry.get_indices().len() as u32
    }

    pub fn destroy(&self, device: &Device) {
        unsafe {
            device.destroy_buffer(self.index_buffer, None);
            device.free_memory(self.index_buffer_memory, None);

            device.destroy_buffer(self.vertex_buffer, None);
            device.free_memory(self.vertex_buffer_memory, None);
        }
    }
}
