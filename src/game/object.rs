use super::Layer;
use crate::{vulkan::ImageData, ModelViewProjection, Vertex, Window};
use ash::{
    vk::{Buffer, DeviceMemory},
    Device,
};
use glam::Vec2;
use std::cell::Ref;

pub enum GameObject {
    Static,
    Dynamic,
}

pub struct Object {
    mvp: ModelViewProjection,
    textures: Vec<ImageData>,
    pub texture_index: usize,
    depth_layer: Layer,
}

impl Object {
    pub fn create(mvp: ModelViewProjection, textures: Vec<ImageData>, depth_layer: Layer) -> Self {
        Self {
            mvp,
            textures,
            texture_index: 0,
            depth_layer,
        }
    }

    pub fn get_mvp(&self) -> &ModelViewProjection {
        &self.mvp
    }

    pub fn get_textures(&self) -> &[ImageData] {
        &self.textures
    }

    pub fn get_depth(&self) -> u8 {
        self.depth_layer.value()
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn destroy(&self, device: &Device) {
        for texture in &self.textures {
            texture.destroy(device);
        }
    }
}

pub struct Quad {
    indices: Vec<u16>,
    _vertices: Vec<Vertex>,
    vertex_buffer: Buffer,
    vertex_buffer_memory: DeviceMemory,
    index_buffer: Buffer,
    index_buffer_memory: DeviceMemory,
}

impl Quad {
    pub fn create(window: Ref<Window>) -> Self {
        let bottom_left = Vertex::create(Vec2 { x: -0.5, y: -0.5 }, Vec2 { x: 0.0, y: 0.0 });
        let bottom_right = Vertex::create(Vec2 { x: 0.5, y: -0.5 }, Vec2 { x: 1.0, y: 0.0 });
        let top_right = Vertex::create(Vec2 { x: 0.5, y: 0.5 }, Vec2 { x: 1.0, y: 1.0 });
        let top_left = Vertex::create(Vec2 { x: -0.5, y: 0.5 }, Vec2 { x: 0.0, y: 1.0 });

        let _vertices: Vec<Vertex> = vec![bottom_left, bottom_right, top_right, top_left];
        let indices = vec![0, 1, 2, 2, 3, 0];

        let (index_buffer, index_buffer_memory) = window.create_index_buffer(&indices);
        let (vertex_buffer, vertex_buffer_memory) = window.create_vertex_buffer(&_vertices);

        Self {
            indices,
            _vertices,
            vertex_buffer,
            vertex_buffer_memory,
            index_buffer,
            index_buffer_memory,
        }
    }

    pub fn get_vertex_buffer(&self) -> Buffer {
        self.vertex_buffer
    }

    pub fn get_index_buffer(&self) -> Buffer {
        self.index_buffer
    }

    pub fn get_index_count(&self) -> u32 {
        self.indices.len() as u32
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn destroy(&self, device: &Device) {
        device.destroy_buffer(self.index_buffer, None);
        device.free_memory(self.index_buffer_memory, None);

        device.destroy_buffer(self.vertex_buffer, None);
        device.free_memory(self.vertex_buffer_memory, None);
    }
}
