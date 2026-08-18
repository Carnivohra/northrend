use flate2::{Decompress, FlushDecompress, Status};

use crate::mpq::MpqError;

pub(super) fn decompress(bytes: &[u8], output: &mut [u8]) -> Result<(), MpqError> {
    let mut decompressor = Decompress::new(true);
    let status = decompressor
        .decompress(bytes, output, FlushDecompress::Finish)
        .map_err(|_| MpqError::InvalidFile)?;

    if status != Status::StreamEnd || decompressor.total_out() != output.len() as u64 {
        return Err(MpqError::InvalidFile);
    }

    Ok(())
}
