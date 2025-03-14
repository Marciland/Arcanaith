use crate::{
    scenes::{self, MainMenu, Menu, Scene, SettingsMenu},
    Game,
};

use rendering::RenderAPI;
use std::{thread, time::Instant};

#[derive(Debug)]
pub enum GameEvent {
    NewGame,
    ExitGame,
    SettingsMenu,
    MainMenu,
}

pub trait UserEventHandler {
    fn load_settings_menu(&mut self);
    fn load_main_menu(&mut self);
    fn load_new_game(&mut self);
}

pub trait WindowEventHandler {
    fn redraw_requested(&mut self);
}

impl<API: RenderAPI> UserEventHandler for Game<API> {
    fn load_settings_menu(&mut self) {
        match self.current_scene {
            Scene::Menu(Menu::MainMenu(_)) => self.current_scene.destroy(&mut self.ecs),
            _ => panic!("SettingsMenu event should not have been send!"),
        }

        self.current_scene = Scene::Menu(Menu::SettingsMenu(SettingsMenu::create(&mut self.ecs)));
    }

    fn load_main_menu(&mut self) {
        self.current_scene.destroy(&mut self.ecs);

        self.current_scene = Scene::Menu(Menu::MainMenu(MainMenu::create(&mut self.ecs)));
    }

    fn load_new_game(&mut self) {
        self.current_scene.destroy(&mut self.ecs);

        self.current_scene = Scene::Game(scenes::Game::create(&mut self.ecs));
    }
}

impl<API: RenderAPI> WindowEventHandler for Game<API> {
    fn redraw_requested(&mut self) {
        let start_time = Instant::now();

        self.ecs
            .process_inputs(&self.current_scene, &self.event_proxy);

        self.ecs.update_positions(&self.current_scene);

        let window = self
            .window
            .as_mut()
            .expect("Window was lost before rendering!");

        let minimized = window.is_minimized().unwrap_or(false);
        if !minimized {
            self.ecs
                .render(&mut window.render_context, &self.current_scene);
        }

        let render_time = Instant::elapsed(&start_time);

        println!("{render_time:?}");

        let remaining_time = self.frame_time.saturating_sub(render_time);
        if !remaining_time.is_zero() {
            thread::sleep(remaining_time);
        }

        self.window
            .as_ref()
            .expect("Window was lost after rendering!")
            .request_render();
    }
}
