use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum MpqError {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error("MPQ header was not found")]
    HeaderNotFound,

    #[error("MPQ header is invalid")]
    InvalidHeader,

    #[error("MPQ table is invalid")]
    InvalidTable,

    #[error("file was not found in the MPQ archive")]
    FileNotFound,

    #[error("MPQ file is invalid")]
    InvalidFile,

    #[error("MPQ file flags {0:#010x} are not supported")]
    UnsupportedFileFlags(u32),

    #[error("MPQ compression type {0:#04x} is not supported")]
    UnsupportedCompression(u8),

    #[error("MPQ format version {0} is not supported")]
    UnsupportedVersion(u16),
}
