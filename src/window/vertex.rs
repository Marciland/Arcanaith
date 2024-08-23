use std::mem::{self, offset_of};

use ash::vk::{
    Format, VertexInputAttributeDescription, VertexInputBindingDescription, VertexInputRate,
};
use glam::{Vec2, Vec3};

// https://docs.vulkan.org/tutorial/latest/04_Vertex_buffers/00_Vertex_input_description.html
#[derive(Clone)]
pub struct Vertex {
    pub pos: Vec2,
    pub color: Vec3,
}

impl Vertex {
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
                .format(Format::R32G32_SFLOAT)
                .offset(offset_of!(Vertex, pos) as u32),
            VertexInputAttributeDescription::default()
                .binding(0)
                .location(1)
                .format(Format::R32G32B32_SFLOAT)
                .offset(offset_of!(Vertex, color) as u32),
        ]
    }
}
