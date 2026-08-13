use northrend_backend::BackendApplication;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId as WinitWindowId},
};

use super::context::WinitBackendContext;

pub(crate) struct WinitApplication<A> {
    application: A,
    windows: Vec<Window>,
    started: bool,
}

impl<A> WinitApplication<A> {
    pub fn new(application: A) -> Self {
        Self {
            application,
            windows: Vec::new(),
            started: false,
        }
    }
}

impl<A: BackendApplication> ApplicationHandler for WinitApplication<A> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let mut context = WinitBackendContext::new(event_loop, &mut self.windows);

        if !self.started {
            self.application.started(&mut context);
            self.started = true;
        }

        self.application.resumed(&mut context);
    }

    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        let mut context = WinitBackendContext::new(event_loop, &mut self.windows);
        self.application.suspended(&mut context);
    }

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: WinitWindowId,
        _event: WindowEvent,
    ) {
        self.application.event();
    }
}
