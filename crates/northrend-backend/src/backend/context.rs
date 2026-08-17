use crate::{BackendError, Window, WindowDescriptor};

pub trait BackendContext {
    fn create_window(&mut self, descriptor: WindowDescriptor) -> Result<Window, BackendError>;
    fn destroy_window(&mut self, window: Window);
    fn request_redraw(&self, window: &Window);
    fn exit(&mut self);
}
