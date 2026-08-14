mod kind;

pub use kind::WindowEventKind;

use crate::WindowId;

pub struct WindowEvent {
    pub window_id: WindowId,
    pub kind: WindowEventKind,
}
