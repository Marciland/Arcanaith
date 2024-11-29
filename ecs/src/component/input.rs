use super::{ComponentStorage, Entity};

use winit::event_loop::EventLoopProxy;

pub struct InputComponent<E>
where
    E: 'static,
{
    pub is_active: bool,
    pub activate: fn(event_proxy: &EventLoopProxy<E>) -> (),
    pub next: Option<Entity>,
    pub previous: Option<Entity>,
}

impl<E> ComponentStorage<InputComponent<E>>
where
    E: 'static,
{
    pub fn get_active_entity(&self) -> Option<&Entity> {
        for (entity, input) in &self.components {
            if input.is_active {
                return Some(entity);
            }
        }

        None
    }

    pub fn set_next_of(&mut self, current: &Entity, next: &Entity) {
        let Some(current_input) = self.components.get_mut(current) else {
            return;
        };

        current_input.next = Some(*next);
    }

    pub fn set_previous_of(&mut self, current: &Entity, previous: &Entity) {
        let Some(current_input) = self.components.get_mut(current) else {
            return;
        };

        current_input.previous = Some(*previous);
    }
}
