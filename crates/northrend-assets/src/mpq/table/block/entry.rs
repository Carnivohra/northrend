const IMPLODE: u32 = 0x0000_0100;
const COMPRESS: u32 = 0x0000_0200;
const ENCRYPTED: u32 = 0x0001_0000;
const FIX_KEY: u32 = 0x0002_0000;
const PATCH_FILE: u32 = 0x0010_0000;
const SINGLE_UNIT: u32 = 0x0100_0000;
const SECTOR_CRC: u32 = 0x0400_0000;
const EXISTS: u32 = 0x8000_0000;

#[derive(Clone, Copy, Debug)]
pub(in crate::mpq) struct MpqBlockEntry {
    offset: u32,
    compressed_size: u32,
    size: u32,
    flags: u32,
}

impl MpqBlockEntry {
    pub(super) fn parse(bytes: &[u8]) -> Self {
        Self {
            offset: read_u32(bytes, 0),
            compressed_size: read_u32(bytes, 4),
            size: read_u32(bytes, 8),
            flags: read_u32(bytes, 12),
        }
    }

    pub(in crate::mpq) fn offset(&self) -> u32 {
        self.offset
    }

    pub(in crate::mpq) fn compressed_size(&self) -> u32 {
        self.compressed_size
    }

    pub(in crate::mpq) fn size(&self) -> u32 {
        self.size
    }

    pub(in crate::mpq) fn flags(&self) -> u32 {
        self.flags
    }

    pub(in crate::mpq) fn exists(&self) -> bool {
        self.flags & EXISTS != 0
    }

    pub(in crate::mpq) fn is_compressed(&self) -> bool {
        self.flags & (IMPLODE | COMPRESS) != 0
    }

    pub(in crate::mpq) fn uses_multi_compression(&self) -> bool {
        self.flags & COMPRESS != 0
    }

    pub(in crate::mpq) fn is_encrypted(&self) -> bool {
        self.flags & ENCRYPTED != 0
    }

    pub(in crate::mpq) fn uses_fixed_key(&self) -> bool {
        self.flags & FIX_KEY != 0
    }

    pub(in crate::mpq) fn is_patch(&self) -> bool {
        self.flags & PATCH_FILE != 0
    }

    pub(in crate::mpq) fn is_single_unit(&self) -> bool {
        self.flags & SINGLE_UNIT != 0
    }

    pub(in crate::mpq) fn has_sector_crc(&self) -> bool {
        self.flags & SECTOR_CRC != 0
    }
}

fn read_u32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes([
        bytes[offset],
        bytes[offset + 1],
        bytes[offset + 2],
        bytes[offset + 3],
    ])
}
