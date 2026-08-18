use std::fs::File;

use crate::mpq::{
    MpqError,
    compression,
    cryptography::{FILE_KEY_HASH, decrypt, hash_path},
    io::read_exact_at,
    table::block::entry::MpqBlockEntry,
};

const IMPLODE_COMPRESSION: u8 = 0x08;

pub(in crate::mpq) fn read_file(
    file: &File,
    file_size: u64,
    archive_offset: u64,
    sector_size: u32,
    block: MpqBlockEntry,
    path: &str,
) -> Result<Vec<u8>, MpqError> {
    if block.is_patch() {
        return Err(MpqError::UnsupportedFileFlags(block.flags()));
    }

    let raw_offset = archive_offset
        .checked_add(u64::from(block.offset()))
        .ok_or(MpqError::InvalidFile)?;
    let raw_size = block.compressed_size() as usize;
    let raw_end = raw_offset
        .checked_add(raw_size as u64)
        .ok_or(MpqError::InvalidFile)?;

    if raw_end > file_size {
        return Err(MpqError::InvalidFile);
    }

    let mut bytes = allocate(raw_size)?;
    read_exact_at(file, &mut bytes, raw_offset)?;

    let size = block.size() as usize;

    if size == 0 {
        return Ok(Vec::new());
    }

    let key = block
        .is_encrypted()
        .then(|| file_key(path, block))
        .transpose()?;

    if block.is_single_unit() {
        read_single_unit(bytes, size, block, key)
    } else {
        read_sectors(bytes, size, sector_size as usize, block, key)
    }
}

fn read_single_unit(
    mut bytes: Vec<u8>,
    size: usize,
    block: MpqBlockEntry,
    key: Option<u32>,
) -> Result<Vec<u8>, MpqError> {
    if let Some(key) = key {
        decrypt(&mut bytes, key);
    }

    if bytes.len() == size {
        return Ok(bytes);
    }

    if bytes.len() > size || !block.is_compressed() {
        return Err(MpqError::InvalidFile);
    }

    let mut output = allocate(size)?;
    decompress(&bytes, &mut output, block)?;

    Ok(output)
}

fn read_sectors(
    mut bytes: Vec<u8>,
    size: usize,
    sector_size: usize,
    block: MpqBlockEntry,
    key: Option<u32>,
) -> Result<Vec<u8>, MpqError> {
    if sector_size == 0 {
        return Err(MpqError::InvalidFile);
    }

    if !block.is_compressed() {
        if bytes.len() < size {
            return Err(MpqError::InvalidFile);
        }

        if let Some(key) = key {
            for (index, sector) in bytes[..size].chunks_mut(sector_size).enumerate() {
                decrypt(sector, key.wrapping_add(index as u32));
            }
        }

        bytes.truncate(size);
        return Ok(bytes);
    }

    let sector_count = size.div_ceil(sector_size);
    let offset_count = sector_count.checked_add(1).ok_or(MpqError::InvalidFile)?;
    let offset_table_size = offset_count.checked_mul(4).ok_or(MpqError::InvalidFile)?;
    let minimum_data_offset = offset_count
        .checked_add(usize::from(block.has_sector_crc()))
        .and_then(|count| count.checked_mul(4))
        .ok_or(MpqError::InvalidFile)?;

    if bytes.len() < minimum_data_offset {
        return Err(MpqError::InvalidFile);
    }

    if let Some(key) = key {
        decrypt(&mut bytes[..offset_table_size], key.wrapping_sub(1));
    }

    let first_offset = read_u32(&bytes, 0) as usize;

    if first_offset < minimum_data_offset || first_offset > bytes.len() {
        return Err(MpqError::InvalidFile);
    }

    let mut output = allocate(size)?;

    for index in 0..sector_count {
        let encoded_start = read_u32(&bytes, index * 4) as usize;
        let encoded_end = read_u32(&bytes, (index + 1) * 4) as usize;
        let output_start = index * sector_size;
        let output_end = (output_start + sector_size).min(size);

        if encoded_start < first_offset
            || encoded_end < encoded_start
            || encoded_end > bytes.len()
        {
            return Err(MpqError::InvalidFile);
        }

        let encoded = &mut bytes[encoded_start..encoded_end];
        let decoded = &mut output[output_start..output_end];

        if let Some(key) = key {
            decrypt(encoded, key.wrapping_add(index as u32));
        }

        match encoded.len().cmp(&decoded.len()) {
            std::cmp::Ordering::Less if encoded.is_empty() => {}
            std::cmp::Ordering::Less => decompress(encoded, decoded, block)?,
            std::cmp::Ordering::Equal => decoded.copy_from_slice(encoded),
            std::cmp::Ordering::Greater => return Err(MpqError::InvalidFile),
        }
    }

    Ok(output)
}

fn decompress(
    bytes: &[u8],
    output: &mut [u8],
    block: MpqBlockEntry,
) -> Result<(), MpqError> {
    if block.uses_multi_compression() {
        compression::decompress(bytes, output)
    } else {
        Err(MpqError::UnsupportedCompression(IMPLODE_COMPRESSION))
    }
}

fn file_key(path: &str, block: MpqBlockEntry) -> Result<u32, MpqError> {
    let name = path
        .rsplit(['/', '\\'])
        .next()
        .ok_or(MpqError::InvalidFile)?;
    let mut key = hash_path(name, FILE_KEY_HASH).ok_or(MpqError::InvalidFile)?;

    if block.uses_fixed_key() {
        key = key.wrapping_add(block.offset()) ^ block.size();
    }

    Ok(key)
}

fn allocate(size: usize) -> Result<Vec<u8>, MpqError> {
    let mut bytes = Vec::new();

    bytes
        .try_reserve_exact(size)
        .map_err(|_| MpqError::InvalidFile)?;
    bytes.resize(size, 0);

    Ok(bytes)
}

fn read_u32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes([
        bytes[offset],
        bytes[offset + 1],
        bytes[offset + 2],
        bytes[offset + 3],
    ])
}
