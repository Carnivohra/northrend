use northrend_assets::{Adt, AdtChunk};
use northrend_math::Vec3;

use crate::{TerrainChunkCoordinate, TerrainVertex};

use super::{TerrainTile, TerrainTileBuilder, TerrainTileError};

const CHUNK_WIDTH: f32 = 100.0 / 3.0;
const CELL_WIDTH: f32 = CHUNK_WIDTH / 8.0;
const VERTICES_PER_CHUNK: usize = 145;
const MAX_INDICES_PER_CHUNK: usize = 8 * 8 * 12;

pub(super) fn load(adt: &Adt) -> Result<TerrainTile, TerrainTileError> {
    let mut builder = TerrainTileBuilder::with_capacity(
        adt.chunks().len(),
        adt.chunks().len() * VERTICES_PER_CHUNK,
        adt.chunks().len() * MAX_INDICES_PER_CHUNK,
    );

    for chunk in adt.chunks() {
        let vertices = vertices(chunk);
        let indices = indices(chunk.holes());

        builder.push_chunk(
            TerrainChunkCoordinate::new(
                u16::from(chunk.coordinate().x),
                u16::from(chunk.coordinate().y),
            ),
            &vertices,
            &indices,
        )?;
    }

    Ok(builder.build())
}

fn vertices(chunk: &AdtChunk) -> [TerrainVertex; VERTICES_PER_CHUNK] {
    let mut vertices = [TerrainVertex::new(Vec3::ZERO, Vec3::ZERO); VERTICES_PER_CHUNK];
    let position = chunk.position();

    for row in 0..=8 {
        for column in 0..=8 {
            let index = outer_index(column, row);
            vertices[index].position = Vec3::new(
                position[1] - column as f32 * CELL_WIDTH,
                position[2] + chunk.heights()[index],
                position[0] - row as f32 * CELL_WIDTH,
            );
        }
    }

    for row in 0..8 {
        for column in 0..8 {
            let index = center_index(column, row);
            vertices[index].position = Vec3::new(
                position[1] - (column as f32 + 0.5) * CELL_WIDTH,
                position[2] + chunk.heights()[index],
                position[0] - (row as f32 + 0.5) * CELL_WIDTH,
            );
        }
    }

    for (vertex, normal) in vertices.iter_mut().zip(chunk.normals()) {
        vertex.normal = Vec3::new(
            f32::from(normal[1]),
            f32::from(normal[2]),
            f32::from(normal[0]),
        )
        .normalize();
    }

    vertices
}

fn indices(holes: u32) -> Vec<u16> {
    let mut indices = Vec::with_capacity(MAX_INDICES_PER_CHUNK);

    for row in 0..8 {
        for column in 0..8 {
            let hole = (row / 2) * 4 + column / 2;

            if holes & (1 << hole) != 0 {
                continue;
            }

            let top_left = outer_index(column, row) as u16;
            let top_right = outer_index(column + 1, row) as u16;
            let bottom_left = outer_index(column, row + 1) as u16;
            let bottom_right = outer_index(column + 1, row + 1) as u16;
            let center = center_index(column, row) as u16;

            indices.extend_from_slice(&[
                top_left,
                center,
                top_right,
                top_right,
                center,
                bottom_right,
                bottom_right,
                center,
                bottom_left,
                bottom_left,
                center,
                top_left,
            ]);
        }
    }

    indices
}

const fn outer_index(column: usize, row: usize) -> usize {
    row * 17 + column
}

const fn center_index(column: usize, row: usize) -> usize {
    row * 17 + 9 + column
}
