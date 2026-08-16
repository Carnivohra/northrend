use northrend_backend::Backend;
use northrend_render::Renderer;

use crate::App;

pub struct AppBuilder<B = (), R = ()> {
    backend: B,
    renderer: R,
}

impl AppBuilder<(), ()> {
    pub(super) fn new() -> Self {
        Self {
            backend: (),
            renderer: (),
        }
    }
}

impl<R> AppBuilder<(), R> {
    pub fn backend<B: Backend>(self, backend: B) -> AppBuilder<B, R> {
        AppBuilder {
            backend,
            renderer: self.renderer,
        }
    }
}

impl<B, R> AppBuilder<B, R> {
    pub fn renderer<T: Renderer>(self, renderer: T) -> AppBuilder<B, T> {
        AppBuilder {
            backend: self.backend,
            renderer,
        }
    }
}

impl<B: Backend, R: Renderer> AppBuilder<B, R> {
    pub fn build(self) -> App<B, R> {
        App {
            backend: self.backend,
            renderer: self.renderer,
        }
    }
}
