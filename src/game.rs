use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

use crate::{
    constants::{FPS, ICONPATH, TITLE},
    window::Window,
};

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Fullscreen::Borderless, Icon, WindowId},
};

pub struct Game {
    window: Option<Window>,
    is_running: Arc<AtomicBool>,
    frame_time: Duration,
}

impl Game {
    fn exit(&mut self, event_loop: &ActiveEventLoop) {
        event_loop.exit();
        self.is_running.store(false, Ordering::Release);
        self.window.as_mut().unwrap().destroy();
    }
}

impl Default for Game {
    fn default() -> Self {
        Game {
            window: None,
            is_running: Arc::new(AtomicBool::new(true)),
            frame_time: Duration::from_secs_f64(1.0 / FPS as f64),
        }
    }
}

impl ApplicationHandler for Game {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let (icon_rgba, icon_width, icon_height) = {
            let image = image::open(ICONPATH).unwrap().into_rgba8();
            let (width, height) = image.dimensions();
            let rgba = image.into_raw();
            (rgba, width, height)
        };
        let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();
        let attributes = winit::window::Window::default_attributes()
            .with_title(TITLE)
            .with_window_icon(Some(icon))
            .with_fullscreen(Some(Borderless(None)));
        let window = event_loop.create_window(attributes).unwrap();
        self.window = Some(Window::new(window))
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => self.exit(event_loop),
            WindowEvent::RedrawRequested => println!("rendering"),
            _ => (), //println!("event: {:?}", event),
        }
    }
}
