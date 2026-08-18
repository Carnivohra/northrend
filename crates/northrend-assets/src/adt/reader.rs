use std::str;

use super::{
    Adt, AdtChunk, AdtChunkCoordinate, AdtError, AdtProfile, AdtTextureLayer,
    chunk::{ADT_CHUNK_HEIGHT_COUNT, ADT_CHUNK_LAYER_CAPACITY, ADT_CHUNK_NORMAL_COUNT},
};

const ADT_VERSION: u32 = 18;
const CHUNK_HEADER_SIZE: usize = 8;
const MCNK_HEADER_SIZE: usize = 128;
const MCVT_SIZE: usize = ADT_CHUNK_HEIGHT_COUNT * size_of::<f32>();
const MCNR_SIZE: usize = ADT_CHUNK_NORMAL_COUNT * 3;
const MCLY_ENTRY_SIZE: usize = 16;

const MVER: [u8; 4] = *b"REVM";
const MHDR: [u8; 4] = *b"RDHM";
const MCIN: [u8; 4] = *b"NICM";
const MTEX: [u8; 4] = *b"XETM";
const MFBO: [u8; 4] = *b"OBFM";
const MH2O: [u8; 4] = *b"O2HM";
const MCNK: [u8; 4] = *b"KNCM";
const MCVT: [u8; 4] = *b"TVCM";
const MCNR: [u8; 4] = *b"RNCM";
const MCLY: [u8; 4] = *b"YLCM";

pub(super) fn read(bytes: &[u8]) -> Result<Adt, AdtError> {
    let mut version = None;
    let mut has_header = false;
    let mut has_chunk_index = false;
    let mut has_mfbo = false;
    let mut has_mh2o = false;
    let mut texture_names = Vec::new();
    let mut chunks = std::iter::repeat_with(|| None)
        .take(AdtChunkCoordinate::COUNT)
        .collect::<Vec<Option<AdtChunk>>>();
    let mut chunk_count = 0;
    let mut offset = 0;

    while offset < bytes.len() {
        let header_end = offset
            .checked_add(CHUNK_HEADER_SIZE)
            .filter(|end| *end <= bytes.len())
            .ok_or(AdtError::InvalidChunk(offset))?;
        let size = u32_at(bytes, offset + 4) as usize;
        let end = header_end
            .checked_add(size)
            .filter(|end| *end <= bytes.len())
            .ok_or(AdtError::InvalidChunk(offset))?;
        let id = bytes[offset..offset + 4]
            .try_into()
            .map_err(|_| AdtError::InvalidChunk(offset))?;
        let data = &bytes[header_end..end];

        match id {
            MVER => {
                require_size(data, size_of::<u32>(), offset)?;
                version = Some(u32_at(data, 0));
            }
            MHDR => {
                require_size(data, 64, offset)?;
                has_header = true;
            }
            MCIN => {
                require_size(data, AdtChunkCoordinate::COUNT * 16, offset)?;
                has_chunk_index = true;
            }
            MTEX => parse_texture_names(data, &mut texture_names)?,
            MFBO => has_mfbo = true,
            MH2O => has_mh2o = true,
            MCNK => {
                let chunk = parse_terrain_chunk(bytes, offset, end)?;
                let coordinate = chunk.coordinate();
                let index = coordinate
                    .index()
                    .ok_or(AdtError::InvalidTerrainChunk(offset))?;

                if chunks[index].replace(chunk).is_some() {
                    return Err(AdtError::DuplicateTerrainChunk {
                        x: coordinate.x,
                        y: coordinate.y,
                    });
                }

                chunk_count += 1;
            }
            _ => {}
        }

        offset = end;
    }

    let version = version.ok_or(AdtError::MissingChunk("MVER"))?;

    if version != ADT_VERSION {
        return Err(AdtError::UnsupportedVersion(version));
    }

    if !has_header {
        return Err(AdtError::MissingChunk("MHDR"));
    }

    if !has_chunk_index {
        return Err(AdtError::MissingChunk("MCIN"));
    }

    if chunk_count != AdtChunkCoordinate::COUNT {
        return Err(AdtError::UnexpectedTerrainChunkCount(chunk_count));
    }

    let chunks = chunks
        .into_iter()
        .collect::<Option<Vec<_>>>()
        .ok_or(AdtError::UnexpectedTerrainChunkCount(chunk_count))?;
    let profile = AdtProfile::detect(has_mfbo, has_mh2o);

    Ok(Adt::new(version, profile, texture_names, chunks))
}

fn parse_terrain_chunk(
    bytes: &[u8],
    chunk_offset: usize,
    chunk_end: usize,
) -> Result<AdtChunk, AdtError> {
    let header_start = chunk_offset + CHUNK_HEADER_SIZE;
    let header_end = header_start
        .checked_add(MCNK_HEADER_SIZE)
        .filter(|end| *end <= chunk_end)
        .ok_or(AdtError::InvalidTerrainChunk(chunk_offset))?;
    let header = &bytes[header_start..header_end];
    let x = terrain_coordinate(u32_at(header, 4), chunk_offset)?;
    let y = terrain_coordinate(u32_at(header, 8), chunk_offset)?;
    let coordinate = AdtChunkCoordinate::new(x, y);
    let layer_count = usize::try_from(u32_at(header, 12))
        .ok()
        .filter(|count| *count <= ADT_CHUNK_LAYER_CAPACITY)
        .ok_or(AdtError::InvalidTerrainChunk(chunk_offset))?;
    let heights = parse_heights(
        subchunk(bytes, chunk_offset, chunk_end, u32_at(header, 20), MCVT)?,
        chunk_offset,
    )?;
    let normals = parse_normals(
        subchunk(bytes, chunk_offset, chunk_end, u32_at(header, 24), MCNR)?,
        chunk_offset,
    )?;
    let layers = parse_layers(
        bytes,
        chunk_offset,
        chunk_end,
        u32_at(header, 28),
        layer_count,
    )?;
    let position = [
        f32_at(header, 104),
        f32_at(header, 108),
        f32_at(header, 112),
    ];
    let legacy_liquid = u32_at(header, 96) != 0 && u32_at(header, 100) > 8;

    Ok(AdtChunk {
        coordinate,
        flags: u32_at(header, 0),
        area_id: u32_at(header, 52),
        holes: u32_at(header, 60),
        position,
        heights,
        normals,
        layers,
        layer_count: layer_count as u8,
        legacy_liquid,
    })
}

fn parse_heights(
    data: &[u8],
    chunk_offset: usize,
) -> Result<[f32; ADT_CHUNK_HEIGHT_COUNT], AdtError> {
    require_terrain_size(data, MCVT_SIZE, chunk_offset)?;
    let mut heights = [0.0; ADT_CHUNK_HEIGHT_COUNT];

    for (height, bytes) in heights.iter_mut().zip(data[..MCVT_SIZE].chunks_exact(4)) {
        *height = f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    }

    Ok(heights)
}

fn parse_normals(
    data: &[u8],
    chunk_offset: usize,
) -> Result<[[i8; 3]; ADT_CHUNK_NORMAL_COUNT], AdtError> {
    require_terrain_size(data, MCNR_SIZE, chunk_offset)?;
    let mut normals = [[0; 3]; ADT_CHUNK_NORMAL_COUNT];

    for (normal, bytes) in normals.iter_mut().zip(data[..MCNR_SIZE].chunks_exact(3)) {
        *normal = [bytes[0] as i8, bytes[1] as i8, bytes[2] as i8];
    }

    Ok(normals)
}

fn parse_layers(
    bytes: &[u8],
    chunk_offset: usize,
    chunk_end: usize,
    relative_offset: u32,
    count: usize,
) -> Result<[AdtTextureLayer; ADT_CHUNK_LAYER_CAPACITY], AdtError> {
    let mut layers = [AdtTextureLayer::default(); ADT_CHUNK_LAYER_CAPACITY];

    if count == 0 {
        return Ok(layers);
    }

    let data = subchunk(bytes, chunk_offset, chunk_end, relative_offset, MCLY)?;
    require_terrain_size(data, count * MCLY_ENTRY_SIZE, chunk_offset)?;

    for (layer, bytes) in layers
        .iter_mut()
        .zip(data.chunks_exact(MCLY_ENTRY_SIZE))
        .take(count)
    {
        *layer = AdtTextureLayer::new(
            u32_at(bytes, 0),
            u32_at(bytes, 4),
            u32_at(bytes, 8),
            u32_at(bytes, 12),
        );
    }

    Ok(layers)
}

fn subchunk(
    bytes: &[u8],
    chunk_offset: usize,
    chunk_end: usize,
    relative_offset: u32,
    expected: [u8; 4],
) -> Result<&[u8], AdtError> {
    let offset = chunk_offset
        .checked_add(relative_offset as usize)
        .filter(|offset| *offset >= chunk_offset + CHUNK_HEADER_SIZE + MCNK_HEADER_SIZE)
        .ok_or(AdtError::InvalidTerrainChunk(chunk_offset))?;
    let header_end = offset
        .checked_add(CHUNK_HEADER_SIZE)
        .filter(|end| *end <= chunk_end)
        .ok_or(AdtError::InvalidTerrainChunk(chunk_offset))?;

    if bytes[offset..offset + 4] != expected {
        return Err(AdtError::InvalidTerrainChunk(chunk_offset));
    }

    let size = u32_at(bytes, offset + 4) as usize;
    let end = header_end
        .checked_add(size)
        .filter(|end| *end <= chunk_end)
        .ok_or(AdtError::InvalidTerrainChunk(chunk_offset))?;

    Ok(&bytes[header_end..end])
}

fn parse_texture_names(data: &[u8], names: &mut Vec<Box<str>>) -> Result<(), AdtError> {
    for name in data
        .split(|byte| *byte == 0)
        .filter(|name| !name.is_empty())
    {
        names.push(
            str::from_utf8(name)
                .map_err(|_| AdtError::InvalidTextureName)?
                .into(),
        );
    }

    Ok(())
}

fn terrain_coordinate(value: u32, chunk_offset: usize) -> Result<u8, AdtError> {
    u8::try_from(value)
        .ok()
        .filter(|value| usize::from(*value) < AdtChunkCoordinate::WIDTH)
        .ok_or(AdtError::InvalidTerrainChunk(chunk_offset))
}

fn require_size(data: &[u8], size: usize, offset: usize) -> Result<(), AdtError> {
    if data.len() < size {
        Err(AdtError::InvalidChunk(offset))
    } else {
        Ok(())
    }
}

fn require_terrain_size(data: &[u8], size: usize, offset: usize) -> Result<(), AdtError> {
    if data.len() < size {
        Err(AdtError::InvalidTerrainChunk(offset))
    } else {
        Ok(())
    }
}

fn u32_at(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes([
        bytes[offset],
        bytes[offset + 1],
        bytes[offset + 2],
        bytes[offset + 3],
    ])
}

fn f32_at(bytes: &[u8], offset: usize) -> f32 {
    f32::from_le_bytes([
        bytes[offset],
        bytes[offset + 1],
        bytes[offset + 2],
        bytes[offset + 3],
    ])
}
