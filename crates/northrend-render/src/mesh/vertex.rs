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
