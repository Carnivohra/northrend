#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TerrainChunkCoordinate {
    pub x: u16,
    pub y: u16,
}

impl TerrainChunkCoordinate {
    pub const fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}
