use crate::{
    ecs::component::{composition::RenderTarget, Layer, PositionComponent},
    vulkan::structs::ModelViewProjection,
};
use glam::Mat4;

pub struct PositionSystem;

impl Default for PositionSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl PositionSystem {
    pub fn new() -> Self {
        Self
    }

    pub fn get_render_positions(
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
}
