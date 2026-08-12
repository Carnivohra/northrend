use northrend_backend::Backend;

use crate::{App, Engine};

pub struct AppBuilder<B = ()> {
    backend: B,
}

impl AppBuilder<()> {
    pub(super) fn new() -> Self {
        Self { backend: () }
    }

    pub fn backend<B: Backend>(self, backend: B) -> AppBuilder<B> {
        AppBuilder { backend }
    }
}

impl<B: Backend> AppBuilder<B> {
    pub fn build(self) -> App<B> {
        App {
            backend: self.backend,
            engine: Engine::new(),
        }
    }
}
