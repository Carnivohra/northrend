mod event;

pub use event::WindowEvent;

use northrend_math::Size;

pub trait Window {
    fn set_title(&self, title: &str);
    fn set_size(&self, size: Size);
}
