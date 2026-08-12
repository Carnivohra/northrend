mod builder;

pub use builder::AppBuilder;
use northrend_backend::Backend;

use crate::Engine;

pub struct App<B = ()> {
    backend: B,
    engine: Engine,
}

impl App<()> {
    pub fn builder() -> AppBuilder<()> {
        AppBuilder::new()
    }
}

impl<B: Backend> App<B> {
    pub fn run(self) -> Result<(), B::Error> {
        self.backend.run(self.engine)
    }
}
