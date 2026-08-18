use super::Vertex;

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
