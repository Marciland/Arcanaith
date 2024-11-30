mod app;
mod event;

use crate::{
    constants::{FPS, FULLSCREEN, ICONPATH, TITLE},
    scenes::{MainMenu, Menu, Scene},
    Window,
};
use ecs::ECS;
pub use event::GameEvent;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};
use winit::{
    event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy},
    window::{Fullscreen::Borderless, Icon},
};

pub struct Game {
    window: Option<Window>,
    is_running: Arc<AtomicBool>,
    frame_time: Duration,
    ecs: ECS<GameEvent>,
    event_proxy: EventLoopProxy<GameEvent>,
    current_scene: Scene,
}

impl Game {
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
        let (icon_rgba, icon_width, icon_height) = {
            let image = image::open(ICONPATH)
                .expect("Failed to open icon image!")
                .into_rgba8();
            let (width, height) = image.dimensions();
            let rgba = image.into_raw();
            (rgba, width, height)
        };
        let icon =
            Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to create icon!");
        let mut attributes = winit::window::Window::default_attributes()
            .with_title(TITLE)
            .with_window_icon(Some(icon))
            .with_visible(false);
        if FULLSCREEN {
            attributes = attributes.with_fullscreen(Some(Borderless(None)));
        }
        let inner_window = event_loop
            .create_window(attributes)
            .expect("Failed to create inner window!");

        let texture_count = self.ecs.get_max_texture_count();
        let mut window = Window::create(inner_window, texture_count);

        self.ecs.initialize(window.get_render_context());

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

        let render_context = window_ref.get_render_context();

        render_context.wait_idle();

        self.ecs.destroy(render_context.get_device());

        unsafe { render_context.destroy() };
    }
}
