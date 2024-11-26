use crate::{
    ecs::system::PhysicsSystem,
    scenes::{self, MainMenu, Menu, Scene, SettingsMenu},
    Game,
};
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

impl UserEventHandler for Game {
    fn load_settings_menu(&mut self) {
        let device_ref = self
            .window
            .as_ref()
            .expect("Failed to get window while loading settings menu!")
            .get_device();

        match self.current_scene {
            Scene::Menu(Menu::MainMenu(_)) => self.current_scene.destroy(device_ref, &mut self.ecs),
            _ => panic!("SettingsMenu event should not have been send!"),
        }

        self.current_scene = Scene::Menu(Menu::SettingsMenu(SettingsMenu::create(&mut self.ecs)));
    }

    fn load_main_menu(&mut self) {
        let device_ref = self
            .window
            .as_ref()
            .expect("Failed to get window while loading main menu!")
            .get_device();

        self.current_scene.destroy(device_ref, &mut self.ecs);

        self.current_scene = Scene::Menu(Menu::MainMenu(MainMenu::create(&mut self.ecs)));
    }

    fn load_new_game(&mut self) {
        let device_ref = self
            .window
            .as_ref()
            .expect("Failed to get window while starting new game!")
            .get_device();

        self.current_scene.destroy(device_ref, &mut self.ecs);

        self.current_scene = Scene::Game(scenes::Game::create(&mut self.ecs));
    }
}

impl WindowEventHandler for Game {
    fn redraw_requested(&mut self) {
        let start_time = Instant::now();

        self.ecs.system_manager.input_system.process_inputs(
            &self.current_scene,
            &mut self.ecs.component_manager,
            &self.event_proxy,
        );

        PhysicsSystem::update_positions(&self.current_scene, &mut self.ecs.component_manager);

        self.ecs.system_manager.render_system.draw(
            self.window
                .as_mut()
                .expect("Window was lost before rendering!"),
            &self.current_scene,
            &mut self.ecs.component_manager.visual_storage,
            &mut self.ecs.component_manager.text_storage,
            &self.ecs.component_manager.position_storage,
            &mut self.ecs.system_manager.resource_system,
        );

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
