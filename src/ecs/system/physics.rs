use crate::{ecs::component::ComponentManager, scenes::Scene};

pub struct PhysicsSystem;

impl Default for PhysicsSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl PhysicsSystem {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update_positions(current_scene: &Scene, component_manager: &mut ComponentManager) {
        for obj in current_scene.get_objects() {
            let entity = obj.id();

            let Some(entity_physics) = component_manager.physics_storage.get(entity) else {
                continue;
            };

            let Some(entity_position) = component_manager.position_storage.get_mut(entity) else {
                continue;
            };

            entity_position.xyz += entity_physics.velocity;
        }
    }
}
