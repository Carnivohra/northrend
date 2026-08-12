use northrend_backend::{Backend, BackendApplication};

use crate::WinitError;

pub struct WinitBackend;

impl WinitBackend {
    pub fn new() -> Self {
        Self
    }
}

impl Backend for WinitBackend {
    type Error = WinitError;

    fn run<A>(self, _application: A) -> Result<(), Self::Error>
    where
        A: BackendApplication + 'static,
    {
        todo!()
    }
}
