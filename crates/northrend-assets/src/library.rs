mod archive;
mod error;
mod mount;

pub use error::AssetError;
use std::{collections::HashMap, path::Path, sync::RwLock};

use self::archive::MountedArchive;
use crate::{adt::Adt, mpq::MpqError, wdt::Wdt};

#[derive(Debug)]
pub struct AssetLibrary {
    archives: Box<[MountedArchive]>,
    resolved_paths: RwLock<HashMap<Box<str>, Option<usize>>>,
}

impl AssetLibrary {
    pub fn open(path: impl AsRef<Path>, base_archive_order: &[&str]) -> Result<Self, AssetError> {
        let paths = mount::discover(path.as_ref(), base_archive_order)?;
        let archives = paths
            .into_iter()
            .map(MountedArchive::open)
            .collect::<Result<_, _>>()?;

        Ok(Self {
            archives,
            resolved_paths: RwLock::new(HashMap::new()),
        })
    }

    pub fn read_file(&self, path: &str) -> Result<Vec<u8>, AssetError> {
        if let Some(index) = self.cached_archive(path) {
            return match index {
                Some(index) => {
                    let archive = &self.archives[index];

                    archive
                        .read_file(path)
                        .map_err(|source| AssetError::ReadArchive {
                            path: archive.path().into(),
                            source,
                        })
                }
                None => Err(AssetError::FileNotFound(path.into())),
            };
        }

        for (index, archive) in self.archives.iter().enumerate().rev() {
            match archive.read_file(path) {
                Ok(bytes) => {
                    self.cache_path(path, Some(index));
                    return Ok(bytes);
                }
                Err(MpqError::FileNotFound) => {}
                Err(source) => {
                    return Err(AssetError::ReadArchive {
                        path: archive.path().into(),
                        source,
                    });
                }
            }
        }

        self.cache_path(path, None);
        Err(AssetError::FileNotFound(path.into()))
    }

    pub fn read_adt(&self, path: &str) -> Result<Adt, AssetError> {
        let bytes = self.read_file(path)?;

        Adt::read(&bytes).map_err(|source| AssetError::InvalidAdt {
            path: path.into(),
            source,
        })
    }

    pub fn read_wdt(&self, path: &str) -> Result<Wdt, AssetError> {
        let bytes = self.read_file(path)?;

        Wdt::read(&bytes).map_err(|source| AssetError::InvalidWdt {
            path: path.into(),
            source,
        })
    }

    pub fn archive_count(&self) -> usize {
        self.archives.len()
    }

    fn cached_archive(&self, path: &str) -> Option<Option<usize>> {
        self.resolved_paths
            .read()
            .unwrap_or_else(|error| error.into_inner())
            .get(path)
            .copied()
    }

    fn cache_path(&self, path: &str, index: Option<usize>) {
        self.resolved_paths
            .write()
            .unwrap_or_else(|error| error.into_inner())
            .insert(path.into(), index);
    }
}
