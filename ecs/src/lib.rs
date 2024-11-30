mod component;
mod entity;
mod system;

use component::ComponentManager;
use entity::EntityManager;
use system::{MouseHandler, MousePosition, RenderSystem, SystemManager};

use ash::{vk::Extent2D, Device};
use winit::{
    dpi::PhysicalPosition,
    event::{DeviceId, ElementState, MouseButton},
    event_loop::EventLoopProxy,
    keyboard::Key,
};

pub use component::{
    Component, ImageData, InputComponent, Layer, PhysicsComponent, PositionComponent,
    TextComponent, TextContent, VisualComponent, MVP,
};
pub use entity::{Entity, EntityProvider};
pub use system::{InputHandler, MouseEvent, RenderContext};

pub struct ECS<E>
where
    E: 'static,
{
    entity_manager: EntityManager,
    component_manager: ComponentManager<E>,
    system_manager: SystemManager,
}

impl<E> ECS<E>
where
    E: 'static,
{
    pub fn create(texture_path: &str, font_path: &str) -> Self {
        Self {
            entity_manager: EntityManager::default(),
            component_manager: ComponentManager::create(),
            system_manager: SystemManager::create(texture_path, font_path),
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        self.entity_manager.create_entity()
    }

    pub fn initialize<R>(&mut self, renderer: &R)
    where
        R: RenderContext,
    {
        self.system_manager.initialize(renderer);
    }

    pub fn add_component(&mut self, entity: Entity, component: Component<E>) {
        match component {
            Component::Position(position_component) => self
                .component_manager
                .position_storage
                .add(entity, position_component),
            Component::Visual(visual_component) => self
                .component_manager
                .visual_storage
                .add(entity, visual_component),
            Component::Text(text_component) => self
                .component_manager
                .text_storage
                .add(entity, text_component),
            Component::Input(input_component) => self
                .component_manager
                .input_storage
                .add(entity, input_component),
            Component::Physics(physics_component) => self
                .component_manager
                .physics_storage
                .add(entity, physics_component),
        }
    }

    pub fn get_max_texture_count(&self) -> u32 {
        self.system_manager.resource_system.get_texture_count()
    }

    pub fn get_texture_index(&self, texture_name: &str) -> usize {
        self.system_manager
            .resource_system
            .get_texture_index(texture_name)
    }

    pub fn get_active_entity(&self) -> Option<&Entity> {
        self.component_manager.input_storage.get_active_entity()
    }

    pub fn set_next_of(&mut self, current: &Entity, next: &Entity) {
        self.component_manager
            .input_storage
            .set_next_of(current, next);
    }

    pub fn set_previous_of(&mut self, current: &Entity, previous: &Entity) {
        self.component_manager
            .input_storage
            .set_previous_of(current, previous);
    }

    pub fn set_next_active(&mut self, currently_active: Entity) {
        self.system_manager
            .input_system
            .set_next_entity_to_active(&mut self.component_manager, currently_active);
    }

    pub fn set_previous_active(&mut self, currently_active: Entity) {
        self.system_manager
            .input_system
            .set_previous_entity_to_active(&mut self.component_manager, currently_active);
    }

    pub fn position_matches_entity(&self, position: &MousePosition, entity: &Entity) -> bool {
        self.system_manager.input_system.entity_was_clicked(
            &self.component_manager,
            position,
            entity,
        )
    }

    pub fn activate_entity(&self, entity: &Entity, event_proxy: &EventLoopProxy<E>) {
        if let Some(active_input) = self.component_manager.input_storage.get(*entity) {
            (active_input.activate)(event_proxy)
        }
    }

    pub fn update_keyboard_input(&mut self, state: ElementState, key: Key) {
        self.system_manager
            .input_system
            .update_keyboard_input(state, key);
    }

    pub fn update_cursor_position(
        &mut self,
        id: DeviceId,
        position: PhysicalPosition<f64>,
        window_size: Extent2D,
    ) {
        self.system_manager
            .input_system
            .update_cursor_position(id, position, window_size);
    }

    pub fn add_mouse_input(
        &mut self,
        device_id: DeviceId,
        mouse_button: MouseButton,
        state: ElementState,
    ) {
        self.system_manager
            .input_system
            .add_mouse_input(device_id, mouse_button, state);
    }

    pub fn process_inputs<T: InputHandler<E>>(
        &mut self,
        handler: &T,
        event_proxy: &EventLoopProxy<E>,
    ) {
        handler.handle_mouse_events(
            self,
            &self.system_manager.input_system.mouse_inputs,
            event_proxy,
        );
        handler.handle_key_events(
            self,
            &self
                .system_manager
                .input_system
                .keyboard_pressed_inputs
                .clone(),
            event_proxy,
        );

        // clear each frame
        self.system_manager.input_system.mouse_inputs.clear();
        self.system_manager
            .input_system
            .keyboard_pressed_inputs
            .clear();
    }

    pub fn update_positions<P>(&mut self, provider: &P)
    where
        P: EntityProvider,
    {
        for entity in provider.get_entities() {
            let Some(entity_physics) = self.component_manager.physics_storage.get(*entity) else {
                continue;
            };

            let Some(entity_position) = self.component_manager.position_storage.get_mut(*entity)
            else {
                continue;
            };

            entity_position.xyz += entity_physics.velocity;
        }
    }

    pub fn render<R, P>(&mut self, renderer: &mut R, provider: &P)
    where
        P: EntityProvider,
        R: RenderContext,
    {
        RenderSystem::draw(
            renderer,
            provider,
            &mut self.component_manager.visual_storage,
            &mut self.component_manager.text_storage,
            &self.component_manager.position_storage,
            &mut self.system_manager.resource_system,
        );
    }

    pub fn destroy_entity(&mut self, entity: Entity, device: &Device) {
        self.component_manager.clear_entity(entity, device);
        self.entity_manager.destroy_entity(entity);
    }

    pub fn destroy(&mut self, device: &Device) {
        self.component_manager.destroy(device);
        self.system_manager.destroy(device);
    }
}
