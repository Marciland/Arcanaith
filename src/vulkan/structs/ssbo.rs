use crate::{
    structs::ModelViewProjection,
    vulkan::{InternalVulkan, VulkanWrapper, Wrapper},
};
use ash::{
    vk::{
        Buffer, BufferUsageFlags, DescriptorSet, DeviceMemory, MemoryMapFlags, MemoryPropertyFlags,
        PhysicalDevice,
    },
    Device, Instance,
};
use std::{mem::size_of, ptr::copy_nonoverlapping};

pub struct StorageBufferObject {
    buffer: Buffer,
    memory: DeviceMemory,
    _mapped: *mut ModelViewProjection,
    _capacity: usize,
}

impl StorageBufferObject {
    pub fn create(
        instance: &Instance,
        physical_device: PhysicalDevice,
        device: &Device,
        capacity: usize,
    ) -> Self {
        let buffer_size = (capacity * size_of::<ModelViewProjection>()) as u64;

        let (buffer, memory) = InternalVulkan::create_buffer(
            instance,
            physical_device,
            device,
            buffer_size,
            BufferUsageFlags::STORAGE_BUFFER,
            MemoryPropertyFlags::HOST_VISIBLE | MemoryPropertyFlags::HOST_COHERENT,
        );
        let mapped = unsafe {
            device
                .map_memory(memory, 0, buffer_size, MemoryMapFlags::empty())
                .expect("Failed to map memory for SSBO!") as *mut ModelViewProjection
        };

        Self {
            buffer,
            memory,
            _mapped: mapped,
            _capacity: capacity,
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
        if entity_count <= self._capacity {
            return;
        }

        // add 50% more capacity
        let buffer_size =
            ((entity_count as f32 * 1.5) as usize * size_of::<ModelViewProjection>()) as u64;
        let (buffer, memory) = InternalVulkan::create_buffer(
            instance,
            physical_device,
            device,
            buffer_size,
            BufferUsageFlags::STORAGE_BUFFER,
            MemoryPropertyFlags::HOST_VISIBLE | MemoryPropertyFlags::HOST_COHERENT,
        );

        unsafe {
            self.destroy(device);
        }

        self.buffer = buffer;
        self.memory = memory;
        self._mapped = unsafe {
            device
                .map_memory(memory, 0, buffer_size, MemoryMapFlags::empty())
                .expect("Failed to map memory for SSBO while resizing!")
                as *mut ModelViewProjection
        };

        VulkanWrapper::update_mvp_descriptors(device, descriptor_set, entity_count, self.buffer);
    }

    pub fn update_data(&self, data: &[ModelViewProjection]) {
        assert!(
            data.len() <= self._capacity,
            "More MVPs than mvp_buffer capacity!"
        );

        unsafe {
            copy_nonoverlapping(data.as_ptr(), self._mapped, data.len());
        }
    }

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
