use super::Entity;
use winit::event_loop::EventLoopProxy;

pub(crate) struct InputComponent {
    is_active: bool,
    activate: fn(event_proxy: &EventLoopProxy<GameEvent>) -> (),
    next: Option<Entity>,
    previous: Option<Entity>,
}
