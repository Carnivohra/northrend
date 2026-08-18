use std::{
    fs::File,
    io::Read,
    path::Path,
};

use super::{
    MpqError,
    file::read_file,
    header::MpqHeader,
    io::read_exact_at,
    table::{
        MpqBlockTable, MpqHashTable,
        block::entry::MpqBlockEntry,
    },
};

const HEADER_ALIGNMENT: usize = 0x200;
const HEADER_SEARCH_BLOCK_SIZE: usize = 0x1000;
const HEADER_SEARCH_LIMIT: u64 = 0x0800_0000;

#[derive(Debug)]
pub struct MpqArchive {
    file: File,
    file_size: u64,
    header_offset: u64,
    header: MpqHeader,
    hash_table: MpqHashTable,
    block_table: MpqBlockTable,
}

impl MpqArchive {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, MpqError> {
        let mut file = File::open(path)?;
        let file_size = file.metadata()?.len();
        let (header_offset, header) = find_header(&mut file, file_size)?;
        let hash_table = MpqHashTable::decode(read_table(
            &file,
            header_offset,
            header.hash_table_offset(),
            header.hash_table_entries(),
        )?)?;
        let block_table = MpqBlockTable::decode(read_table(
            &file,
            header_offset,
            header.block_table_offset(),
            header.block_table_entries(),
        )?)?;

        Ok(Self {
            file,
            file_size,
            header_offset,
            header,
            hash_table,
            block_table,
        })
    }

    pub fn file_size(&self) -> u64 {
        self.file_size
    }

    pub fn header_offset(&self) -> u64 {
        self.header_offset
    }

    pub fn format_version(&self) -> u16 {
        self.header.format_version()
    }

    pub fn sector_size(&self) -> u32 {
        self.header.sector_size()
    }

    pub fn contains_file(&self, path: &str) -> bool {
        self.find_block(path).is_some()
    }

    pub fn read_file(&self, path: &str) -> Result<Vec<u8>, MpqError> {
        let block = self.find_block(path).ok_or(MpqError::FileNotFound)?;

        read_file(
            &self.file,
            self.file_size,
            self.header_offset,
            self.header.sector_size(),
            block,
            path,
        )
    }

    fn find_block(&self, path: &str) -> Option<MpqBlockEntry> {
        let index = self.hash_table.find(path, self.block_table.len())?;
        self.block_table.get(index).filter(|block| block.exists())
    }
}

fn read_table(
    file: &File,
    archive_offset: u64,
    table_offset: u64,
    entries: u32,
) -> Result<Vec<u8>, MpqError> {
    let offset = archive_offset
        .checked_add(table_offset)
        .ok_or(MpqError::InvalidTable)?;
    let size = usize::try_from(entries)
        .ok()
        .and_then(|entries| entries.checked_mul(16))
        .ok_or(MpqError::InvalidTable)?;
    let mut bytes = Vec::new();

    bytes
        .try_reserve_exact(size)
        .map_err(|_| MpqError::InvalidTable)?;
    bytes.resize(size, 0);

    read_exact_at(file, &mut bytes, offset)?;

    Ok(bytes)
}

fn find_header(file: &mut File, file_size: u64) -> Result<(u64, MpqHeader), MpqError> {
    if file_size < MpqHeader::MIN_SIZE as u64 {
        return Err(MpqError::HeaderNotFound);
    }

    let search_size = file_size.min(HEADER_SEARCH_LIMIT);
    let mut block_offset = 0u64;
    let mut block = [0u8; HEADER_SEARCH_BLOCK_SIZE];
    let mut candidate_error = None;

    while block_offset < search_size {
        let read_size = (search_size - block_offset).min(HEADER_SEARCH_BLOCK_SIZE as u64) as usize;
        file.read_exact(&mut block[..read_size])?;

        for candidate_offset in (0..read_size).step_by(HEADER_ALIGNMENT) {
            let candidate = &block[candidate_offset..read_size];

            if !MpqHeader::has_signature(candidate) {
                continue;
            }

            let header_offset = block_offset + candidate_offset as u64;

            match MpqHeader::parse(candidate, header_offset, file_size) {
                Ok(header) => return Ok((header_offset, header)),
                Err(error) => candidate_error = Some(error),
            }
        }

        block_offset += read_size as u64;
    }

    Err(candidate_error.unwrap_or(MpqError::HeaderNotFound))
}
