use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::{Duration, Instant},
};

use crate::{
    constants::{BG_FPS, FPS, FULLSCREEN, ICONPATH, TITLE},
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
        unsafe { self.window.as_mut().unwrap().destroy() }
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
        let mut attributes = winit::window::Window::default_attributes()
            .with_title(TITLE)
            .with_window_icon(Some(icon))
            .with_visible(false);
        if FULLSCREEN {
            attributes = attributes.with_fullscreen(Some(Borderless(None)));
        }
        let window = event_loop.create_window(attributes).unwrap();
        self.window = unsafe { Some(Window::new(window)) }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => self.exit(event_loop),
            WindowEvent::RedrawRequested => {
                let start_time = Instant::now();
                unsafe {
                    self.window.as_mut().unwrap().draw_frame();
                }
                let end_time = Instant::now();
                let render_time = end_time - start_time;
                let remaining_time = self.frame_time.saturating_sub(render_time);

                if !remaining_time.is_zero() {
                    thread::sleep(remaining_time)
                }
                self.window.as_mut().unwrap().window.request_redraw();
            }
            WindowEvent::Resized(_size) => {
                let is_minimized = self.window.as_mut().unwrap().window.is_minimized().unwrap();
                if is_minimized {
                    self.frame_time = Duration::from_secs_f64(1.0 / BG_FPS as f64)
                } else {
                    self.frame_time = Duration::from_secs_f64(1.0 / FPS as f64)
                }
                // recreate swapchain
            }
            WindowEvent::Focused(focused) => {
                if focused {
                    self.frame_time = Duration::from_secs_f64(1.0 / FPS as f64)
                } else {
                    self.frame_time = Duration::from_secs_f64(1.0 / BG_FPS as f64)
                }
            }
            _ => (), //println!("event: {:?}", event),
        }
    }
}
