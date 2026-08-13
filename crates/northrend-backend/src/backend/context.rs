use crate::{BackendError, WindowDescriptor, WindowId};

pub trait BackendContext {
    fn create_window(&mut self, descriptor: WindowDescriptor) -> Result<WindowId, BackendError>;
}
