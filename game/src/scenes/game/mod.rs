mod overlay;

use crate::game::GameEvent;

use ecs::{Entity, InputHandler, MouseEvent, ECS};
use indexmap::IndexSet;
use overlay::Overlay;
use winit::{event_loop::EventLoopProxy, keyboard::Key};

pub struct Game {
    pub player_id: Entity,
    pub objects: Vec<Entity>,
}

impl Game {
    pub fn create(ecs: &mut ECS<GameEvent>) -> Self {
        let mut objects = Vec::with_capacity(100);

        objects.extend(Overlay::create(ecs).objects);

        // TODO background

        // objects.push(Player::create(ecs));
        let player = ecs.create_entity();

        // TODO spawner

        Self {
            player_id: player,
            objects,
        }
    }

    pub fn get_objects(&self) -> &[Entity] {
        &self.objects
    }

    pub fn destroy(&self, ecs: &mut ECS<GameEvent>) {
        for obj in &self.objects {
            ecs.destroy_entity(*obj);
        }
    }
}

impl InputHandler<GameEvent> for Game {
    fn handle_mouse_events(
        &self,
        _ecs: &ECS<GameEvent>,
        events: &[MouseEvent],
        _event_proxy: &EventLoopProxy<GameEvent>,
    ) {
        // TODO player movement

        // TODO skills / movement?
        for _event in events {}
    }

    fn handle_key_events(
        &self,
        _ecs: &mut ECS<GameEvent>,
        pressed_keys: &IndexSet<Key>,
        _event_proxy: &EventLoopProxy<GameEvent>,
    ) {
        // TODO player movement

        // TODO skills
        for _key in pressed_keys {}
    }
}
