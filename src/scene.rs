use std::{cell::RefCell, rc::Rc};

use ash::vk::{Buffer, DeviceMemory};

use crate::{vertex::Vertex, window::Window};

pub struct Scene {
    _window: Rc<RefCell<Window>>,
    index_buffer: Buffer,
    _index_buffer_memory: DeviceMemory,
    vertex_buffer: Buffer,
    _vertex_buffer_memory: DeviceMemory,
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
            unsafe { window.borrow().create_buffers(vertices, indices) };

        Self {
            _window: window,
            index_buffer,
            _index_buffer_memory: index_buffer_memory,
            vertex_buffer,
            _vertex_buffer_memory: vertex_buffer_memory,
        }
    }

    pub fn get_buffers(&self) -> (Buffer, Buffer) {
        (self.index_buffer, self.vertex_buffer)
    }
}
