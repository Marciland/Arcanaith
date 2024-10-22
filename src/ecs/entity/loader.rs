use crate::ecs::{
    component::{ComponentManager, Layer, PositionComponent, VisualComponent},
    entity::EntityManager,
    system::ResourceSystem,
};
use glam::Vec3;

pub struct EntityLoader;

impl EntityLoader {
    pub fn load_main_menu(
        entity_manager: &mut EntityManager,
        component_manager: &mut ComponentManager,
        resource_system: &ResourceSystem,
    ) {
        let background = entity_manager.create_entity();
        component_manager.visual_storage.add(
            background,
            VisualComponent::new(
                vec![resource_system.get_texture_index("main_menu_background")],
                Layer::Background,
                0,
                0,
            ),
        );
        component_manager.position_storage.add(
            background,
            PositionComponent {
                xyz: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                scale: Vec3 {
                    x: 2.0,
                    y: 2.0,
                    z: 1.0,
                },
            },
        );
    }

    pub fn load_game(
        _entity_manager: &mut EntityManager,
        _component_manager: &mut ComponentManager,
        _resource_system: &ResourceSystem,
    ) {
    }

    pub fn load_pause_menu(
        _entity_manager: &mut EntityManager,
        _component_manager: &mut ComponentManager,
        _resource_system: &ResourceSystem,
    ) {
    }
}
