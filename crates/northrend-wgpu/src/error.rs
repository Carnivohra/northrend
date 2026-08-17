use thiserror::Error;
use wgpu::{CreateSurfaceError, RequestAdapterError, RequestDeviceError};

#[derive(Debug, Error)]
pub enum WgpuError {
    #[error(transparent)]
    SurfaceCreation(#[from] CreateSurfaceError),

    #[error(transparent)]
    AdapterRequest(#[from] RequestAdapterError),

    #[error(transparent)]
    DeviceRequest(#[from] RequestDeviceError),

    #[error("surface is not supported by the selected graphics adapter")]
    UnsupportedSurface,

    #[error("surface was lost")]
    SurfaceLost,

    #[error("surface validation failed")]
    SurfaceValidationFailed,

    #[error("renderer is not initialized")]
    RendererNotInitialized,

    #[error("renderer resource capacity was exceeded")]
    ResourceCapacityExceeded,

    #[error("shader handle is invalid")]
    InvalidShader,

    #[error("material handle is invalid")]
    InvalidMaterial,

    #[error("frame contains too many render views")]
    TooManyViews,

    #[error("mesh handle is invalid")]
    InvalidMesh,

    #[error("mesh contains invalid vertex or index data")]
    InvalidMeshData,
}
