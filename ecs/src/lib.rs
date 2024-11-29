mod component;
mod entity;
mod system;

use component::ComponentManager;
use entity::{Entity, EntityManager, EntityProvider};
use system::{InputHandler, RenderContext, SystemManager};

use ash::Device;
use winit::event_loop::EventLoopProxy;

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

    pub fn initialize<R>(&mut self, renderer: &R)
    where
        R: RenderContext,
    {
        self.system_manager.initialize(renderer);
    }

    pub fn get_max_texture_count(&self) -> u32 {
        self.system_manager.resource_system.get_texture_count()
    }

    pub fn process_inputs<T: InputHandler>(
        &mut self,
        handler: &T,
        event_proxy: &EventLoopProxy<E>,
    ) {
        handler.handle_mouse_events(
            &self.system_manager.input_system.mouse_inputs,
            &mut self.component_manager,
            event_proxy,
        );
        handler.handle_key_events(
            &self.system_manager.input_system.keyboard_pressed_inputs,
            &mut self.component_manager,
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
            let Some(entity_physics) = self.component_manager.physics_storage.get(entity) else {
                continue;
            };

            let Some(entity_position) = self.component_manager.position_storage.get_mut(entity)
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
        self.system_manager.render_system.draw(
            renderer,
            provider,
            &mut self.component_manager.visual_storage,
            &mut self.component_manager.text_storage,
            &self.component_manager.position_storage,
            &mut self.system_manager.resource_system,
        );
    }

    fn destroy_entity(&mut self, entity: Entity, device: &Device) {
        self.component_manager.clear_entity(entity, device);
        self.entity_manager.destroy_entity(entity);
    }

    pub fn destroy(&mut self, device: &Device) {
        self.component_manager.destroy(device);
        self.system_manager.destroy(device);
    }
}
