use crate::{
    ecs::{component::ComponentManager, system::input::InputHandler},
    scenes, MouseEvent,
};
use indexmap::IndexSet;
use std::collections::HashSet;
use winit::keyboard::Key;

impl InputHandler for scenes::Game {
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

impl scenes::Game {
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
