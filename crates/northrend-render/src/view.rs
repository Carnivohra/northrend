use crate::{Camera, MeshDraw};

#[derive(Debug, Clone, Copy)]
pub struct RenderView<'a, M, T> {
    pub camera: &'a Camera,
    pub draws: &'a [MeshDraw<'a, M, T>],
}

impl<'a, M, T> RenderView<'a, M, T> {
    pub const fn new(
        camera: &'a Camera,
        draws: &'a [MeshDraw<'a, M, T>],
    ) -> Self {
        Self { camera, draws }
    }
}
