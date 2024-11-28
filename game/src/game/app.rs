use super::{
    event::{UserEventHandler, WindowEventHandler},
    Game, GameEvent,
};
use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop,
    window::WindowId,
};

impl ApplicationHandler<GameEvent> for Game {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.initialize(event_loop);
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: GameEvent) {
        match event {
            GameEvent::NewGame => self.load_new_game(),

            GameEvent::ExitGame => self.exit(event_loop),

            GameEvent::SettingsMenu => self.load_settings_menu(),

            GameEvent::MainMenu => self.load_main_menu(),
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
                self.ecs
                    .system_manager
                    .input_system
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

                self.ecs.system_manager.input_system.update_cursor_position(
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
                self.ecs
                    .system_manager
                    .input_system
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
