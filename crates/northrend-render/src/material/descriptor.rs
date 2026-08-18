use crate::ShaderHandle;

#[derive(Debug, Clone, Copy)]
pub struct MaterialDescriptor {
    pub shader: ShaderHandle,
}

impl MaterialDescriptor {
    pub const fn new(shader: ShaderHandle) -> Self {
        Self { shader }
    }
}
