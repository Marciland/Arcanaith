use crate::{
    constants::{FPS, FULLSCREEN, ICONPATH, TITLE},
    ecs::{component::ComponentManager, entity::EntityManager, system::SystemManager},
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

pub enum GameState {
    Menu,
    _Game,
    _Pause,
}

pub struct Game {
    window: Option<Window>,
    is_running: Arc<AtomicBool>,
    frame_time: Duration,
    current_state: GameState,
    entity_manager: EntityManager,
    component_manager: ComponentManager,
    system_manager: SystemManager,
}

impl Game {
    fn exit(&mut self, event_loop: &ActiveEventLoop) {
        event_loop.exit();
        self.is_running.store(false, Ordering::Release);

        let window_ref = self
            .window
            .as_ref()
            .expect("Failed to get window ref while exiting!");
        window_ref.wait_idle();
        unsafe {
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
            frame_time: Duration::from_secs_f64(1.0 / f64::from(FPS)),
            current_state: GameState::Menu,
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

        let texture_count = self.system_manager.resource.get_texture_count();
        let window = Window::create(inner_window, texture_count);
        self.system_manager.render.initialize(&window);
        self.system_manager.resource.initialize(&window);
        self.entity_manager.load(
            &self.current_state,
            &mut self.component_manager,
            &self.system_manager.resource,
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
                self.system_manager
                    .input
                    .process_inputs(&self.current_state);

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
            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } => {
                self.system_manager.input.add_keyboard_input(event);
            }
            WindowEvent::CursorMoved {
                device_id,
                position,
            } => {
                self.system_manager
                    .input
                    .update_cursor_position(device_id, position);
            }
            WindowEvent::MouseInput {
                device_id: _,
                state,
                button,
            } => {
                self.system_manager.input.add_mouse_input(button, state);
            }
            _ => (), //println!("unprocessed event: {:?}", event),
        }
    }
}
