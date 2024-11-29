mod overlay;

use crate::game::GameEvent;

use ash::Device;
use ecs::{Entity, InputHandler, MouseEvent, ECS};
use indexmap::IndexSet;
use overlay::Overlay;
use winit::{event_loop::EventLoopProxy, keyboard::Key};

pub struct Game {
    pub objects: Vec<Entity>,
}

impl Game {
    pub fn create(ecs: &mut ECS<GameEvent>) -> Game {
        let mut objects = Vec::with_capacity(100);

        objects.extend(Overlay::create(ecs).objects);

        // TODO background

        // objects.push(Player::create(ecs));

        // TODO spawner

        Game { objects }
    }

    pub fn get_objects(&self) -> &[Entity] {
        &self.objects
    }

    pub fn destroy(&self, device: &Device, ecs: &mut ECS<GameEvent>) {
        for obj in &self.objects {
            ecs.destroy_entity(obj, device);
        }
    }
}

impl InputHandler for Game {
    fn handle_mouse_events<GameEvent>(
        &self,
        ecs: &ECS<GameEvent>,
        events: &[MouseEvent],
        _event_proxy: &EventLoopProxy<GameEvent>,
    ) {
        // TODO player movement
        let _player = self.get_player();
        // TODO skills / movement?
        for _event in events {}
    }

    fn handle_key_events<GameEvent>(
        &self,
        ecs: &ECS<GameEvent>,
        pressed_keys: &IndexSet<Key>,
        _event_proxy: &EventLoopProxy<GameEvent>,
    ) {
        // TODO player movement
        let _player = self.get_player();
        // TODO skills
        for _key in pressed_keys {}
    }
}
