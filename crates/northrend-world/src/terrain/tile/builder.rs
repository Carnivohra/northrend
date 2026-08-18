use crate::{TerrainChunk, TerrainChunkCoordinate, TerrainVertex};

use super::{TerrainTile, TerrainTileError};

#[derive(Debug, Default)]
pub struct TerrainTileBuilder {
    chunks: Vec<TerrainChunk>,
    vertices: Vec<TerrainVertex>,
    indices: Vec<u16>,
}

impl TerrainTileBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(
        chunk_capacity: usize,
        vertex_capacity: usize,
        index_capacity: usize,
    ) -> Self {
        Self {
            chunks: Vec::with_capacity(chunk_capacity),
            vertices: Vec::with_capacity(vertex_capacity),
            indices: Vec::with_capacity(index_capacity),
        }
    }

    pub fn push_chunk(
        &mut self,
        coordinate: TerrainChunkCoordinate,
        vertices: &[TerrainVertex],
        indices: &[u16],
    ) -> Result<(), TerrainTileError> {
        if indices.iter().any(|&index| usize::from(index) >= vertices.len()) {
            return Err(TerrainTileError::InvalidChunkIndex);
        }

        let vertex_offset = u32::try_from(self.vertices.len())
            .map_err(|_| TerrainTileError::VertexCapacityExceeded)?;
        let index_offset = u32::try_from(self.indices.len())
            .map_err(|_| TerrainTileError::IndexCapacityExceeded)?;

        let vertex_end = self.vertices.len().checked_add(vertices.len())
            .and_then(|count| u32::try_from(count).ok())
            .ok_or(TerrainTileError::VertexCapacityExceeded)?;

        let index_end = self.indices.len().checked_add(indices.len())
            .and_then(|count| u32::try_from(count).ok())
            .ok_or(TerrainTileError::IndexCapacityExceeded)?;

        self.chunks.push(TerrainChunk::new(
            coordinate,
            vertex_offset,
            vertex_end - vertex_offset,
            index_offset,
            index_end - index_offset,
        ));

        self.vertices.extend_from_slice(vertices);
        self.indices.extend_from_slice(indices);
        Ok(())
    }

    pub fn build(self) -> TerrainTile {
        TerrainTile::new(self.chunks, self.vertices, self.indices)
    }
}
