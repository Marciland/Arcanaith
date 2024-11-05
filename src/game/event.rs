use super::{Game, GameState};
use std::thread;

pub trait UserEventHandler {
    fn load_settings_menu(&mut self);
    fn back_from_pause(&mut self);
    fn back_from_settings(&mut self);
}

pub trait WindowEventHandler {
    fn redraw_requested(&mut self);
}

impl UserEventHandler for Game {
    fn load_settings_menu(&mut self) {
        // can be send from main menu and pause menu
        match self.current_state {
            GameState::MainMenu => {
                self.entity_manager.clear(&mut self.component_manager);
            }
            GameState::_Pause => {
                self.component_manager.visual_storage.hide_all();
            }
            _ => panic!("SettingsMenu event should not have been send!"),
        }

        self.previous_state = Some(self.current_state.clone());
        self.current_state = GameState::Settings;
        self.entity_manager.load(
            &self.current_state,
            &mut self.component_manager,
            &self.system_manager.resource,
        );
    }
    fn back_from_pause(&mut self) {
        self.current_state = GameState::Game;
        todo!("unhide game and remove settings entities, continue updating")
    }
    fn back_from_settings(&mut self) {
        match self.previous_state {
            Some(GameState::_Pause) => {
                todo!("remove settings menu entitites and show pause menu entities")
            }
            Some(GameState::MainMenu) => {
                self.entity_manager.clear(&mut self.component_manager);

                self.previous_state = None;
                self.current_state = GameState::MainMenu;

                self.entity_manager.load(
                    &self.current_state,
                    &mut self.component_manager,
                    &self.system_manager.resource,
                );
            }
            _ => panic!("No previous state when trying to go back!"),
        }
    }
}

impl WindowEventHandler for Game {
    fn redraw_requested(&mut self) {
        self.system_manager.input.process_inputs(
            &self.current_state,
            &mut self.component_manager,
            &self.event_proxy,
        );

        let render_time = self.system_manager.render.draw(
            &mut self.component_manager,
            &self.system_manager.resource,
            self.window
                .as_mut()
                .expect("Window was lost while rendering!"),
        );

        // println!("{:?}", render_time);

        let remaining_time = self.frame_time.saturating_sub(render_time);
        if !remaining_time.is_zero() {
            thread::sleep(remaining_time);
        }

        self.window
            .as_ref()
            .expect("Window was lost while rendering!")
            .request_render();
    }
}
