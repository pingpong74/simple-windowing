use std::time::{Duration, Instant};

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    window::{Window, WindowAttributes},
};

use crate::{input::InputManager, *};

pub(crate) struct Runner<A: Application> {
    app: Option<A>,
    window: Option<Window>,
    input_manager: Option<InputManager>,

    last_frame: Instant,
    accumulator: Duration,
    tick_duration: Duration,

    dt: f64,
    alpha: f64,
}

impl<A: Application> Runner<A> {
    pub(crate) fn new(tick_duration: f64) -> Self {
        Self {
            app: None,
            window: None,
            input_manager: None,

            last_frame: Instant::now(),
            accumulator: Duration::new(0, 0),
            tick_duration: Duration::from_secs_f64(tick_duration),

            dt: 0.0,
            alpha: 0.0,
        }
    }
}

impl<A: Application> ApplicationHandler for Runner<A> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = event_loop
            .create_window(
                WindowAttributes::default()
                    .with_inner_size(winit::dpi::LogicalSize::new(1280.0, 720.0)),
            )
            .expect("Failed to create window");

        window
            .set_cursor_grab(winit::window::CursorGrabMode::Locked)
            .expect(":(");
        window.set_cursor_visible(false);

        self.input_manager = Some(InputManager::new());
        self.app = Some(Application::new(&window));
        self.window = Some(window);
    }

    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        let now = Instant::now();
        let dt = now.duration_since(self.last_frame);

        self.last_frame = now;
        self.accumulator += dt;

        let app = self.app.as_mut().unwrap();

        while self.accumulator >= self.tick_duration {
            app.fixed_update();
            self.accumulator -= self.tick_duration;
        }

        self.dt = dt.as_secs_f64();
        self.alpha = self.accumulator.as_secs_f64() / self.tick_duration.as_secs_f64();

        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if self.window.is_none() || self.app.is_none() || self.input_manager.is_none() {
            return;
        }

        let app = self.app.as_mut().unwrap();
        let input_manager = self.input_manager.as_mut().unwrap();
        let window = self.window.as_ref().unwrap();

        let size = window.inner_size();

        input_manager.handle_window_event(&event);

        match event {
            WindowEvent::RedrawRequested => {
                app.update(&UpdateInfo {
                    dt: self.dt,
                    input_manager,
                    width: size.width,
                    height: size.height,
                });
                input_manager.poll();
            }
            WindowEvent::Resized(size) => {
                app.resize(size.width, size.height);
            }
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            _ => {}
        }
    }

    fn device_event(
        &mut self,
        _: &winit::event_loop::ActiveEventLoop,
        _: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        if let Some(app) = &mut self.input_manager {
            app.handle_device_event(&event);
        }
    }
}
