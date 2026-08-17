use northrend_backend::Window;

pub(crate) struct WindowTarget<S> {
    pub(crate) window: Window,
    pub(crate) surface: S,
}

impl<S> WindowTarget<S> {
    pub(crate) const fn new(window: Window, surface: S) -> Self {
        Self { window, surface }
    }
}
