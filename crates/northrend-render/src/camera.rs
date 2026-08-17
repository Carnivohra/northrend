use northrend_math::{Mat4, Vec3};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Camera {
    pub position: Vec3,
    pub view_projection: Mat4,
}

impl Camera {
    pub const fn new(position: Vec3, view_projection: Mat4) -> Self {
        Self {
            position,
            view_projection,
        }
    }
}
