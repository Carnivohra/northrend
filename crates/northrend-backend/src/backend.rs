mod application;
mod context;
mod error;

pub use application::BackendApplication;
pub use context::BackendContext;
pub use error::BackendError;

pub trait Backend {
    type Error;

    fn run<A>(self, application: A) -> Result<(), Self::Error>
    where
        A: BackendApplication + 'static;
}
