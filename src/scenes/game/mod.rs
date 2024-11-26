mod overlay;

use crate::{
    ecs::{component::ComponentManager, system::input::InputHandler},
    objects::{Object, Player},
    MouseEvent, ECS,
};
use ash::Device;
use indexmap::IndexSet;
use overlay::Overlay;
use std::collections::HashSet;
use winit::keyboard::Key;

pub struct Game {
    pub objects: Vec<Object>,
}

impl Game {
    pub fn create(ecs: &mut ECS) -> Game {
        let mut objects = Vec::with_capacity(100);

        let overlay = Overlay::create(ecs);
        objects.extend(overlay.objects);

        // TODO background

        let player = Player::create(ecs);
        objects.push(Object::Player(player));

        // TODO spawner

        Game { objects }
    }

    pub fn get_objects(&self) -> &[Object] {
        &self.objects
    }

    pub fn get_player(&self) -> &Player {
        for obj in &self.objects {
            if let Object::Player(player) = obj {
                return player;
            }
        }

        panic!("Game has no Player Object!")
    }

    pub fn destroy(&self, device: &Device, ecs: &mut ECS) {
        for obj in &self.objects {
            ecs.destroy_entity(obj.id(), device);
        }
    }

    pub fn handle_player_events(
        &self,
        keyboard_pressed_inputs: &IndexSet<Key>,
        _active_keyboard_inputs: &HashSet<Key>,
        mouse_inputs: &[MouseEvent],
        _component_manager: &mut ComponentManager,
    ) {
        // TODO player movement
        let _player = self.get_player();

        // TODO skills
        for _key in keyboard_pressed_inputs {}

        // TODO skills / movement?
        for _event in mouse_inputs {}
    }
}

impl InputHandler for Game {
    fn handle_mouse_events(
        &self,
        _events: &[MouseEvent],
        _component_manager: &mut ComponentManager,
        _event_proxy: &winit::event_loop::EventLoopProxy<crate::GameEvent>,
    ) {
        todo!()
    }

    fn handle_key_events(
        &self,
        _pressed_keys: &IndexSet<Key>,
        _component_manager: &mut ComponentManager,
        _event_proxy: &winit::event_loop::EventLoopProxy<crate::GameEvent>,
    ) {
        todo!()
    }
}
