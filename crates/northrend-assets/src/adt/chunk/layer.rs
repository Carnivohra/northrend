#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct AdtTextureLayer {
    pub texture_id: u32,
    pub flags: u32,
    pub alpha_offset: u32,
    pub effect_id: u32,
}

impl AdtTextureLayer {
    pub const fn new(texture_id: u32, flags: u32, alpha_offset: u32, effect_id: u32) -> Self {
        Self {
            texture_id,
            flags,
            alpha_offset,
            effect_id,
        }
    }
}
