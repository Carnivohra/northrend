use thiserror::Error;

#[derive(Debug, Error)]
pub enum WdtError {
    #[error("WDT chunk at offset {0:#x} is invalid")]
    InvalidChunk(usize),

    #[error("WDT is missing the {0} chunk")]
    MissingChunk(&'static str),

    #[error("WDT version {0} is not supported")]
    UnsupportedVersion(u32),
}
