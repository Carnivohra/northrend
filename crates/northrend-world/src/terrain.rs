mod chunk;
mod tile;
mod vertex;

use std::collections::HashMap;

pub use chunk::{TerrainChunk, TerrainChunkCoordinate};
pub use tile::{TerrainTile, TerrainTileBuilder, TerrainTileCoordinate, TerrainTileError};
pub use vertex::TerrainVertex;

#[derive(Debug, Default)]
pub struct Terrain {
    tiles: HashMap<TerrainTileCoordinate, TerrainTile>,
}

impl Terrain {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tile(&self, coordinate: TerrainTileCoordinate) -> Option<&TerrainTile> {
        self.tiles.get(&coordinate)
    }

    pub fn tile_mut(&mut self, coordinate: TerrainTileCoordinate) -> Option<&mut TerrainTile> {
        self.tiles.get_mut(&coordinate)
    }

    pub fn insert(
        &mut self,
        coordinate: TerrainTileCoordinate,
        tile: TerrainTile,
    ) -> Option<TerrainTile> {
        self.tiles.insert(coordinate, tile)
    }

    pub fn remove(&mut self, coordinate: TerrainTileCoordinate) -> Option<TerrainTile> {
        self.tiles.remove(&coordinate)
    }

    pub fn tiles(&self) -> impl Iterator<Item = (&TerrainTileCoordinate, &TerrainTile)> {
        self.tiles.iter()
    }

    pub fn len(&self) -> usize {
        self.tiles.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tiles.is_empty()
    }
}
