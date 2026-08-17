use crate::WgpuShader;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WgpuMaterial(usize);

impl WgpuMaterial {
    pub(crate) const fn new(index: usize) -> Self {
        Self(index)
    }

    pub(crate) const fn index(self) -> usize {
        self.0
    }
}

pub(crate) struct WgpuMaterialResource {
    pub(crate) shader: WgpuShader,
}
