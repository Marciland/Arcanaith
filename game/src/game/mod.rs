mod app;
mod event;

use crate::{
    constants::FPS,
    scenes::{MainMenu, Menu, Scene},
    Window,
};
use ecs::ECS;
pub use event::GameEvent;
use rendering::RenderAPI;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};
use winit::event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy};

pub struct Game<API: RenderAPI> {
    window: Option<Window<API>>,
    is_running: Arc<AtomicBool>,
    frame_time: Duration,
    ecs: ECS<GameEvent>,
    event_proxy: EventLoopProxy<GameEvent>,
    current_scene: Scene,
}

impl<API: RenderAPI> Game<API> {
    #[must_use]
    pub fn new(event_loop: &EventLoop<GameEvent>) -> Self {
        Self {
            window: None,
            is_running: Arc::new(AtomicBool::new(true)),
            frame_time: Duration::from_secs_f64(1.0 / f64::from(FPS)),
            ecs: ECS::create("res/texture_table.json", "res/fonts"),
            event_proxy: event_loop.create_proxy(),
            current_scene: Scene::None,
        }
    }

    fn initialize(&mut self, event_loop: &ActiveEventLoop) {
        let texture_count = self.ecs.get_max_texture_count();
        let window = Window::create(event_loop, texture_count);

        self.ecs.initialize(&window.render_context);

        self.current_scene = Scene::Menu(Menu::MainMenu(MainMenu::create(&mut self.ecs)));

        self.window = Some(window);
    }

    fn exit(&mut self, event_loop: &ActiveEventLoop) {
        event_loop.exit();
        self.is_running.store(false, Ordering::Release);

        let window_ref = self
            .window
            .as_mut()
            .expect("Failed to get window ref while exiting!");

        window_ref.render_context.wait_idle();

        self.ecs.destroy();

        window_ref.render_context.destroy();
    }
}
