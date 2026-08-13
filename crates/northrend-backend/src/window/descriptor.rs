#[derive(Debug, Clone)]
pub struct WindowDescriptor {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub resizable: bool,
    pub visible: bool,
}

impl Default for WindowDescriptor {
    fn default() -> Self {
        Self {
            title: String::from("Northrend"),
            width: 1280,
            height: 720,
            resizable: true,
            visible: true,
        }
    }
}
