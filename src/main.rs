use std::panic;
use winit::event_loop::{ControlFlow, EventLoop};
use witch_s_ascendancy::Game;

fn main() {
    panic::set_hook(Box::new(|panic_info| {
        if let Some(payload) = panic_info.payload().downcast_ref::<String>() {
            println!("{payload}");
        }
        if let Some(panic_location) = panic_info.location() {
            println!("{panic_location}");
        }
    }));

    let event_loop = EventLoop::new().expect("Failed to create new event loop!");
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut game = Game::default();

    event_loop.run_app(&mut game).expect("Failed to run game!");
}
