use winit::event_loop::{ControlFlow, EventLoop};
use witch_s_ascendancy::Game;

fn main() {
    std::panic::set_hook(Box::new(|panic_info| {
        if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            println!("{s:?}");
        }
        if let Some(l) = panic_info.location() {
            println!("{l:?}");
        }
    }));

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut game = Game::default();

    event_loop.run_app(&mut game).unwrap();
}
