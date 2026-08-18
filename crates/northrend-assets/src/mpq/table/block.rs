use crate::mpq::{
    MpqError,
    cryptography::{BLOCK_TABLE_KEY, decrypt},
};

pub(in crate::mpq) mod entry;

use entry::MpqBlockEntry;

const ENTRY_SIZE: usize = 16;

#[derive(Debug)]
pub(in crate::mpq) struct MpqBlockTable {
    bytes: Box<[u8]>,
}

impl MpqBlockTable {
    pub(in crate::mpq) fn decode(mut bytes: Vec<u8>) -> Result<Self, MpqError> {
        if !bytes.len().is_multiple_of(ENTRY_SIZE) {
            return Err(MpqError::InvalidTable);
        }

        decrypt(&mut bytes, BLOCK_TABLE_KEY);

        Ok(Self {
            bytes: bytes.into_boxed_slice(),
        })
    }

    pub(in crate::mpq) fn len(&self) -> usize {
        self.bytes.len() / ENTRY_SIZE
    }

    pub(in crate::mpq) fn get(&self, index: usize) -> Option<MpqBlockEntry> {
        let offset = index.checked_mul(ENTRY_SIZE)?;
        let bytes = self.bytes.get(offset..offset + ENTRY_SIZE)?;

        Some(MpqBlockEntry::parse(bytes))
    }
}
