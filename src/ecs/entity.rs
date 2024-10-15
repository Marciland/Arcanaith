use crate::ecs::{
    component::{Layer, PositionComponent, VisualComponent},
    system::ResourceSystem,
    ComponentManager,
};
use glam::Vec3;
use std::collections::HashSet;

pub type Entity = u32;

pub struct EntityManager {
    next_id: Entity,
    entities: HashSet<Entity>,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            entities: HashSet::new(),
        }
    }

    pub fn load_main_menu(
        &mut self,
        component_manager: &mut ComponentManager,
        resource_system: &ResourceSystem,
    ) {
        let background = self.create_entity();
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

    fn create_entity(&mut self) -> Entity {
        let entity = self.next_id;
        self.entities.insert(entity);
        self.next_id += 1;
        entity
    }

    /*
    fn remove_entity(&mut self, entity: Entity) {
        self.entities.remove(&entity);
    }

    fn is_valid(&self, entity: Entity) -> bool {
        self.entities.contains(&entity)
    }
    */
}
