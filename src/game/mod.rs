mod event;
use crate::{
    constants::{FPS, FULLSCREEN, ICONPATH, TITLE},
    ecs::{component::ComponentManager, entity::EntityManager, system::SystemManager},
    scenes::create_main_menu,
    Window,
};
use event::{UserEventHandler, WindowEventHandler};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy},
    window::{Fullscreen::Borderless, Icon, WindowId},
};

#[derive(Debug)]
pub enum GameEvent {
    NewGame,
    ExitGame,
    SettingsMenu,
    Back,
}

#[derive(PartialEq, Clone)]
pub enum GameState {
    MainMenu,
    Settings,
    Game,
    _Pause,
}

pub struct Game {
    window: Option<Window>,
    is_running: Arc<AtomicBool>,
    frame_time: Duration,
    previous_state: Option<GameState>,
    current_state: GameState,
    entity_manager: EntityManager,
    component_manager: ComponentManager,
    system_manager: SystemManager,
    event_proxy: EventLoopProxy<GameEvent>,
}

impl Game {
    #[must_use]
    pub fn new(event_loop: &EventLoop<GameEvent>) -> Self {
        Self {
            window: None,
            is_running: Arc::new(AtomicBool::new(true)),
            frame_time: Duration::from_secs_f64(1.0 / f64::from(FPS)),
            previous_state: None,
            current_state: GameState::MainMenu,
            entity_manager: EntityManager::new(),
            component_manager: ComponentManager::new(),
            system_manager: SystemManager::create(),
            event_proxy: event_loop.create_proxy(),
        }
    }

    fn exit(&mut self, event_loop: &ActiveEventLoop) {
        event_loop.exit();
        self.is_running.store(false, Ordering::Release);

        let window_ref = self
            .window
            .as_ref()
            .expect("Failed to get window ref while exiting!");
        let device_ref = window_ref.get_device();

        window_ref.wait_idle();
        self.component_manager.text_storage.destroy(device_ref);
        unsafe {
            self.system_manager.destroy(device_ref);
            window_ref.destroy();
        }
    }
}

impl ApplicationHandler<GameEvent> for Game {
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

        create_main_menu(
            &mut self.component_manager,
            &self.system_manager.resource,
            &mut self.entity_manager,
        );

        self.window = Some(window);
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: GameEvent) {
        match event {
            GameEvent::NewGame => self.start_new_game(),

            GameEvent::ExitGame => self.exit(event_loop),

            GameEvent::SettingsMenu => self.load_settings_menu(),

            #[allow(clippy::match_wildcard_for_single_variants)]
            GameEvent::Back => match self.current_state {
                GameState::MainMenu => {
                    self.exit(event_loop);
                    todo!("ask if user really wants to exit");
                }
                GameState::_Pause => self.back_from_pause(),
                GameState::Settings => {
                    self.back_from_settings();
                    todo!("ask if user wants to save settings");
                }
                _ => panic!("Back event should not have been send!"),
            },
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => self.exit(event_loop),

            WindowEvent::RedrawRequested => self.redraw_requested(),

            WindowEvent::KeyboardInput {
                event,
                is_synthetic: false,
                ..
            } => {
                self.system_manager
                    .input
                    .update_keyboard_input(event.state, event.logical_key);
            }

            WindowEvent::CursorMoved {
                device_id,
                position,
            } => {
                let window_ref = self
                    .window
                    .as_ref()
                    .expect("Window was lost while updating cursor position!");

                self.system_manager.input.update_cursor_position(
                    device_id,
                    position,
                    window_ref.get_current_size(),
                );
            }

            WindowEvent::MouseInput {
                device_id,
                state,
                button,
            } => {
                self.system_manager
                    .input
                    .add_mouse_input(device_id, button, state);
            }

            WindowEvent::Moved(_)
            | WindowEvent::Resized(_)
            | WindowEvent::CursorEntered { device_id: _ }
            | WindowEvent::CursorLeft { device_id: _ } => (),

            _ => println!("unprocessed event: {event:?}"),
        }
    }
}
