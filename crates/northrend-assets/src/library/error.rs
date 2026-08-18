use std::{io, path::PathBuf};

use thiserror::Error;

use crate::{adt::AdtError, mpq::MpqError, wdt::WdtError};

#[derive(Debug, Error)]
pub enum AssetError {
    #[error("failed to scan asset directory `{path}`")]
    Scan {
        path: PathBuf,
        #[source]
        source: io::Error,
    },

    #[error("failed to open MPQ archive `{path}`")]
    OpenArchive {
        path: PathBuf,
        #[source]
        source: MpqError,
    },

    #[error("failed to read from MPQ archive `{path}`")]
    ReadArchive {
        path: PathBuf,
        #[source]
        source: MpqError,
    },

    #[error("failed to parse ADT asset `{path}`")]
    InvalidAdt {
        path: Box<str>,
        #[source]
        source: AdtError,
    },

    #[error("failed to parse WDT asset `{path}`")]
    InvalidWdt {
        path: Box<str>,
        #[source]
        source: WdtError,
    },

    #[error("asset file `{0}` was not found")]
    FileNotFound(Box<str>),
}
