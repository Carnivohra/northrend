mod coordinate;

pub use coordinate::TerrainChunkCoordinate;

#[derive(Debug, Clone, Copy)]
pub struct TerrainChunk {
    coordinate: TerrainChunkCoordinate,
    vertex_offset: u32,
    vertex_count: u32,
    index_offset: u32,
    index_count: u32,
}

impl TerrainChunk {
    pub(super) const fn new(
        coordinate: TerrainChunkCoordinate,
        vertex_offset: u32,
        vertex_count: u32,
        index_offset: u32,
        index_count: u32,
    ) -> Self {
        Self {
            coordinate,
            vertex_offset,
            vertex_count,
            index_offset,
            index_count,
        }
    }

    pub const fn coordinate(&self) -> TerrainChunkCoordinate {
        self.coordinate
    }

    pub const fn vertex_offset(&self) -> u32 {
        self.vertex_offset
    }

    pub const fn vertex_count(&self) -> u32 {
        self.vertex_count
    }

    pub const fn index_offset(&self) -> u32 {
        self.index_offset
    }

    pub const fn index_count(&self) -> u32 {
        self.index_count
    }
}
