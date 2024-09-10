use std::rc::Rc;

use ash::vk::{Buffer, DeviceMemory};

use crate::{vertex::Vertex, window::Window};

pub struct Scene {
    window: Rc<Window>,
    index_buffer: Buffer,
    index_buffer_memory: DeviceMemory,
    objects: Vec<(Buffer, DeviceMemory)>,
}

impl Scene {
    pub fn load_menu(window: Rc<Window>) -> Self {
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

        let all_vertices = vec![vertices];
        let (index_buffer, index_buffer_memory, objects) =
            unsafe { window.create_buffers(all_vertices, indices) };

        Self {
            window,
            index_buffer,
            index_buffer_memory,
            objects,
        }
    }

    pub fn get_buffers(&self) -> (Buffer, Vec<Buffer>) {
        let mut vertex_buffers: Vec<Buffer> = Vec::new();

        for (buffer, _memory) in &self.objects {
            vertex_buffers.push(*buffer);
        }

        (self.index_buffer, vertex_buffers)
    }
}
