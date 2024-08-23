use winit::event_loop::{ControlFlow, EventLoop};
use witch_s_ascendancy::Game;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut game = Game::default();

    event_loop.run_app(&mut game).unwrap();
}
