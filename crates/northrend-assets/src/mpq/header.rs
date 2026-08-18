use super::MpqError;

const SIGNATURE: [u8; 4] = *b"MPQ\x1a";
const VERSION_ONE_SIZE: usize = 0x20;
const VERSION_TWO_SIZE: usize = 0x2c;
const TABLE_ENTRY_SIZE: u64 = 16;

#[derive(Debug)]
pub(super) struct MpqHeader {
    format_version: u16,
    sector_size: u32,
    hash_table_offset: u64,
    block_table_offset: u64,
    hash_table_entries: u32,
    block_table_entries: u32,
}

impl MpqHeader {
    pub(super) const MIN_SIZE: usize = VERSION_ONE_SIZE;

    pub(super) fn has_signature(bytes: &[u8]) -> bool {
        bytes.starts_with(&SIGNATURE)
    }

    pub(super) fn parse(
        bytes: &[u8],
        archive_offset: u64,
        file_size: u64,
    ) -> Result<Self, MpqError> {
        if bytes.len() < VERSION_ONE_SIZE || !Self::has_signature(bytes) {
            return Err(MpqError::InvalidHeader);
        }

        let header_size = read_u32(bytes, 0x04) as usize;
        let archive_size = read_u32(bytes, 0x08) as u64;
        let format_version = read_u16(bytes, 0x0c);
        let sector_size_shift = read_u16(bytes, 0x0e);
        let hash_table_offset = read_u32(bytes, 0x10) as u64;
        let block_table_offset = read_u32(bytes, 0x14) as u64;
        let hash_table_entries = read_u32(bytes, 0x18);
        let block_table_entries = read_u32(bytes, 0x1c);

        let required_size = match format_version {
            0 => VERSION_ONE_SIZE,
            1 => VERSION_TWO_SIZE,
            version => return Err(MpqError::UnsupportedVersion(version)),
        };

        if header_size < required_size || bytes.len() < required_size {
            return Err(MpqError::InvalidHeader);
        }

        let header_end = archive_offset
            .checked_add(header_size as u64)
            .ok_or(MpqError::InvalidHeader)?;

        if header_end > file_size || !hash_table_entries.is_power_of_two() {
            return Err(MpqError::InvalidHeader);
        }

        if format_version == 0 {
            let archive_end = archive_offset
                .checked_add(archive_size)
                .ok_or(MpqError::InvalidHeader)?;

            if archive_size < header_size as u64 || archive_end > file_size {
                return Err(MpqError::InvalidHeader);
            }
        }

        let sector_size = 0x200u32
            .checked_shl(u32::from(sector_size_shift))
            .ok_or(MpqError::InvalidHeader)?;

        let (hash_table_offset, block_table_offset) = if format_version == 1 {
            (
                hash_table_offset | (u64::from(read_u16(bytes, 0x28)) << 32),
                block_table_offset | (u64::from(read_u16(bytes, 0x2a)) << 32),
            )
        } else {
            (hash_table_offset, block_table_offset)
        };

        validate_table(
            archive_offset,
            hash_table_offset,
            hash_table_entries,
            file_size,
        )?;
        validate_table(
            archive_offset,
            block_table_offset,
            block_table_entries,
            file_size,
        )?;

        Ok(Self {
            format_version,
            sector_size,
            hash_table_offset,
            block_table_offset,
            hash_table_entries,
            block_table_entries,
        })
    }

    pub(super) fn format_version(&self) -> u16 {
        self.format_version
    }

    pub(super) fn sector_size(&self) -> u32 {
        self.sector_size
    }

    pub(super) fn hash_table_offset(&self) -> u64 {
        self.hash_table_offset
    }

    pub(super) fn block_table_offset(&self) -> u64 {
        self.block_table_offset
    }

    pub(super) fn hash_table_entries(&self) -> u32 {
        self.hash_table_entries
    }

    pub(super) fn block_table_entries(&self) -> u32 {
        self.block_table_entries
    }
}

fn validate_table(
    archive_offset: u64,
    table_offset: u64,
    entries: u32,
    file_size: u64,
) -> Result<(), MpqError> {
    let table_size = u64::from(entries)
        .checked_mul(TABLE_ENTRY_SIZE)
        .ok_or(MpqError::InvalidHeader)?;
    let table_end = archive_offset
        .checked_add(table_offset)
        .and_then(|offset| offset.checked_add(table_size))
        .ok_or(MpqError::InvalidHeader)?;

    if table_end > file_size {
        return Err(MpqError::InvalidHeader);
    }

    Ok(())
}

fn read_u16(bytes: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes([bytes[offset], bytes[offset + 1]])
}

fn read_u32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes([
        bytes[offset],
        bytes[offset + 1],
        bytes[offset + 2],
        bytes[offset + 3],
    ])
}
