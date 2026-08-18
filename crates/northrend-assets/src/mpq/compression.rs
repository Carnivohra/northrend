mod zlib;

use super::MpqError;

const ZLIB: u8 = 0x02;

pub(super) fn decompress(bytes: &[u8], output: &mut [u8]) -> Result<(), MpqError> {
    let (&compression, bytes) = bytes.split_first().ok_or(MpqError::InvalidFile)?;

    match compression {
        ZLIB => zlib::decompress(bytes, output),
        compression => Err(MpqError::UnsupportedCompression(compression)),
    }
}
