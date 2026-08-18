mod chunk;
mod error;
mod profile;
mod reader;

pub use chunk::{AdtChunk, AdtChunkCoordinate, AdtTextureLayer};
pub use error::AdtError;
pub use profile::AdtProfile;

#[derive(Debug)]
pub struct Adt {
    version: u32,
    profile: AdtProfile,
    texture_names: Box<[Box<str>]>,
    chunks: Box<[AdtChunk]>,
}

impl Adt {
    pub fn read(bytes: &[u8]) -> Result<Self, AdtError> {
        reader::read(bytes)
    }

    pub(super) fn new(
        version: u32,
        profile: AdtProfile,
        texture_names: Vec<Box<str>>,
        chunks: Vec<AdtChunk>,
    ) -> Self {
        Self {
            version,
            profile,
            texture_names: texture_names.into_boxed_slice(),
            chunks: chunks.into_boxed_slice(),
        }
    }

    pub const fn version(&self) -> u32 {
        self.version
    }

    pub const fn profile(&self) -> AdtProfile {
        self.profile
    }

    pub fn texture_count(&self) -> usize {
        self.texture_names.len()
    }

    pub fn texture_name(&self, index: usize) -> Option<&str> {
        self.texture_names.get(index).map(AsRef::as_ref)
    }

    pub fn chunks(&self) -> &[AdtChunk] {
        &self.chunks
    }

    pub fn chunk(&self, coordinate: AdtChunkCoordinate) -> Option<&AdtChunk> {
        coordinate.index().and_then(|index| self.chunks.get(index))
    }
}
