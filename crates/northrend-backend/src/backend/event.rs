use crate::window::WindowEvent;

pub enum BackendEvent {
    Resumed,
    Suspended,
    Window(WindowEvent),
    Exiting,
}
