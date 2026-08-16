mod source;

use source::WindowHandleSource;

use std::sync::Arc;

use raw_window_handle::{
    DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, WindowHandle as RawWindowHandle,
};

#[derive(Clone)]
pub struct WindowHandle {
    source: Arc<dyn WindowHandleSource>,
}

impl WindowHandle {
    pub fn new<W>(window: Arc<W>) -> Self
    where
        W: HasDisplayHandle + HasWindowHandle + Send + Sync + 'static,
    {
        Self { source: window }
    }
}

impl HasDisplayHandle for WindowHandle {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        self.source.display_handle()
    }
}

impl HasWindowHandle for WindowHandle {
    fn window_handle(&self) -> Result<RawWindowHandle<'_>, HandleError> {
        self.source.window_handle()
    }
}
