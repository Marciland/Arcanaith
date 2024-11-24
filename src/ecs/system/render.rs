use crate::{
    ecs::{
        component::{
            composition::{TextWithPosition, VisualWithPosition},
            ComponentManager, Layer,
        },
        system::ResourceSystem,
    },
    objects::{Object, Quad},
    scenes::Scene,
    structs::ModelViewProjection,
    Window,
};
use ash::{
    vk::{Buffer, DeviceMemory, ImageView},
    Device,
};
use glam::Mat4;
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
            window.create_index_buffer(self.geometry.get_indices());
        (self.vertex_buffer, self.vertex_buffer_memory) =
            window.create_vertex_buffer(self.geometry.get_vertices());
    }

    pub fn draw(
        &mut self,
        current_scene: &Scene,
        component_manager: &mut ComponentManager,
        resource_system: &ResourceSystem,
        window: &mut Window,
    ) -> Duration {
        let start_time = Instant::now();
        let entity_amount =
            component_manager.visual_storage.size() + component_manager.text_storage.size();
        let mut image_data: Vec<ImageView> = Vec::with_capacity(entity_amount);
        let mut positions: Vec<ModelViewProjection> = Vec::with_capacity(entity_amount);

        let (bitmaps, text_positions) =
            prepare_text_components(component_manager, resource_system, window);
        image_data.extend(bitmaps);
        positions.extend(text_positions);

        let (visual_textures, visual_positions) =
            prepare_visual_components(current_scene, component_manager, resource_system);
        image_data.extend(visual_textures);
        positions.extend(visual_positions);

        window.draw(self, &image_data, &positions);

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
        self.geometry.get_indices().len() as u32
    }

    pub fn destroy(&self, device: &Device) {
        unsafe {
            device.destroy_buffer(self.index_buffer, None);
            device.free_memory(self.index_buffer_memory, None);

            device.destroy_buffer(self.vertex_buffer, None);
            device.free_memory(self.vertex_buffer_memory, None);
        }
    }
}

fn prepare_visual_components(
    current_scene: &Scene,
    component_manager: &mut ComponentManager,
    resource_system: &ResourceSystem,
) -> (Vec<ImageView>, Vec<ModelViewProjection>) {
    // get all components that have a visual and a position
    let mut visual_components: Vec<VisualWithPosition> =
        Vec::with_capacity(component_manager.visual_storage.size());
    for (entity, visual) in component_manager.visual_storage.iter_mut() {
        if !visual.should_render() {
            continue;
        }
        if let Some(position) = component_manager.position_storage.get(entity) {
            visual_components.push(VisualWithPosition { visual, position });
        }
    }

    // update textures of all animated components
    for component in &mut visual_components {
        component.visual.update_animation();
    }

    // sort by layer and by individual z inside layers
    visual_components.sort_by(|a, b| {
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

    let textures: Vec<ImageView> = visual_components
        .iter()
        .map(|visual_with_position| {
            let texture_index = visual_with_position.visual.get_current_texture();
            resource_system.get_texture(texture_index).get_view()
        })
        .collect();

    let view_matrix = match current_scene {
        Scene::Game(game) => {
            let mut player_id: Option<u32> = None;

            for obj in &game.objects {
                player_id = match obj {
                    Object::Player(player) => Some(player.id),
                    _ => None,
                };
            }

            let Some(player_id) = player_id else {
                panic!("No player id in game!")
            };

            let Some(player_position) = component_manager.position_storage.get(player_id) else {
                panic!("Player has no position!")
            };

            Mat4::from_translation(-player_position.xyz)
        }
        // never move anything outside of game
        _ => Mat4::IDENTITY,
    };

    let mvps: Vec<ModelViewProjection> = visual_components
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

    (textures, mvps)
}

fn prepare_text_components(
    component_manager: &mut ComponentManager,
    resource_system: &ResourceSystem,
    window: &Window,
) -> (Vec<ImageView>, Vec<ModelViewProjection>) {
    // get all components that have a text and a position
    let mut text_components: Vec<TextWithPosition> =
        Vec::with_capacity(component_manager.text_storage.size());
    for (entity, text) in component_manager.text_storage.iter_mut() {
        if let Some(position) = component_manager.position_storage.get(entity) {
            text_components.push(TextWithPosition { text, position });
        }
    }

    let bitmaps: Vec<ImageView> = text_components
        .iter_mut()
        .map(|text_with_position| text_with_position.text.get_bitmap(window, resource_system))
        .collect();

    let positions: Vec<ModelViewProjection> = text_components
        .iter()
        .map(|text_with_position| ModelViewProjection {
            model: text_with_position.position.to_model_matrix(),
            view: Mat4::IDENTITY,
            projection: ModelViewProjection::get_projection(),
        })
        .collect();

    (bitmaps, positions)
}
