use std::{cell::RefCell, rc::Rc};

use ash::{
    vk::{Buffer, DeviceMemory},
    Device,
};

use crate::{vertex::Vertex, window::Window};

pub struct Scene {
    _window: Rc<RefCell<Window>>,
    indices: Vec<u16>,
    index_buffer: Buffer,
    index_buffer_memory: DeviceMemory,
    vertex_buffer: Buffer,
    vertex_buffer_memory: DeviceMemory,
}

impl Scene {
    pub fn load_menu(window: Rc<RefCell<Window>>) -> Self {
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

        let (index_buffer, index_buffer_memory, vertex_buffer, vertex_buffer_memory) =
            unsafe { window.borrow().create_buffers(vertices, &indices) };

        Self {
            _window: window,
            indices,
            index_buffer,
            index_buffer_memory,
            vertex_buffer,
            vertex_buffer_memory,
        }
    }

    pub fn get_buffers(&self) -> (Buffer, Buffer) {
        (self.index_buffer, self.vertex_buffer)
    }

    pub fn get_index_count(&self) -> u32 {
        self.indices.len() as u32
    }

    pub unsafe fn destroy_buffers(&self, device: &Device) {
        device.destroy_buffer(self.index_buffer, None);
        device.free_memory(self.index_buffer_memory, None);
        device.destroy_buffer(self.vertex_buffer, None);
        device.free_memory(self.vertex_buffer_memory, None);
    }
}
