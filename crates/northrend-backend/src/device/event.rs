use crate::InputState;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DeviceEvent {
    MouseMotion { horizontal: f64, vertical: f64 },
}

impl DeviceEvent {
    pub fn update_input(self, input: &mut InputState) {
        match self {
            Self::MouseMotion { horizontal, vertical } => {
                input.add_mouse_motion(horizontal, vertical);
            }
        }
    }
}
