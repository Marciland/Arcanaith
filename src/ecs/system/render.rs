use crate::{
    ecs::{
        component::{ComponentManager, PositionComponent, VisualComponent},
        system::ResourceSystem,
    },
    structs::{ModelViewProjection, Vertex},
    Window,
};
use ash::{
    vk::{Buffer, DeviceMemory, ImageView},
    Device,
};
use glam::{Mat4, Vec2};
use std::{
    cmp::Ordering,
    time::{Duration, Instant},
};

struct VisualWithPosition<'component> {
    visual: &'component mut VisualComponent,
    position: &'component PositionComponent,
}

pub struct RenderSystem {
    vertices: Vec<Vertex>,
    vertex_buffer: Buffer,
    vertex_buffer_memory: DeviceMemory,
    indices: Vec<u16>,
    index_buffer: Buffer,
    index_buffer_memory: DeviceMemory,
}

impl RenderSystem {
    pub fn create() -> Self {
        let bottom_left = Vertex::new(Vec2 { x: -0.5, y: -0.5 }, Vec2 { x: 0.0, y: 0.0 });
        let bottom_right = Vertex::new(Vec2 { x: 0.5, y: -0.5 }, Vec2 { x: 1.0, y: 0.0 });
        let top_right = Vertex::new(Vec2 { x: 0.5, y: 0.5 }, Vec2 { x: 1.0, y: 1.0 });
        let top_left = Vertex::new(Vec2 { x: -0.5, y: 0.5 }, Vec2 { x: 0.0, y: 1.0 });

        let vertices: Vec<Vertex> = vec![bottom_left, bottom_right, top_right, top_left];
        let indices: Vec<u16> = vec![0, 1, 2, 2, 3, 0];

        Self {
            vertices,
            vertex_buffer: Buffer::null(),
            vertex_buffer_memory: DeviceMemory::null(),
            indices,
            index_buffer: Buffer::null(),
            index_buffer_memory: DeviceMemory::null(),
        }
    }

    pub fn initialize(&mut self, window: &Window) {
        (self.index_buffer, self.index_buffer_memory) = window.create_index_buffer(&self.indices);
        (self.vertex_buffer, self.vertex_buffer_memory) =
            window.create_vertex_buffer(&self.vertices);
    }

    pub fn draw(
        &mut self,
        component_manager: &mut ComponentManager,
        resource_system: &ResourceSystem,
        window: &mut Window,
    ) -> Duration {
        let start_time = Instant::now();

        // get all components that have a visual and a position
        let mut components: Vec<VisualWithPosition> =
            Vec::with_capacity(component_manager.visual_storage.size());
        for (entity, visual) in component_manager.visual_storage.iter_mut() {
            if let Some(position) = component_manager.position_storage.get(entity) {
                components.push(VisualWithPosition { visual, position });
            }
        }

        // update textures of all animated components
        for component in &mut components {
            component.visual.update_animation();
        }

        // sort by layer and by individual z inside layers
        components.sort_by(|a, b| {
            let layer_ordering = a.visual.get_layer().cmp(&b.visual.get_layer());
            if layer_ordering == Ordering::Equal {
                a.position.xyz.z.total_cmp(&b.position.xyz.z)
            } else {
                layer_ordering
            }
        });

        let textures: Vec<ImageView> = components
            .iter()
            .map(|visual_with_position| {
                resource_system
                    .get_texture(visual_with_position.visual.get_current_texture())
                    .get_view()
            })
            .collect();
        let mvps: Vec<ModelViewProjection> = components
            .iter()
            .map(|visual_with_position| ModelViewProjection {
                model: visual_with_position.position.to_model_matrix(),
                view: Mat4::IDENTITY,
                projection: ModelViewProjection::get_projection(),
            })
            .collect();

        window.draw(self, &textures, &mvps);

        let end_time = Instant::now();

        end_time - start_time
    }

    pub fn get_index_buffer(&self) -> Buffer {
        self.index_buffer
    }

    pub fn get_vertex_buffer(&self) -> Buffer {
        self.vertex_buffer
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
