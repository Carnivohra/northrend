use northrend_backend::{
    BackendApplication, WindowId,
    window::{WindowEvent, event::WindowEventKind},
};
use winit::{
    application::ApplicationHandler,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId as WinitWindowId},
};

use super::context::WinitBackendContext;

pub(crate) struct WinitApplication<A> {
    application: A,
    windows: Vec<Option<Window>>,
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
        event_loop: &ActiveEventLoop,
        window_id: WinitWindowId,
        event: winit::event::WindowEvent,
    ) {
        let kind = match event {
            winit::event::WindowEvent::CloseRequested => WindowEventKind::CloseRequested,
            winit::event::WindowEvent::Resized(size) => WindowEventKind::Resized {
                width: size.width,
                height: size.height,
            },
            winit::event::WindowEvent::RedrawRequested => WindowEventKind::RedrawRequested,
            _ => return,
        };

        let Some(window_id) = self
            .windows
            .iter()
            .position(|window| {
                window
                    .as_ref()
                    .is_some_and(|window| window.id() == window_id)
            })
            .and_then(|index| u64::try_from(index).ok())
            .and_then(|index| index.checked_add(1))
            .map(WindowId::new)
        else {
            return;
        };

        let mut context = WinitBackendContext::new(event_loop, &mut self.windows);
        let event = WindowEvent { window_id, kind };
        self.application.window_event(&mut context, event);
    }
}
