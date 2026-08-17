use std::num::NonZeroU32;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShaderHandle(NonZeroU32);

impl ShaderHandle {
    pub fn from_index(index: usize) -> Option<Self> {
        let value = u32::try_from(index).ok()?.checked_add(1)?;
        NonZeroU32::new(value).map(Self)
    }

    pub const fn index(self) -> usize {
        self.0.get() as usize - 1
    }
}
