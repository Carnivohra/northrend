use thiserror::Error;

#[derive(Debug, Error)]
pub enum BackendError {
    #[error("failed to create window")]
    WindowCreationFailed,
}
