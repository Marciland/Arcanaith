use crate::{ecs::entity::Entity, GameEvent};
use winit::event_loop::EventLoopProxy;

pub struct InputComponent {
    pub is_active: bool,
    pub activate: fn(event_proxy: &EventLoopProxy<GameEvent>) -> (),
    pub next: Option<Entity>,
    pub previous: Option<Entity>,
}
