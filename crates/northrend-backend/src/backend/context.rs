use crate::{BackendError, WindowDescriptor, WindowHandle, WindowId};

pub trait BackendContext {
    fn create_window(&mut self, descriptor: WindowDescriptor) -> Result<WindowId, BackendError>;
    fn destroy_window(&mut self, window_id: WindowId);
    fn window_handle(&self, window_id: WindowId) -> Option<WindowHandle>;
    fn request_redraw(&self, window_id: WindowId);
    fn exit(&mut self);
}
