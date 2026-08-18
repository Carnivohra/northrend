use std::sync::OnceLock;

const CRYPT_TABLE_SIZE: usize = 0x500;

pub(super) const HASH_TABLE_KEY: u32 = 0xc3af_3770;
pub(super) const BLOCK_TABLE_KEY: u32 = 0xec83_b3a3;
pub(super) const TABLE_OFFSET_HASH: usize = 0x000;
pub(super) const NAME_A_HASH: usize = 0x100;
pub(super) const NAME_B_HASH: usize = 0x200;
pub(super) const FILE_KEY_HASH: usize = 0x300;

pub(super) fn decrypt(bytes: &mut [u8], mut key: u32) {
    let table = crypt_table();
    let mut seed = 0xeeee_eeeeu32;

    for chunk in bytes.chunks_exact_mut(4) {
        seed = seed.wrapping_add(table[0x400 + (key & 0xff) as usize]);

        let value = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]])
            ^ key.wrapping_add(seed);

        chunk.copy_from_slice(&value.to_le_bytes());

        key = (!key)
            .wrapping_shl(21)
            .wrapping_add(0x1111_1111)
            | (key >> 11);
        seed = value
            .wrapping_add(seed)
            .wrapping_add(seed.wrapping_shl(5))
            .wrapping_add(3);
    }
}

pub(super) fn hash_path(path: &str, hash_type: usize) -> Option<u32> {
    if path.is_empty() || !path.is_ascii() {
        return None;
    }

    let table = crypt_table();
    let mut seed1 = 0x7fed_7fedu32;
    let mut seed2 = 0xeeee_eeeeu32;

    for byte in path.bytes() {
        let byte = match byte {
            b'a'..=b'z' => byte - (b'a' - b'A'),
            b'/' => b'\\',
            _ => byte,
        };

        seed1 = table[hash_type + byte as usize] ^ seed1.wrapping_add(seed2);
        seed2 = u32::from(byte)
            .wrapping_add(seed1)
            .wrapping_add(seed2)
            .wrapping_add(seed2.wrapping_shl(5))
            .wrapping_add(3);
    }

    Some(seed1)
}

fn crypt_table() -> &'static [u32; CRYPT_TABLE_SIZE] {
    static TABLE: OnceLock<[u32; CRYPT_TABLE_SIZE]> = OnceLock::new();

    TABLE.get_or_init(|| {
        let mut table = [0; CRYPT_TABLE_SIZE];
        let mut seed = 0x0010_0001u32;

        for first_index in 0..0x100 {
            let mut index = first_index;

            for _ in 0..5 {
                seed = seed.wrapping_mul(125).wrapping_add(3) % 0x002a_aaab;
                let first = (seed & 0xffff) << 16;

                seed = seed.wrapping_mul(125).wrapping_add(3) % 0x002a_aaab;
                let second = seed & 0xffff;

                table[index] = first | second;
                index += 0x100;
            }
        }

        table
    })
}
