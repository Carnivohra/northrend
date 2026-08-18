use super::{Wdt, WdtError, WdtTile, WdtTileCoordinate};

const WDT_VERSION: u32 = 18;
const CHUNK_HEADER_SIZE: usize = 8;
const MAIN_ENTRY_SIZE: usize = 8;

const MVER: [u8; 4] = *b"REVM";
const MAIN: [u8; 4] = *b"NIAM";

pub(super) fn read(bytes: &[u8]) -> Result<Wdt, WdtError> {
    let mut version = None;
    let mut tiles = None;
    let mut offset = 0;

    while offset < bytes.len() {
        let header_end = offset
            .checked_add(CHUNK_HEADER_SIZE)
            .filter(|end| *end <= bytes.len())
            .ok_or(WdtError::InvalidChunk(offset))?;
        let size = u32_at(bytes, offset + 4) as usize;
        let end = header_end
            .checked_add(size)
            .filter(|end| *end <= bytes.len())
            .ok_or(WdtError::InvalidChunk(offset))?;
        let id = bytes[offset..offset + 4]
            .try_into()
            .map_err(|_| WdtError::InvalidChunk(offset))?;
        let data = &bytes[header_end..end];

        match id {
            MVER => {
                require_size(data, size_of::<u32>(), offset)?;
                version = Some(u32_at(data, 0));
            }
            MAIN => tiles = Some(read_tiles(data, offset)?),
            _ => {}
        }

        offset = end;
    }

    let version = version.ok_or(WdtError::MissingChunk("MVER"))?;

    if version != WDT_VERSION {
        return Err(WdtError::UnsupportedVersion(version));
    }

    Ok(Wdt::new(
        version,
        tiles.ok_or(WdtError::MissingChunk("MAIN"))?,
    ))
}

fn read_tiles(data: &[u8], offset: usize) -> Result<Vec<WdtTile>, WdtError> {
    require_size(data, WdtTileCoordinate::COUNT * MAIN_ENTRY_SIZE, offset)?;

    let mut tiles = Vec::with_capacity(WdtTileCoordinate::COUNT);

    for (index, bytes) in data
        .chunks_exact(MAIN_ENTRY_SIZE)
        .take(WdtTileCoordinate::COUNT)
        .enumerate()
    {
        let x = (index % WdtTileCoordinate::WIDTH) as u8;
        let y = (index / WdtTileCoordinate::WIDTH) as u8;

        tiles.push(WdtTile::new(
            WdtTileCoordinate::new(x, y),
            u32_at(bytes, 0),
            u32_at(bytes, 4),
        ));
    }

    Ok(tiles)
}

fn require_size(data: &[u8], size: usize, offset: usize) -> Result<(), WdtError> {
    if data.len() < size {
        Err(WdtError::InvalidChunk(offset))
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
