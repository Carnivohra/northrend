use crate::{Camera, MeshDraw};

#[derive(Debug, Clone, Copy)]
pub struct RenderView<'a> {
    pub camera: &'a Camera,
    pub draws: &'a [MeshDraw],
}

impl<'a> RenderView<'a> {
    pub const fn new(
        camera: &'a Camera,
        draws: &'a [MeshDraw],
    ) -> Self {
        Self { camera, draws }
    }
}
