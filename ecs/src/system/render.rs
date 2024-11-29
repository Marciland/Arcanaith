use crate::{
    component::{ImageData, MVP},
    entity::EntityProvider,
};

use super::{
    super::{
        component::{
            composition::{RenderTarget, TextWithPosition, VisualWithPosition},
            ComponentStorage, Layer, PositionComponent, TextComponent, VisualComponent,
        },
        entity::Entity,
    },
    ResourceSystem,
};
use ash::vk::ImageView;
use glam::Mat4;
use image::{ImageBuffer, Rgba};
use std::cmp::Ordering;

pub trait RenderContext {
    fn draw(&mut self, textures: &[ImageView], positions: &[MVP]);
    fn create_image_data(&self, image: ImageBuffer<Rgba<u8>, Vec<u8>>) -> ImageData;
}

pub struct RenderSystem;

impl RenderSystem {
    pub fn draw<'components, P, R>(
        renderer: &mut R,
        provider: &P,
        visual_storage: &'components mut ComponentStorage<VisualComponent>,
        text_storage: &'components mut ComponentStorage<TextComponent>,
        position_storage: &'components ComponentStorage<PositionComponent>,
        resource_system: &mut ResourceSystem,
    ) where
        P: EntityProvider,
        R: RenderContext,
    {
        let entities: Vec<Entity> = provider.get_entities();

        let mut render_targets: Vec<RenderTarget> =
            get_render_targets(&entities, visual_storage, text_storage, position_storage);

        let textures = get_render_resources(renderer, &mut render_targets, resource_system);
        let positions =
            get_render_positions(&mut render_targets, provider.get_player(), position_storage);

        renderer.draw(&textures, &positions);
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
        // only render currently active entities
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
        // only render currently active entities
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

fn get_render_resources<R>(
    renderer: &mut R,
    render_targets: &mut [RenderTarget],
    resource_system: &mut ResourceSystem,
) -> Vec<ImageView>
where
    R: RenderContext,
{
    render_targets
        .iter_mut()
        .map(|target| match target {
            RenderTarget::Visual(v) => resource_system
                .get_texture(v.visual.get_current_texture())
                .get_view(),
            RenderTarget::Text(t) => resource_system.get_bitmap(renderer, t.text),
        })
        .collect()
}

fn get_render_positions(
    render_targets: &mut [RenderTarget],
    player: Option<Entity>,
    position_storage: &ComponentStorage<PositionComponent>,
) -> Vec<MVP> {
    let player_position: Option<&PositionComponent> = match player {
        Some(player_entity) => position_storage.get(player_entity),
        None => None,
    };

    let view_matrix = match player_position {
        Some(pos) => Mat4::from_translation(-pos.xyz),
        // no player => no camera movement
        None => Mat4::IDENTITY,
    };

    render_targets
        .iter()
        .map(|target| match target {
            RenderTarget::Visual(visual_with_position) => {
                // even if there is a player, interface should not move!
                let view = match visual_with_position.visual.layer {
                    Layer::Interface => Mat4::IDENTITY,
                    Layer::Game | Layer::Background => view_matrix,
                };
                MVP {
                    model: MVP::get_model_matrix(visual_with_position.position),
                    view,
                    projection: MVP::get_projection(),
                }
            }
            RenderTarget::Text(text) => MVP {
                model: MVP::get_model_matrix(text.position),
                view: Mat4::IDENTITY,
                projection: MVP::get_projection(),
            },
        })
        .collect()
}
