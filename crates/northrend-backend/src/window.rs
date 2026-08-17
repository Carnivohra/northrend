mod descriptor;
pub mod event;
mod handle;
mod id;

pub use descriptor::WindowDescriptor;
pub use event::WindowEvent;
pub use handle::WindowHandle;
pub use id::WindowId;

pub struct Window {
    id: WindowId,
    handle: WindowHandle,
}

impl Window {
    pub const fn new(id: WindowId, handle: WindowHandle) -> Self {
        Self { id, handle }
    }

    pub const fn id(&self) -> WindowId {
        self.id
    }

    pub fn handle(&self) -> WindowHandle {
        self.handle.clone()
    }
}
