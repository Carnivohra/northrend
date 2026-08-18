#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WdtTileCoordinate {
    pub x: u8,
    pub y: u8,
}

impl WdtTileCoordinate {
    pub const WIDTH: usize = 64;
    pub const COUNT: usize = Self::WIDTH * Self::WIDTH;

    pub const fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}
