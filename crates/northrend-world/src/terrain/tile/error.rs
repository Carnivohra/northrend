use thiserror::Error;

#[derive(Debug, Error)]
pub enum TerrainTileError {
    #[error("terrain tile vertex capacity was exceeded")]
    VertexCapacityExceeded,

    #[error("terrain tile index capacity was exceeded")]
    IndexCapacityExceeded,

    #[error("terrain chunk contains an out-of-bounds index")]
    InvalidChunkIndex,
}
