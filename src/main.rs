use std::panic;
use winit::event_loop::{ControlFlow, EventLoop};
use witch_s_ascendancy::{Game, GameEvent};

fn main() {
    panic::set_hook(Box::new(|panic_info| {
        // print todo! macro
        if let Some(payload) = panic_info.payload().downcast_ref::<&'static str>() {
            println!("{payload}");
        }
        // print except
        else if let Some(payload) = panic_info.payload().downcast_ref::<String>() {
            println!("{payload}");
        }
        // print location
        if let Some(panic_location) = panic_info.location() {
            println!("{panic_location}");
        }
    }));

    let event_loop = EventLoop::<GameEvent>::with_user_event()
        .build()
        .expect("Failed to build custom event loop!");
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut game = Game::new(&event_loop);

    event_loop.run_app(&mut game).expect("Failed to run game!");
}
