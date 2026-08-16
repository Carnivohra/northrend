mod builder;

use builder::AppBuilder;

use northrend_backend::Backend;
use northrend_render::Renderer;

use crate::engine::Engine;

pub struct App<B = (), R = ()> {
    backend: B,
    renderer: R,
}

impl App<(), ()> {
    pub fn builder() -> AppBuilder<(), ()> {
        AppBuilder::new()
    }
}

impl<B: Backend, R: Renderer + 'static> App<B, R> {
    pub fn run(self) -> Result<(), B::Error> {
        self.backend.run(Engine::new(self.renderer))
    }
}
