use northrend_world::TerrainTile;

use crate::{MeshDraw, Renderer, terrain::TerrainRenderer};

pub struct RenderScene {
    draws: Vec<MeshDraw>,
    terrain: TerrainRenderer,
}

impl RenderScene {
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {
            draws: Vec::new(),
            terrain: TerrainRenderer::new(),
        }
    }

    pub fn load_terrain<R: Renderer>(
        &mut self,
        renderer: &mut R,
        tile: &TerrainTile,
    ) -> Result<(), R::Error> {
        let draw = self.terrain.create(renderer, tile)?;

        self.draws.push(draw);
        Ok(())
    }

    pub fn draws(&self) -> &[MeshDraw] {
        &self.draws
    }
}
