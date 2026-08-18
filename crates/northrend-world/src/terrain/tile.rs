mod builder;
mod coordinate;
mod error;

pub use builder::TerrainTileBuilder;
pub use coordinate::TerrainTileCoordinate;
pub use error::TerrainTileError;

use super::{TerrainChunk, TerrainVertex};

#[derive(Debug)]
pub struct TerrainTile {
    chunks: Box<[TerrainChunk]>,
    vertices: Box<[TerrainVertex]>,
    indices: Box<[u16]>,
}

impl TerrainTile {
    pub(super) fn new(
        chunks: Vec<TerrainChunk>,
        vertices: Vec<TerrainVertex>,
        indices: Vec<u16>,
    ) -> Self {
        Self {
            chunks: chunks.into_boxed_slice(),
            vertices: vertices.into_boxed_slice(),
            indices: indices.into_boxed_slice(),
        }
    }

    pub fn chunks(&self) -> &[TerrainChunk] {
        &self.chunks
    }

    pub fn vertices(&self) -> &[TerrainVertex] {
        &self.vertices
    }

    pub fn indices(&self) -> &[u16] {
        &self.indices
    }

    pub fn chunk_vertices(&self, chunk_index: usize) -> Option<&[TerrainVertex]> {
        let chunk = self.chunks.get(chunk_index)?;
        let start = chunk.vertex_offset() as usize;
        let end = start + chunk.vertex_count() as usize;
        Some(&self.vertices[start..end])
    }

    pub fn chunk_indices(&self, chunk_index: usize) -> Option<&[u16]> {
        let chunk = self.chunks.get(chunk_index)?;
        let start = chunk.index_offset() as usize;
        let end = start + chunk.index_count() as usize;
        Some(&self.indices[start..end])
    }
}
