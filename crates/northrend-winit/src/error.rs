use thiserror::Error;
use winit::error::EventLoopError;

#[derive(Debug, Error)]
pub enum WinitError {
    #[error(transparent)]
    EventLoop(#[from] EventLoopError),
}
