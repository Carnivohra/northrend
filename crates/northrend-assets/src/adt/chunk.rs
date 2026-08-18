mod coordinate;
mod layer;

pub use coordinate::AdtChunkCoordinate;
pub use layer::AdtTextureLayer;

pub const ADT_CHUNK_HEIGHT_COUNT: usize = 145;
pub const ADT_CHUNK_NORMAL_COUNT: usize = 145;
pub const ADT_CHUNK_LAYER_CAPACITY: usize = 4;

#[derive(Debug)]
pub struct AdtChunk {
    pub(in crate::adt) coordinate: AdtChunkCoordinate,
    pub(in crate::adt) flags: u32,
    pub(in crate::adt) area_id: u32,
    pub(in crate::adt) holes: u32,
    pub(in crate::adt) position: [f32; 3],
    pub(in crate::adt) heights: [f32; ADT_CHUNK_HEIGHT_COUNT],
    pub(in crate::adt) normals: [[i8; 3]; ADT_CHUNK_NORMAL_COUNT],
    pub(in crate::adt) layers: [AdtTextureLayer; ADT_CHUNK_LAYER_CAPACITY],
    pub(in crate::adt) layer_count: u8,
    pub(in crate::adt) legacy_liquid: bool,
}

impl AdtChunk {
    pub const fn coordinate(&self) -> AdtChunkCoordinate {
        self.coordinate
    }

    pub const fn flags(&self) -> u32 {
        self.flags
    }

    pub const fn area_id(&self) -> u32 {
        self.area_id
    }

    pub const fn holes(&self) -> u32 {
        self.holes
    }

    pub const fn position(&self) -> [f32; 3] {
        self.position
    }

    pub const fn heights(&self) -> &[f32; ADT_CHUNK_HEIGHT_COUNT] {
        &self.heights
    }

    pub const fn normals(&self) -> &[[i8; 3]; ADT_CHUNK_NORMAL_COUNT] {
        &self.normals
    }

    pub fn layers(&self) -> &[AdtTextureLayer] {
        &self.layers[..usize::from(self.layer_count)]
    }

    pub const fn has_legacy_liquid(&self) -> bool {
        self.legacy_liquid
    }
}
