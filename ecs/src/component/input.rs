use super::Entity;
use winit::event_loop::EventLoopProxy;

pub(crate) struct InputComponent<E>
where
    E: 'static,
{
    is_active: bool,
    pub activate: fn(event_proxy: &EventLoopProxy<E>) -> (),
    next: Option<Entity>,
    previous: Option<Entity>,
}
