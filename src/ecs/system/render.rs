use crate::{
    ecs::{
        component::{
            composition::{RenderTarget, TextWithPosition, VisualWithPosition},
            ComponentStorage, Layer, PositionComponent, TextComponent, VisualComponent,
        },
        entity::Entity,
        system::ResourceSystem,
    },
    objects::{Object, Quad},
    scenes::Scene,
    vulkan::structs::ModelViewProjection,
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

    pub fn draw<'components>(
        &self,
        window: &mut Window,
        current_scene: &Scene,
        visual_storage: &'components mut ComponentStorage<VisualComponent>,
        text_storage: &'components mut ComponentStorage<TextComponent>,
        position_storage: &'components ComponentStorage<PositionComponent>,
        resource_system: &mut ResourceSystem,
    ) -> Duration {
        let start_time = Instant::now();

        let entities: Vec<Entity> = current_scene.get_objects().iter().map(Object::id).collect();

        let mut render_targets: Vec<RenderTarget> =
            get_render_targets(&entities, visual_storage, text_storage, position_storage);

        let textures = get_render_resources(window, &mut render_targets, resource_system);

        let player_position: Option<&PositionComponent> = match current_scene {
            Scene::Menu(_) => None,
            Scene::Game(game) => {
                let Some(player) = game.get_player() else {
                    println!("Game without player entity!");
                    return Instant::elapsed(&start_time);
                };
                position_storage.get(player.id)
            }
        };

        let positions = get_render_positions(&mut render_targets, player_position);

        window.draw(self, &textures, &positions);

        Instant::elapsed(&start_time)
    }
}

fn get_render_targets<'components>(
    entities: &[Entity],
    visual_storage: &'components mut ComponentStorage<VisualComponent>,
    text_storage: &'components mut ComponentStorage<TextComponent>,
    position_storage: &'components ComponentStorage<PositionComponent>,
) -> Vec<RenderTarget<'components>> {
    let mut render_targets: Vec<RenderTarget> = Vec::with_capacity(entities.len());

    // collect visual entities
    for (entity, visual) in visual_storage.iter_mut() {
        // only render current scene
        if !entities.contains(&entity) {
            continue;
        }

        // skip invisible
        if !visual.should_render() {
            continue;
        }

        let Some(position) = position_storage.get(entity) else {
            continue;
        };

        // update textures of all animated components
        visual.update_animation();

        render_targets.push(RenderTarget::Visual(VisualWithPosition {
            visual,
            position,
        }));
    }

    // collect text entities
    for (entity, text) in text_storage.iter_mut() {
        // only render current scene
        if !entities.contains(&entity) {
            continue;
        }

        let Some(position) = position_storage.get(entity) else {
            continue;
        };

        render_targets.push(RenderTarget::Text(TextWithPosition { text, position }));
    }

    // sort all by layer and by individual z inside layers
    render_targets.sort_by(|a, b| {
        let layer_ordering = a.get_layer().value().cmp(&b.get_layer().value());
        if layer_ordering == Ordering::Equal {
            a.get_position().xyz.z.total_cmp(&b.get_position().xyz.z)
        } else {
            layer_ordering
        }
    });

    render_targets
}

pub fn get_render_resources(
    window: &mut Window,
    render_targets: &mut [RenderTarget],
    resource_system: &mut ResourceSystem,
) -> Vec<ImageView> {
    render_targets
        .iter_mut()
        .map(|target| match target {
            RenderTarget::Visual(v) => resource_system
                .get_texture(v.visual.get_current_texture())
                .get_view(),
            RenderTarget::Text(t) => resource_system.get_bitmap(window, t.text),
        })
        .collect()
}

fn get_render_positions(
    render_targets: &mut [RenderTarget],
    player_position: Option<&PositionComponent>,
) -> Vec<ModelViewProjection> {
    let view_matrix = match player_position {
        Some(pos) => Mat4::from_translation(-pos.xyz),
        // no player => no camera movement
        None => Mat4::IDENTITY,
    };

    render_targets
        .iter()
        .map(|target| match target {
            RenderTarget::Visual(visual) => {
                // even if there is a player, interface should not move!
                let view = match visual.get_layer() {
                    Layer::Interface => Mat4::IDENTITY,
                    Layer::Game | Layer::Background => view_matrix,
                };
                ModelViewProjection {
                    model: ModelViewProjection::get_model_matrix(visual.position),
                    view,
                    projection: ModelViewProjection::get_projection(),
                }
            }
            RenderTarget::Text(text) => ModelViewProjection {
                model: ModelViewProjection::get_model_matrix(text.position),
                view: Mat4::IDENTITY,
                projection: ModelViewProjection::get_projection(),
            },
        })
        .collect()
}
