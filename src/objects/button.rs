use crate::{
    ecs::{
        component::{
            ComponentManager, InputComponent, PositionComponent, TextComponent, VisualComponent,
        },
        entity::Entity,
    },
    objects::Content,
    GameEvent, ECS,
};
use glam::Vec2;
use winit::event_loop::EventLoopProxy;

pub struct Button {
    pub id: Entity,
}

impl Button {
    pub fn set_next(&self, next: &Button, component_manager: &mut ComponentManager) {
        if let Some(current_input) = component_manager.input_storage.get_mut(self.id) {
            current_input.next = Some(next.id);
        }
    }

    pub fn set_previous(&self, previous: &Button, component_manager: &mut ComponentManager) {
        if let Some(current_input) = component_manager.input_storage.get_mut(self.id) {
            current_input.previous = Some(previous.id);
        }
    }
}

impl ECS {
    pub fn new_button(
        &mut self,
        position: Vec2,
        size: Vec2,
        content: Content,
        is_focused: bool,
        callback: fn(&EventLoopProxy<GameEvent>) -> (),
    ) -> Button {
        let id = self.entity_manager.create_entity();

        self.component_manager.position_storage.add(
            id,
            PositionComponent {
                xyz: position.extend(0.0),
                scale: size.extend(1.0),
            },
        );

        match content {
            Content::Image { name, layer } => {
                self.component_manager.visual_storage.add(
                    id,
                    VisualComponent::new(
                        vec![self.system_manager.resource_system.get_texture_index(name)],
                        layer,
                        0,
                    ),
                );
            }
            Content::Text(content) => {
                self.component_manager
                    .text_storage
                    .add(id, TextComponent::create(content));
            }
        }

        self.component_manager.input_storage.add(
            id,
            InputComponent {
                is_active: is_focused,
                activate: callback,
                next: None,
                previous: None,
            },
        );

        Button { id }
    }
}
