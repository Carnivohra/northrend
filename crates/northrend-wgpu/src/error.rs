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
}
