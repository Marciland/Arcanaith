use ash::{
    vk::{
        Buffer, DeviceMemory, Format, VertexInputAttributeDescription,
        VertexInputBindingDescription, VertexInputRate,
    },
    Device,
};
use glam::{Mat4, Vec2, Vec3};
use std::{
    ffi::c_void,
    mem::{self, offset_of},
};

pub struct Vertex {
    position: Vec2,
    texture_coordinates: Vec2,
}

impl Vertex {
    pub fn create(xy: Vec2, uv: Vec2) -> Self {
        Self {
            position: xy,
            texture_coordinates: uv,
        }
    }

    pub fn get_binding_description() -> VertexInputBindingDescription {
        VertexInputBindingDescription::default()
            .binding(0)
            .stride(mem::size_of::<Vertex>() as u32)
            .input_rate(VertexInputRate::VERTEX)
    }

    pub fn get_attribute_descriptions() -> Vec<VertexInputAttributeDescription> {
        vec![
            VertexInputAttributeDescription::default()
                .binding(0)
                .location(0)
                .format(Format::R32G32B32_SFLOAT)
                .offset(offset_of!(Vertex, position) as u32),
            VertexInputAttributeDescription::default()
                .binding(0)
                .location(1)
                .format(Format::R32G32_SFLOAT)
                .offset(offset_of!(Vertex, texture_coordinates) as u32),
        ]
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ModelViewProjection {
    pub model: Mat4,
    pub view: Mat4,
    pub projection: Mat4,
}

impl ModelViewProjection {
    pub fn scale(mut self, scaling: Vec2) -> Self {
        self.model *= Mat4::from_scale(Vec3 {
            x: scaling.x,
            y: scaling.y,
            z: 1.0,
        });
        self
    }

    pub fn translate(mut self, translation: Vec2) -> Self {
        self.model *= Mat4::from_translation(Vec3 {
            x: translation.x,
            y: translation.y,
            z: 0.0,
        });
        self
    }
}

impl Default for ModelViewProjection {
    fn default() -> Self {
        Self {
            model: Mat4::IDENTITY,
            view: Mat4::IDENTITY,
            projection: Mat4::orthographic_rh(-1.0, 1.0, -1.0, 1.0, 0.0, 1.0),
        }
    }
}

pub struct UniformBufferObject {
    buffer: Buffer,
    memory: DeviceMemory,
    mapped: *mut c_void,
}

impl UniformBufferObject {
    pub fn create(buffer: Buffer, memory: DeviceMemory, mapped: *mut c_void) -> Self {
        Self {
            buffer,
            memory,
            mapped,
        }
    }

    pub fn get_buffer(&self) -> Buffer {
        self.buffer
    }

    pub fn get_mapped(&self) -> *mut c_void {
        self.mapped
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn destroy(&self, device: &Device) {
        device.destroy_buffer(self.buffer, None);
        device.free_memory(self.memory, None);
    }
}
