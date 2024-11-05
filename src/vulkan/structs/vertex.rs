use ash::vk::{
    Format, VertexInputAttributeDescription, VertexInputBindingDescription, VertexInputRate,
};
use glam::Vec2;
use std::mem::{offset_of, size_of};

pub struct Vertex {
    pub position: Vec2,
    pub texture_coordinates: Vec2,
}

impl Vertex {
    pub fn get_binding_description() -> VertexInputBindingDescription {
        VertexInputBindingDescription::default()
            .binding(0)
            .stride(size_of::<Vertex>() as u32)
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
