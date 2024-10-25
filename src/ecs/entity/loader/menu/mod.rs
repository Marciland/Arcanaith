mod main_menu;
mod settings;
use crate::ecs::{
    component::{InputComponent, Layer, PositionComponent, VisualComponent},
    entity::{Entity, EntityLoader, EntityManager},
};
use glam::Vec3;
pub use main_menu::load as load_main_menu;
pub use settings::load as load_settings_menu;

impl<'loading> EntityLoader<'loading> {
    fn create_menu_entity(
        &mut self,
        entity: Entity,
        texture: &str,
        xyz: Vec3,
        scale: Vec3,
        input: Option<InputComponent>,
    ) {
        let layer = match input {
            Some(component) => {
                self.component_manager.input_storage.add(entity, component);
                Layer::Interface
            }
            None => Layer::Background,
        };

        self.component_manager.visual_storage.add(
            entity,
            VisualComponent::new(
                vec![self.resource_system.get_texture_index(texture)],
                layer,
                0,
                0,
            ),
        );

        self.component_manager
            .position_storage
            .add(entity, PositionComponent { xyz, scale });
    }

    fn create_background(&mut self, entity_manager: &mut EntityManager) {
        let background = entity_manager.create_entity();
        self.create_menu_entity(
            background,
            "menu_background",
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.1,
            },
            Vec3 {
                x: 2.0,
                y: 2.0,
                z: 1.0,
            },
            None,
        );
    }

    fn create_title(&mut self, entity_manager: &mut EntityManager) {
        let title = entity_manager.create_entity();
        self.create_menu_entity(
            title,
            "menu_title",
            Vec3 {
                x: 0.0,
                y: -0.8,
                z: 0.0,
            },
            Vec3 {
                x: 1.5,
                y: 0.5,
                z: 1.0,
            },
            None,
        );
    }
}
