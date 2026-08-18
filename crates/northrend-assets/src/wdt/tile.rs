mod coordinate;

pub use coordinate::WdtTileCoordinate;

#[derive(Debug, Clone, Copy)]
pub struct WdtTile {
    coordinate: WdtTileCoordinate,
    flags: u32,
    asynchronous_id: u32,
}

impl WdtTile {
    pub(super) const fn new(
        coordinate: WdtTileCoordinate,
        flags: u32,
        asynchronous_id: u32,
    ) -> Self {
        Self {
            coordinate,
            flags,
            asynchronous_id,
        }
    }

    pub const fn coordinate(&self) -> WdtTileCoordinate {
        self.coordinate
    }

    pub const fn flags(&self) -> u32 {
        self.flags
    }

    pub const fn asynchronous_id(&self) -> u32 {
        self.asynchronous_id
    }

    pub const fn exists(&self) -> bool {
        self.flags & 1 != 0
    }
}
