mod error;
mod reader;
mod tile;

pub use error::WdtError;
pub use tile::{WdtTile, WdtTileCoordinate};

#[derive(Debug)]
pub struct Wdt {
    version: u32,
    tiles: Box<[WdtTile]>,
}

impl Wdt {
    pub fn read(bytes: &[u8]) -> Result<Self, WdtError> {
        reader::read(bytes)
    }

    pub(super) fn new(version: u32, tiles: Vec<WdtTile>) -> Self {
        Self {
            version,
            tiles: tiles.into_boxed_slice(),
        }
    }

    pub const fn version(&self) -> u32 {
        self.version
    }

    pub fn tiles(&self) -> &[WdtTile] {
        &self.tiles
    }

    pub fn existing_tiles(&self) -> impl Iterator<Item = &WdtTile> {
        self.tiles.iter().filter(|tile| tile.exists())
    }
}
