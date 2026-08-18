use crate::MaterialHandle;

use super::MeshHandle;

#[derive(Debug, Clone, Copy)]
pub struct MeshDraw {
    pub mesh: MeshHandle,
    pub material: MaterialHandle,
}

impl MeshDraw {
    pub const fn new(mesh: MeshHandle, material: MaterialHandle) -> Self {
        Self { mesh, material }
    }
}
