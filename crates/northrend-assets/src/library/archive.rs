use std::path::{Path, PathBuf};

use crate::{
    library::AssetError,
    mpq::{MpqArchive, MpqError},
};

#[derive(Debug)]
pub(super) struct MountedArchive {
    path: PathBuf,
    archive: MpqArchive,
}

impl MountedArchive {
    pub(super) fn open(path: PathBuf) -> Result<Self, AssetError> {
        let archive = MpqArchive::open(&path).map_err(|source| AssetError::OpenArchive {
            path: path.clone(),
            source,
        })?;

        Ok(Self { path, archive })
    }

    pub(super) fn path(&self) -> &Path {
        &self.path
    }

    pub(super) fn read_file(&self, path: &str) -> Result<Vec<u8>, MpqError> {
        self.archive.read_file(path)
    }
}
