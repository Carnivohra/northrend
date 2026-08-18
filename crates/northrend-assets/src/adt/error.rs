use thiserror::Error;

#[derive(Debug, Error)]
pub enum AdtError {
    #[error("ADT chunk at offset {0:#x} is invalid")]
    InvalidChunk(usize),

    #[error("ADT terrain chunk at offset {0:#x} is invalid")]
    InvalidTerrainChunk(usize),

    #[error("ADT is missing the {0} chunk")]
    MissingChunk(&'static str),

    #[error("ADT version {0} is not supported")]
    UnsupportedVersion(u32),

    #[error("ADT terrain chunk coordinate ({x}, {y}) occurs more than once")]
    DuplicateTerrainChunk { x: u8, y: u8 },

    #[error("ADT contains {0} terrain chunks instead of 256")]
    UnexpectedTerrainChunkCount(usize),

    #[error("ADT texture name is not valid UTF-8")]
    InvalidTextureName,
}
