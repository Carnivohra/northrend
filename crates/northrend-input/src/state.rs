use crate::{KeyCode, MouseButton};

#[derive(Debug, Default)]
pub struct InputState {
    keys_down: u128,
    keys_pressed: u128,
    keys_released: u128,
    mouse_buttons_down: u128,
    mouse_buttons_pressed: u128,
    mouse_buttons_released: u128,
    cursor_position: Option<(f64, f64)>,
    cursor_delta: (f64, f64),
    mouse_motion: (f64, f64),
    scroll_lines: (f32, f32),
    scroll_pixels: (f64, f64),
}

impl InputState {
    pub fn is_key_down(&self, key: KeyCode) -> bool {
        self.keys_down & key_bit(key) != 0
    }

    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.keys_pressed & key_bit(key) != 0
    }

    pub fn is_key_released(&self, key: KeyCode) -> bool {
        self.keys_released & key_bit(key) != 0
    }

    pub fn is_mouse_button_down(&self, button: MouseButton) -> bool {
        mouse_button_bit(button)
            .is_some_and(|bit| self.mouse_buttons_down & bit != 0)
    }

    pub fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        mouse_button_bit(button)
            .is_some_and(|bit| self.mouse_buttons_pressed & bit != 0)
    }

    pub fn is_mouse_button_released(&self, button: MouseButton) -> bool {
        mouse_button_bit(button)
            .is_some_and(|bit| self.mouse_buttons_released & bit != 0)
    }

    pub const fn cursor_position(&self) -> Option<(f64, f64)> {
        self.cursor_position
    }

    pub const fn cursor_delta(&self) -> (f64, f64) {
        self.cursor_delta
    }

    pub const fn mouse_motion(&self) -> (f64, f64) {
        self.mouse_motion
    }

    pub const fn scroll_lines(&self) -> (f32, f32) {
        self.scroll_lines
    }

    pub const fn scroll_pixels(&self) -> (f64, f64) {
        self.scroll_pixels
    }

    pub fn set_key(&mut self, key: KeyCode, pressed: bool) {
        update_button_state(
            &mut self.keys_down,
            &mut self.keys_pressed,
            &mut self.keys_released,
            Some(key_bit(key)),
            pressed,
        );
    }

    pub fn set_mouse_button(&mut self, button: MouseButton, pressed: bool) {
        update_button_state(
            &mut self.mouse_buttons_down,
            &mut self.mouse_buttons_pressed,
            &mut self.mouse_buttons_released,
            mouse_button_bit(button),
            pressed,
        );
    }

    pub fn set_cursor_position(&mut self, x: f64, y: f64) {
        if let Some((previous_x, previous_y)) = self.cursor_position {
            self.cursor_delta.0 += x - previous_x;
            self.cursor_delta.1 += y - previous_y;
        }

        self.cursor_position = Some((x, y));
    }

    pub fn clear_cursor_position(&mut self) {
        self.cursor_position = None;
    }

    pub fn add_mouse_motion(&mut self, horizontal: f64, vertical: f64) {
        self.mouse_motion.0 += horizontal;
        self.mouse_motion.1 += vertical;
    }

    pub fn add_scroll_lines(&mut self, horizontal: f32, vertical: f32) {
        self.scroll_lines.0 += horizontal;
        self.scroll_lines.1 += vertical;
    }

    pub fn add_scroll_pixels(&mut self, horizontal: f64, vertical: f64) {
        self.scroll_pixels.0 += horizontal;
        self.scroll_pixels.1 += vertical;
    }

    pub fn focus_lost(&mut self) {
        self.keys_released |= self.keys_down;
        self.mouse_buttons_released |= self.mouse_buttons_down;
        self.keys_down = 0;
        self.mouse_buttons_down = 0;
        self.cursor_position = None;
        self.cursor_delta = (0.0, 0.0);
        self.mouse_motion = (0.0, 0.0);
        self.scroll_lines = (0.0, 0.0);
        self.scroll_pixels = (0.0, 0.0);
    }

    pub fn end_tick(&mut self) {
        self.keys_pressed = 0;
        self.keys_released = 0;
        self.mouse_buttons_pressed = 0;
        self.mouse_buttons_released = 0;
        self.cursor_delta = (0.0, 0.0);
        self.mouse_motion = (0.0, 0.0);
        self.scroll_lines = (0.0, 0.0);
        self.scroll_pixels = (0.0, 0.0);
    }
}

fn update_button_state(
    down: &mut u128,
    pressed_this_tick: &mut u128,
    released_this_tick: &mut u128,
    bit: Option<u128>,
    pressed: bool,
) {
    let Some(bit) = bit else {
        return;
    };
    let was_pressed = *down & bit != 0;

    match (pressed, was_pressed) {
        (true, false) => {
            *down |= bit;
            *pressed_this_tick |= bit;
        }
        (false, true) => {
            *down &= !bit;
            *released_this_tick |= bit;
        }
        _ => {}
    }
}

const fn key_bit(key: KeyCode) -> u128 {
    1 << key as u32
}

fn mouse_button_bit(button: MouseButton) -> Option<u128> {
    let index = match button {
        MouseButton::Left => 0,
        MouseButton::Right => 1,
        MouseButton::Middle => 2,
        MouseButton::Back => 3,
        MouseButton::Forward => 4,
        MouseButton::Other(button) => u32::from(button).checked_add(5)?,
    };

    1_u128.checked_shl(index)
}
