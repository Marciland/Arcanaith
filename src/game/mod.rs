mod object;
mod scene;
mod sprites;
use crate::{
    constants::{FPS, FULLSCREEN, ICONPATH, TITLE},
    window::Window,
};
use object::{GameObject, Quad};
use scene::Layer;
pub use scene::Scene;
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Fullscreen::Borderless, Icon, WindowId},
};

pub struct Game {
    window: Option<Rc<RefCell<Window>>>,
    is_running: Arc<AtomicBool>,
    frame_time: Duration,
    current_scene: Option<Scene>,
}

impl Game {
    fn exit(&mut self, event_loop: &ActiveEventLoop) {
        event_loop.exit();
        self.is_running.store(false, Ordering::Release);

        let scene_ref = self.current_scene.as_ref().unwrap();
        unsafe { self.window.as_ref().unwrap().borrow().destroy(scene_ref) }
    }
}

impl Default for Game {
    fn default() -> Self {
        Game {
            window: None,
            is_running: Arc::new(AtomicBool::new(true)),
            frame_time: Duration::from_secs_f64(1.0 / FPS as f64),
            current_scene: None,
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
        let inner_window = event_loop.create_window(attributes).unwrap();
        let window = Rc::new(RefCell::new(Window::create(inner_window)));
        self.current_scene = Some(Scene::create(window.clone()));
        self.window = Some(window);
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
                let render_time = {
                    let scene_ref = self.current_scene.as_ref().unwrap();
                    let mut window_ref = self.window.as_ref().unwrap().borrow_mut();

                    window_ref.render(scene_ref)
                };

                // println!("{:?}", render_time);
                let remaining_time = self.frame_time.saturating_sub(render_time);
                if !remaining_time.is_zero() {
                    thread::sleep(remaining_time)
                }

                {
                    let window_ref = self.window.as_ref().unwrap().borrow();
                    window_ref.request_render();
                }
            }
            _ => (), //println!("event: {:?}", event),
        }
    }
}
