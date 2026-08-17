use northrend_math::Vec3;

use crate::Color;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vertex {
    pub position: Vec3,
    pub color: Color,
}

impl Vertex {
    pub const fn new(position: Vec3, color: Color) -> Self {
        Self { position, color }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MeshData<'a> {
    pub vertices: &'a [Vertex],
    pub indices: &'a [u32],
}

impl<'a> MeshData<'a> {
    pub const fn new(vertices: &'a [Vertex], indices: &'a [u32]) -> Self {
        Self { vertices, indices }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MeshDraw<'a, M, T> {
    pub mesh: &'a M,
    pub material: T,
}

impl<'a, M, T> MeshDraw<'a, M, T> {
    pub const fn new(mesh: &'a M, material: T) -> Self {
        Self { mesh, material }
    }
}
