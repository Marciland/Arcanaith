use crate::ecs::{
    component::{ComponentManager, ComponentStorage, Layer, PositionComponent, VisualComponent},
    entity::{Entity, EntityManager},
    system::ResourceSystem,
};
use glam::Vec3;

pub struct Player {
    pub id: Entity,
    pub state: PlayerState,
}

impl Player {
    pub fn create(
        component_manager: &mut ComponentManager,
        resource_system: &ResourceSystem,
        entity_manager: &mut EntityManager,
    ) -> Self {
        let id = entity_manager.create_entity();
        let state = PlayerState::Idle;
        let start_position = PositionComponent {
            xyz: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            scale: Vec3 {
                x: 0.3,
                y: 0.3,
                z: 1.0,
            },
        };

        component_manager.position_storage.add(id, start_position);

        component_manager
            .visual_storage
            .add(id, state.get_visual(resource_system));

        Self { id, state }
    }

    pub fn change_state(
        &mut self,
        visual_storage: &mut ComponentStorage<VisualComponent>,
        resource_system: &ResourceSystem,
        new_state: PlayerState,
    ) {
        if self.state == new_state {
            return;
        }

        let new_visual: VisualComponent = match new_state {
            PlayerState::Idle => {
                let visual_component = self.state.get_visual(resource_system);
                visual_component.update_animation_speed(0).reset_animation()
            }
            _ => new_state.get_visual(resource_system),
        };

        visual_storage.add(self.id, new_visual);

        self.state = new_state;
    }
}

#[derive(PartialEq)]
pub enum PlayerState {
    Idle,
    WalkingRight,
    WalkingLeft,
    WalkingUp,
    WalkingDown,
}

impl PlayerState {
    pub fn get_visual(&self, resource_system: &ResourceSystem) -> VisualComponent {
        match self {
            PlayerState::Idle => VisualComponent::new(
                vec![resource_system.get_texture_index("player_0")],
                Layer::Game,
                0,
            ),
            PlayerState::WalkingDown => VisualComponent::new(
                vec![
                    resource_system.get_texture_index("player_0"),
                    resource_system.get_texture_index("player_1"),
                    resource_system.get_texture_index("player_2"),
                    resource_system.get_texture_index("player_3"),
                ],
                Layer::Game,
                15,
            ),
            PlayerState::WalkingUp => VisualComponent::new(
                vec![
                    resource_system.get_texture_index("player_4"),
                    resource_system.get_texture_index("player_5"),
                    resource_system.get_texture_index("player_6"),
                    resource_system.get_texture_index("player_7"),
                ],
                Layer::Game,
                15,
            ),
            PlayerState::WalkingLeft => VisualComponent::new(
                vec![
                    resource_system.get_texture_index("player_8"),
                    resource_system.get_texture_index("player_9"),
                    resource_system.get_texture_index("player_10"),
                    resource_system.get_texture_index("player_11"),
                ],
                Layer::Game,
                15,
            ),
            PlayerState::WalkingRight => VisualComponent::new(
                vec![
                    resource_system.get_texture_index("player_12"),
                    resource_system.get_texture_index("player_13"),
                    resource_system.get_texture_index("player_14"),
                    resource_system.get_texture_index("player_15"),
                ],
                Layer::Game,
                15,
            ),
        }
    }
}
