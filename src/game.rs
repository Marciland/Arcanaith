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
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy},
    window::{Fullscreen::Borderless, Icon, WindowId},
};

#[derive(Debug)]
pub enum GameEvent {
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
        window_ref.wait_idle();
        unsafe {
            self.system_manager.destroy(window_ref.get_device());
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
        self.entity_manager.load(
            &self.current_state,
            &mut self.component_manager,
            &self.system_manager.resource,
        );
        self.window = Some(window);
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: GameEvent) {
        match event {
            GameEvent::ExitGame => self.exit(event_loop),

            // can be send from main menu and pause menu
            GameEvent::SettingsMenu => {
                match self.current_state {
                    GameState::MainMenu => {
                        self.entity_manager.clear(&mut self.component_manager);
                    }
                    GameState::_Pause => {
                        self.component_manager.visual_storage.hide_all();
                    }
                    _ => panic!("SettingsMenu event should not have been send!"),
                }

                self.previous_state = Some(self.current_state.clone());
                self.current_state = GameState::Settings;
                self.entity_manager.load(
                    &self.current_state,
                    &mut self.component_manager,
                    &self.system_manager.resource,
                );
            }

            // can be send from settings menu and pause menu
            GameEvent::Back => match self.current_state {
                GameState::_Pause => {
                    self.current_state = GameState::Game;
                    todo!("unhide game and remove settings entities, continue updating")
                }
                GameState::Settings => match self.previous_state {
                    Some(GameState::_Pause) => {
                        todo!("remove settings menu entitites and show pause menu entities")
                    }
                    Some(GameState::MainMenu) => {
                        self.entity_manager.clear(&mut self.component_manager);

                        self.previous_state = None;
                        self.current_state = GameState::MainMenu;

                        self.entity_manager.load(
                            &self.current_state,
                            &mut self.component_manager,
                            &self.system_manager.resource,
                        );
                    }
                    _ => panic!("No previous state when trying to go back!"),
                },
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

            WindowEvent::RedrawRequested => {
                self.system_manager.input.process_inputs(
                    &self.current_state,
                    &mut self.component_manager,
                    &self.event_proxy,
                );

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
                event:
                    KeyEvent {
                        logical_key: key,
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                self.system_manager.input.add_keyboard_input(key);
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

            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Released,
                        ..
                    },
                ..
            }
            | WindowEvent::Moved(_)
            | WindowEvent::Resized(_)
            | WindowEvent::CursorEntered { device_id: _ }
            | WindowEvent::CursorLeft { device_id: _ } => {
                // ignoring key releases for now
                // println!("ignored event: {event:?}")
            }

            _ => println!("unprocessed event: {event:?}"),
        }
    }
}
