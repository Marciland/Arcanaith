use crate::{ecs::entity::Entity, GameEvent};
use winit::event_loop::EventLoopProxy;

pub struct InputComponent {
    pub is_active: bool,
    pub next: Entity,
    pub activate: fn(event_proxy: &EventLoopProxy<GameEvent>) -> (),
}
