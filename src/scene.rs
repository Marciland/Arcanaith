use ash::vk::{Buffer, DeviceMemory};

use crate::vertex::Vertex;

pub struct Scene {
    index_buffer: Buffer,
    index_buffer_memory: DeviceMemory,
    objects: Vec<Buffer>,
}

impl Scene {
    pub fn new() -> Self {
        let vertices = vec![
            Vertex {
                pos: glam::Vec2 { x: -0.5, y: -0.5 },
                color: glam::Vec3 {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                },
            },
            Vertex {
                pos: glam::Vec2 { x: 0.5, y: -0.5 },
                color: glam::Vec3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
            },
            Vertex {
                pos: glam::Vec2 { x: 0.5, y: 0.5 },
                color: glam::Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
            },
            Vertex {
                pos: glam::Vec2 { x: -0.5, y: 0.5 },
                color: glam::Vec3 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                },
            },
        ];
        let indices: Vec<u16> = vec![0, 1, 2, 2, 3, 0];

        let (index_buffer, index_buffer_memory) = VulkanWrapper::create_index_buffer(
            &vk_instance,
            physical_device,
            &device,
            &indices,
            graphics_queue,
            command_pool,
        );
        let (vertex_buffer, vertex_buffer_memory) = VulkanWrapper::create_vertex_buffer(
            &vk_instance,
            physical_device,
            &device,
            &vertices,
            command_pool,
            graphics_queue,
        );

        Self {
            index_buffer,
            index_buffer_memory,
            objects,
        }
    }
}
