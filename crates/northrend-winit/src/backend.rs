mod application;
mod context;

use northrend_backend::{Backend, BackendApplication};
use winit::event_loop::EventLoop;

use crate::{WinitError, backend::application::WinitApplication};

pub struct WinitBackend;

impl WinitBackend {
    pub fn new() -> Self {
        Self
    }
}

impl Backend for WinitBackend {
    type Error = WinitError;

    fn run<A>(self, application: A) -> Result<(), Self::Error>
    where
        A: BackendApplication + 'static,
    {
        let event_loop = EventLoop::new()?;
        let mut application = WinitApplication::new(application);
        event_loop.run_app(&mut application)?;
        Ok(())
    }
}
