mod application;

pub use application::BackendApplication;

pub trait Backend {
    type Error;

    fn run<A>(self, application: A) -> Result<(), Self::Error>
    where
        A: BackendApplication + 'static;
}
