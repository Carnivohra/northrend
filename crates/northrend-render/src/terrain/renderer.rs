use northrend_math::Vec3;
use northrend_world::TerrainTile;

use crate::{
    Color, MaterialDescriptor, MaterialHandle, MeshData, MeshDraw, Renderer, ShaderDescriptor,
    ShaderSource, Vertex,
};

const SHADER: &str = include_str!("../../shaders/terrain.wgsl");

pub(crate) struct TerrainRenderer {
    material: Option<MaterialHandle>,
}

impl TerrainRenderer {
    pub(crate) const fn new() -> Self {
        Self { material: None }
    }

    pub(crate) fn create<R: Renderer>(
        &mut self,
        renderer: &mut R,
        tile: &TerrainTile,
    ) -> Result<MeshDraw, R::Error> {
        let material = match self.material {
            Some(material) => material,
            None => {
                let shader = renderer.create_shader(ShaderDescriptor {
                    label: Some("northrend terrain shader"),
                    source: ShaderSource::Wgsl(SHADER),
                })?;
                let material = renderer.create_material(MaterialDescriptor::new(shader))?;

                self.material = Some(material);
                material
            }
        };
        let vertices = tile.vertices().iter()
            .map(|vertex| Vertex::new(vertex.position, terrain_color(vertex.normal)))
            .collect::<Vec<_>>();
        let mut indices = Vec::with_capacity(tile.indices().len());

        for (index, chunk) in tile.chunks().iter().enumerate() {
            indices.extend(
                tile.chunk_indices(index)
                    .expect("terrain chunk indices are valid")
                    .iter()
                    .map(|index| chunk.vertex_offset() + u32::from(*index)),
            );
        }

        let mesh = renderer.create_mesh(MeshData::new(&vertices, &indices))?;

        Ok(MeshDraw::new(mesh, material))
    }
}

fn terrain_color(normal: Vec3) -> Color {
    let light = Vec3::new(-0.35, 0.85, 0.4).normalize();
    let intensity = 0.32 + normal.dot(light).max(0.0) * 0.68;

    Color::new(
        0.22 * intensity,
        0.58 * intensity,
        0.18 * intensity,
        1.0,
    )
}
