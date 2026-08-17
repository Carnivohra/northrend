#[derive(Debug, Clone, Copy)]
pub struct MaterialDescriptor<S> {
    pub shader: S,
}

impl<S> MaterialDescriptor<S> {
    pub const fn new(shader: S) -> Self {
        Self { shader }
    }
}
