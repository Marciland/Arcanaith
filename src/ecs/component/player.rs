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
