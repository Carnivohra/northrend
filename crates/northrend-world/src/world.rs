use crate::Terrain;

#[derive(Debug, Default)]
pub struct World {
    terrain: Terrain,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub const fn terrain(&self) -> &Terrain {
        &self.terrain
    }

    pub const fn terrain_mut(&mut self) -> &mut Terrain {
        &mut self.terrain
    }
}
