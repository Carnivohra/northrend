use std::sync::Arc;

use northrend_backend::{
    BackendContext, BackendError, Window, WindowDescriptor, WindowHandle, WindowId,
};
use winit::{
    dpi::PhysicalSize,
    event_loop::ActiveEventLoop,
    window::{Window as WinitWindow, WindowAttributes},
};

pub(super) struct WinitBackendContext<'a> {
    event_loop: &'a ActiveEventLoop,
    windows: &'a mut Vec<Option<Arc<WinitWindow>>>,
}

impl<'a> WinitBackendContext<'a> {
    pub(super) fn new(
        event_loop: &'a ActiveEventLoop,
        windows: &'a mut Vec<Option<Arc<WinitWindow>>>,
    ) -> Self {
        Self {
            event_loop,
            windows,
        }
    }

    fn window(&self, window_id: WindowId) -> Option<&Arc<WinitWindow>> {
        let index = Self::window_index(window_id)?;
        self.windows.get(index)?.as_ref()
    }

    fn window_index(window_id: WindowId) -> Option<usize> {
        window_id.value()
            .checked_sub(1)
            .and_then(|index| usize::try_from(index).ok())
    }
}

impl BackendContext for WinitBackendContext<'_> {
    fn create_window(&mut self, descriptor: WindowDescriptor) -> Result<Window, BackendError> {
        let attributes = WindowAttributes::default()
            .with_title(descriptor.title)
            .with_inner_size(PhysicalSize::new(descriptor.width, descriptor.height))
            .with_resizable(descriptor.resizable)
            .with_visible(descriptor.visible);

        let window = Arc::new(
            self.event_loop.create_window(attributes)
                .map_err(|_| BackendError::WindowCreationFailed)?,
        );

        let window_id = u64::try_from(self.windows.len())
            .ok()
            .and_then(|index| index.checked_add(1))
            .map(WindowId::new)
            .ok_or(BackendError::WindowCreationFailed)?;

        let window_handle = WindowHandle::new(window.clone());

        self.windows.push(Some(window));
        Ok(Window::new(window_id, window_handle))
    }

    fn destroy_window(&mut self, window: Window) {
        let Some(index) = Self::window_index(window.id())
        else {
            return;
        };

        if let Some(window) = self.windows.get_mut(index) {
            *window = None;
        }
    }

    fn request_redraw(&self, window: &Window) {
        if let Some(window) = self.window(window.id()) {
            window.request_redraw();
        }
    }

    fn exit(&mut self) {
        self.event_loop.exit();
    }
}
