use crate::{InputState, KeyCode, MouseButton};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindowEventKind {
    CloseRequested,
    Resized { width: u32, height: u32 },
    RedrawRequested,
    Focused(bool),
    KeyboardInput {
        key: KeyCode,
        pressed: bool,
        repeat: bool,
    },
    MouseInput {
        button: MouseButton,
        pressed: bool,
    },
    CursorMoved { x: f64, y: f64 },
    CursorEntered,
    CursorLeft,
    MouseWheelLines { horizontal: f32, vertical: f32 },
    MouseWheelPixels { horizontal: f64, vertical: f64 },
}

impl WindowEventKind {
    pub fn update_input(self, input: &mut InputState) {
        match self {
            Self::Focused(false) => input.focus_lost(),
            Self::KeyboardInput { key, pressed, .. } => input.set_key(key, pressed),
            Self::MouseInput { button, pressed } => input.set_mouse_button(button, pressed),
            Self::CursorMoved { x, y } => input.set_cursor_position(x, y),
            Self::CursorEntered | Self::CursorLeft => input.clear_cursor_position(),
            Self::MouseWheelLines { horizontal, vertical } => {
                input.add_scroll_lines(horizontal, vertical);
            }
            Self::MouseWheelPixels { horizontal, vertical } => {
                input.add_scroll_pixels(horizontal, vertical);
            }
            _ => {}
        }
    }
}
