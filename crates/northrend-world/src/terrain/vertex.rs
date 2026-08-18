use northrend_math::Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TerrainVertex {
    pub position: Vec3,
    pub normal: Vec3,
}

impl TerrainVertex {
    pub const fn new(position: Vec3, normal: Vec3) -> Self {
        Self { position, normal }
    }
}
