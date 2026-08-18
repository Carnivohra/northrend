use crate::mpq::{
    MpqError,
    cryptography::{
        HASH_TABLE_KEY, NAME_A_HASH, NAME_B_HASH, TABLE_OFFSET_HASH, decrypt, hash_path,
    },
};

const ENTRY_SIZE: usize = 16;
const BLOCK_INDEX_MASK: u32 = 0x0fff_ffff;
const FREE_BLOCK_INDEX: u32 = u32::MAX;
const DELETED_BLOCK_INDEX: u32 = u32::MAX - 1;

#[derive(Debug)]
pub(in crate::mpq) struct MpqHashTable {
    bytes: Box<[u8]>,
}

impl MpqHashTable {
    pub(in crate::mpq) fn decode(mut bytes: Vec<u8>) -> Result<Self, MpqError> {
        let entries = bytes.len() / ENTRY_SIZE;

        if !bytes.len().is_multiple_of(ENTRY_SIZE) || !entries.is_power_of_two() {
            return Err(MpqError::InvalidTable);
        }

        decrypt(&mut bytes, HASH_TABLE_KEY);

        Ok(Self {
            bytes: bytes.into_boxed_slice(),
        })
    }

    pub(in crate::mpq) fn find(&self, path: &str, block_count: usize) -> Option<usize> {
        let name_a = hash_path(path, NAME_A_HASH)?;
        let name_b = hash_path(path, NAME_B_HASH)?;
        let mask = self.len() - 1;
        let mut index = hash_path(path, TABLE_OFFSET_HASH)? as usize & mask;
        let start = index;

        loop {
            let offset = index * ENTRY_SIZE;
            let block_index = read_u32(&self.bytes, offset + 12);

            if block_index == FREE_BLOCK_INDEX {
                return None;
            }

            if block_index != DELETED_BLOCK_INDEX
                && read_u32(&self.bytes, offset) == name_a
                && read_u32(&self.bytes, offset + 4) == name_b
            {
                let block_index = (block_index & BLOCK_INDEX_MASK) as usize;

                return (block_index < block_count).then_some(block_index);
            }

            index = (index + 1) & mask;

            if index == start {
                return None;
            }
        }
    }

    fn len(&self) -> usize {
        self.bytes.len() / ENTRY_SIZE
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
