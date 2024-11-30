use ash::{
    vk::{Buffer, DescriptorSet, DeviceMemory, PhysicalDevice},
    Device, Instance,
};
use std::ptr::copy_nonoverlapping;

use super::{
    super::{internal::Internal, RenderAPI},
    MVP,
};

pub struct StorageBufferObject {
    buffer: Buffer,
    memory: DeviceMemory,
    mapped: *mut MVP,
    capacity: usize,
}

impl StorageBufferObject {
    #[must_use]
    pub fn create(
        instance: &Instance,
        physical_device: PhysicalDevice,
        device: &Device,
        capacity: usize,
    ) -> Self {
        let (buffer, memory, mapped) =
            Internal::create_ssbo(instance, physical_device, device, capacity);

        Self {
            buffer,
            memory,
            mapped,
            capacity,
        }
    }

    pub fn resize_if_needed(
        &mut self,
        instance: &Instance,
        physical_device: PhysicalDevice,
        device: &Device,
        entity_count: usize,
        descriptor_set: DescriptorSet,
    ) {
        if entity_count <= self.capacity {
            return;
        }

        // add 50% more capacity
        let new_size = (entity_count as f32 * 1.5) as usize;
        let (buffer, memory, mapped) =
            Internal::create_ssbo(instance, physical_device, device, new_size);

        unsafe {
            self.destroy(device);
        }

        self.buffer = buffer;
        self.memory = memory;
        self.mapped = mapped;

        RenderAPI::update_mvp_descriptors(device, descriptor_set, entity_count, self.buffer);
    }

    pub fn update_data(&self, data: &[MVP]) {
        assert!(
            data.len() <= self.capacity,
            "More MVPs than mvp_buffer capacity!"
        );

        unsafe {
            copy_nonoverlapping(data.as_ptr(), self.mapped, data.len());
        }
    }

    #[must_use]
    pub fn get_buffer(&self) -> Buffer {
        self.buffer
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn destroy(&self, device: &Device) {
        device.unmap_memory(self.memory);
        device.destroy_buffer(self.buffer, None);
        device.free_memory(self.memory, None);
    }
}
