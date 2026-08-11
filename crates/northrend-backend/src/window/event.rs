use northrend_math::{Point, Size};

pub enum WindowEvent {
    CloseRequested,
    Resized(Size),
    Moved(Point),
}
