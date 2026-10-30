mod input;
mod runner;

use crate::runner::Runner;
use winit::event_loop::EventLoop;

pub use crate::input::InputManager;
pub use winit::{event::MouseButton, keyboard::KeyCode, window::Window};

pub struct UpdateInfo<'a> {
    pub dt: f64,
    pub input_manager: &'a InputManager,
    pub width: u32,
    pub height: u32,
}

pub trait Application {
    fn new(_: &Window) -> Self;

    fn update(&mut self, _: &UpdateInfo);

    fn fixed_update(&mut self) {}

    fn resize(&mut self, width: u32, height: u32);
}

pub fn run<A: Application>(tick_duration: f64) {
    let mut runner = Runner::<A>::new(tick_duration);
    let event_loop = EventLoop::new().expect("Failed to create event loop");

    event_loop
        .run_app(&mut runner)
        .expect("Application running failed");
}
