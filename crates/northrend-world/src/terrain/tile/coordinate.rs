#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TerrainTileCoordinate {
    pub x: i32,
    pub y: i32,
}

impl TerrainTileCoordinate {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
