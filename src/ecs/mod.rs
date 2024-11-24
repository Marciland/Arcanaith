use crate::{objects::Object, scenes::Scene, window::Window, GameEvent};
use ash::Device;
use component::{
    composition::{RenderTarget, TextWithPosition, VisualWithPosition},
    ComponentManager, PositionComponent,
};
use entity::{Entity, EntityManager};
use std::{
    cmp::Ordering,
    time::{Duration, Instant},
};
use system::{PositionSystem, SystemManager};
use winit::event_loop::EventLoopProxy;

pub mod component;
pub mod entity;
pub mod system;

pub struct ECS {
    pub entity_manager: EntityManager,
    pub component_manager: ComponentManager,
    pub system_manager: SystemManager,
}

impl ECS {
    #[must_use]
    pub fn create() -> Self {
        Self {
            entity_manager: EntityManager::new(),
            component_manager: ComponentManager::new(),
            system_manager: SystemManager::create(),
        }
    }

    pub fn initialize(&mut self, window: &Window) {
        self.system_manager.render.initialize(window);
        self.system_manager.resource.initialize(window);
    }

    pub fn process_inputs(
        &mut self,
        current_scene: &Scene,
        event_proxy: &EventLoopProxy<GameEvent>,
    ) {
        self.system_manager.input.process_inputs(
            current_scene,
            &mut self.component_manager,
            &self.system_manager.resource,
            event_proxy,
        );
    }

    pub fn render(&mut self, current_scene: &Scene, window: &mut Window) -> Duration {
        let start_time = Instant::now();

        let entities: Vec<Entity> = current_scene.get_objects().iter().map(Object::id).collect();
        let mut render_targets = Vec::with_capacity(entities.len());

        // collect visual entities
        for (entity, visual) in self.component_manager.visual_storage.iter_mut() {
            // only render current scene
            if !entities.contains(&entity) {
                continue;
            }

            // skip invisible
            if !visual.should_render() {
                continue;
            }

            let Some(position) = self.component_manager.position_storage.get(entity) else {
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
        for (entity, text) in self.component_manager.text_storage.iter_mut() {
            // only render current scene
            if !entities.contains(&entity) {
                continue;
            }

            let Some(position) = self.component_manager.position_storage.get(entity) else {
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

        let textures = self
            .system_manager
            .resource
            .get_render_resources(window, &mut render_targets);

        let player_position: Option<&PositionComponent> = match current_scene {
            Scene::Menu(_) => None,
            Scene::Game(game) => {
                let Some(player) = game.get_player() else {
                    println!("Game without player entity!");
                    return Instant::elapsed(&start_time);
                };
                self.component_manager.position_storage.get(player.id)
            }
        };
        let positions = PositionSystem::get_render_positions(&mut render_targets, player_position);

        window.draw(&self.system_manager.render, &textures, &positions);

        Instant::elapsed(&start_time)
    }

    pub fn destroy_entity(&mut self, entity: Entity, device: &Device) {
        self.component_manager.clear_entity(entity, device);
        self.entity_manager.destroy_entity(entity);
    }

    pub fn destroy(&mut self, device: &Device) {
        self.component_manager.text_storage.destroy(device);
        self.system_manager.destroy(device);
    }
}
