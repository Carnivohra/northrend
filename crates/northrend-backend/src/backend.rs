mod event;

pub use event::BackendEvent;

use crate::Window;

pub trait Backend {
    type Window: Window;

    fn create_window(&self) -> Self::Window;
    fn poll_events(&self) -> Vec<BackendEvent>;
}
