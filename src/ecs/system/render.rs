use crate::{
    ecs::{
        component::{composition::VisualWithPosition, ComponentManager, Layer},
        system::ResourceSystem,
    },
    game::GameState,
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

pub struct RenderSystem {
    geometry: Quad,
    vertex_buffer: Buffer,
    vertex_buffer_memory: DeviceMemory,
    index_buffer: Buffer,
    index_buffer_memory: DeviceMemory,
}

impl RenderSystem {
    pub fn create() -> Self {
        Self {
            geometry: Quad::new(),
            vertex_buffer: Buffer::null(),
            vertex_buffer_memory: DeviceMemory::null(),
            index_buffer: Buffer::null(),
            index_buffer_memory: DeviceMemory::null(),
        }
    }

    pub fn initialize(&mut self, window: &Window) {
        (self.index_buffer, self.index_buffer_memory) =
            window.create_index_buffer(&self.geometry.indices);
        (self.vertex_buffer, self.vertex_buffer_memory) =
            window.create_vertex_buffer(self.geometry.get_vertices());
    }

    pub fn draw(
        &mut self,
        current_state: &GameState,
        component_manager: &mut ComponentManager,
        resource_system: &ResourceSystem,
        window: &mut Window,
    ) -> Duration {
        let start_time = Instant::now();

        // get all components that have a visual and a position
        let mut components: Vec<VisualWithPosition> =
            Vec::with_capacity(component_manager.visual_storage.size());
        for (entity, visual) in component_manager.visual_storage.iter_mut() {
            if !visual.should_render() {
                continue;
            }
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
            let layer_ordering = a
                .visual
                .get_layer()
                .value()
                .cmp(&b.visual.get_layer().value());
            if layer_ordering == Ordering::Equal {
                a.position.xyz.z.total_cmp(&b.position.xyz.z)
            } else {
                layer_ordering
            }
        });

        let textures: Vec<ImageView> = components
            .iter()
            .map(|visual_with_position| {
                let texture_index = visual_with_position.visual.get_current_texture();
                resource_system.get_texture(texture_index).get_view()
            })
            .collect();

        let view_matrix = match current_state {
            GameState::Game => {
                let player_id = component_manager
                    .player_entity
                    .as_ref()
                    .expect("No player entity when determining view matrix in game state!")
                    .id;

                let player_position = component_manager
                    .position_storage
                    .get(player_id)
                    .expect("Failed to get position of player!")
                    .xyz;

                Mat4::from_translation(-player_position)
            }
            // never move anything outside of game
            _ => Mat4::IDENTITY,
        };

        let mvps: Vec<ModelViewProjection> = components
            .iter()
            .map(|visual_with_position| {
                let view = match visual_with_position.visual.get_layer() {
                    // never move interface entities
                    Layer::Interface => Mat4::IDENTITY,
                    Layer::Game | Layer::Background => view_matrix,
                };

                ModelViewProjection {
                    model: visual_with_position.position.to_model_matrix(),
                    view,
                    projection: ModelViewProjection::get_projection(),
                }
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
        self.geometry.indices.len() as u32
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn destroy(&self, device: &Device) {
        device.destroy_buffer(self.index_buffer, None);
        device.free_memory(self.index_buffer_memory, None);

        device.destroy_buffer(self.vertex_buffer, None);
        device.free_memory(self.vertex_buffer_memory, None);
    }
}

pub struct Quad {
    pub top_right: Vec2,
    pub top_left: Vec2,
    pub bottom_left: Vec2,
    pub bottom_right: Vec2,
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
}

impl Quad {
    pub fn new() -> Self {
        let bottom_left = Vertex {
            position: Vec2 { x: -0.5, y: -0.5 },
            texture_coordinates: Vec2 { x: 0.0, y: 0.0 },
        };
        let bottom_right = Vertex {
            position: Vec2 { x: 0.5, y: -0.5 },
            texture_coordinates: Vec2 { x: 1.0, y: 0.0 },
        };
        let top_right = Vertex {
            position: Vec2 { x: 0.5, y: 0.5 },
            texture_coordinates: Vec2 { x: 1.0, y: 1.0 },
        };
        let top_left = Vertex {
            position: Vec2 { x: -0.5, y: 0.5 },
            texture_coordinates: Vec2 { x: 0.0, y: 1.0 },
        };

        Self {
            top_right: top_right.position,
            top_left: top_left.position,
            bottom_left: bottom_left.position,
            bottom_right: bottom_right.position,
            vertices: vec![bottom_left, bottom_right, top_right, top_left],
            indices: vec![0, 1, 2, 2, 3, 0],
        }
    }

    pub fn get_vertices(&self) -> &[Vertex] {
        &self.vertices
    }

    pub fn position_is_inside(&self, position: Vec2) -> bool {
        // https://en.wikipedia.org/wiki/Point_in_polygon#Ray_casting_algorithm
        let mut intersections = 0;

        let edges = [
            (self.top_left, self.top_right),
            (self.top_right, self.bottom_right),
            (self.bottom_right, self.bottom_left),
            (self.bottom_left, self.top_left),
        ];

        for (start, end) in &edges {
            if (start.y > position.y) != (end.y > position.y) {
                let slope = (end.x - start.x) / (end.y - start.y);
                let intersect_x = start.x + slope * (position.y - start.y);

                if position.x < intersect_x {
                    intersections += 1;
                }
            }
        }

        intersections % 2 != 0
    }
}
