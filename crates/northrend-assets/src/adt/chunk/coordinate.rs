#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AdtChunkCoordinate {
    pub x: u8,
    pub y: u8,
}

impl AdtChunkCoordinate {
    pub const WIDTH: usize = 16;
    pub const COUNT: usize = Self::WIDTH * Self::WIDTH;

    pub const fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }

    pub(in crate::adt) const fn index(self) -> Option<usize> {
        if self.x as usize >= Self::WIDTH || self.y as usize >= Self::WIDTH {
            None
        } else {
            Some(self.y as usize * Self::WIDTH + self.x as usize)
        }
    }
}
