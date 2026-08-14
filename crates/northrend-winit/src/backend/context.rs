use northrend_backend::{BackendContext, BackendError, WindowDescriptor, WindowId};
use winit::{
    dpi::PhysicalSize,
    event_loop::ActiveEventLoop,
    window::{Window, WindowAttributes},
};

pub(super) struct WinitBackendContext<'a> {
    event_loop: &'a ActiveEventLoop,
    windows: &'a mut Vec<Option<Window>>,
}

impl<'a> WinitBackendContext<'a> {
    pub fn new(event_loop: &'a ActiveEventLoop, windows: &'a mut Vec<Option<Window>>) -> Self {
        Self {
            event_loop,
            windows,
        }
    }
}

impl BackendContext for WinitBackendContext<'_> {
    fn create_window(&mut self, descriptor: WindowDescriptor) -> Result<WindowId, BackendError> {
        let attributes = WindowAttributes::default()
            .with_title(descriptor.title)
            .with_inner_size(PhysicalSize::new(descriptor.width, descriptor.height))
            .with_resizable(descriptor.resizable)
            .with_visible(descriptor.visible);

        let window = self
            .event_loop
            .create_window(attributes)
            .map_err(|_| BackendError::WindowCreationFailed)?;

        let window_id = u64::try_from(self.windows.len())
            .ok()
            .and_then(|index| index.checked_add(1))
            .map(WindowId::new)
            .ok_or(BackendError::WindowCreationFailed)?;
        self.windows.push(Some(window));

        Ok(window_id)
    }

    fn destroy_window(&mut self, window_id: WindowId) {
        let Some(index) = window_id
            .value()
            .checked_sub(1)
            .and_then(|index| usize::try_from(index).ok())
        else {
            return;
        };

        if let Some(window) = self.windows.get_mut(index) {
            *window = None;
        }
    }

    fn exit(&mut self) {
        self.event_loop.exit();
    }
}
