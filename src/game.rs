use crate::{
    constants::{FPS, FULLSCREEN, ICONPATH, TITLE},
    ecs::{ComponentManager, EntityManager, SystemManager},
    Window,
};
use std::{
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
    window: Option<Window>,
    is_running: Arc<AtomicBool>,
    frame_time: Duration,
    entity_manager: EntityManager,
    component_manager: ComponentManager,
    system_manager: SystemManager,
}

impl Game {
    fn exit(&mut self, event_loop: &ActiveEventLoop) {
        event_loop.exit();
        self.is_running.store(false, Ordering::Release);

        unsafe {
            let window_ref = self
                .window
                .as_ref()
                .expect("Failed to get window ref while exiting!");
            window_ref.wait_idle();
            self.system_manager.destroy(window_ref.get_device());
            window_ref.destroy();
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Game {
            window: None,
            is_running: Arc::new(AtomicBool::new(true)),
            frame_time: Duration::from_secs_f64(1.0 / FPS as f64),
            entity_manager: EntityManager::new(),
            component_manager: ComponentManager::new(),
            system_manager: SystemManager::create(),
        }
    }
}

impl ApplicationHandler for Game {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
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

        let texture_count = self.system_manager.resource_system.get_texture_count();
        let window = Window::create(inner_window, texture_count);
        self.system_manager.render_system.initialize(&window);
        self.system_manager.resource_system.initialize(&window);
        self.entity_manager.load_main_menu(
            &mut self.component_manager,
            &self.system_manager.resource_system,
        );
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
                let render_time = self.system_manager.render_system.render(
                    &mut self.component_manager,
                    &self.system_manager.resource_system,
                    self.window
                        .as_mut()
                        .expect("Window was lost while rendering!"),
                );

                // println!("{:?}", render_time);

                let remaining_time = self.frame_time.saturating_sub(render_time);
                if !remaining_time.is_zero() {
                    thread::sleep(remaining_time)
                }

                self.window
                    .as_ref()
                    .expect("Window was lost while rendering!")
                    .request_render();
            }
            _ => (), //println!("event: {:?}", event),
                     // TODO collect input events and process them in the input system
        }
    }
}
