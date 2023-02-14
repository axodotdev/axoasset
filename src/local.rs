use std::fs;
use std::path::{Path, PathBuf};

use crate::error::*;

/// A local asset contains a path on the local filesystem and its contents
#[derive(Debug)]
pub struct LocalAsset {
    /// A string representing a path on the local filesystem, where the asset
    /// originated. For a new asset, this will be the path you want the asset
    /// to be written to. This path is how the filename is determined for all
    /// asset operations.
    pub origin_path: String,
    /// The contents of the asset as a vector of bytes.
    pub contents: Vec<u8>,
}

impl LocalAsset {
    /// A new asset is created with a path on the local filesystem and a
    /// vector of bytes representing its contents
    pub fn new(origin_path: &str, contents: Vec<u8>) -> Self {
        LocalAsset {
            origin_path: origin_path.to_string(),
            contents,
        }
    }

    /// Loads an asset from a path on the local filesystem, returning a
    /// LocalAsset struct
    pub fn load(origin_path: &str) -> Result<LocalAsset> {
        match Path::new(origin_path).try_exists() {
            Ok(_) => match fs::read(origin_path) {
                Ok(contents) => Ok(LocalAsset {
                    origin_path: origin_path.to_string(),
                    contents,
                }),
                Err(details) => Err(AxoassetError::LocalAssetReadFailed {
                    origin_path: origin_path.to_string(),
                    details: details.to_string(),
                }),
            },
            Err(details) => Err(AxoassetError::LocalAssetNotFound {
                origin_path: origin_path.to_string(),
                details: details.to_string(),
            }),
        }
    }

    /// Loads an asset from a path on the local filesystem, returning a
    /// string of its contents
    pub fn load_string(origin_path: &str) -> Result<String> {
        match Path::new(origin_path).try_exists() {
            Ok(_) => match fs::read_to_string(origin_path) {
                Ok(contents) => Ok(contents),
                Err(details) => Err(AxoassetError::LocalAssetReadFailed {
                    origin_path: origin_path.to_string(),
                    details: details.to_string(),
                }),
            },
            Err(details) => Err(AxoassetError::LocalAssetNotFound {
                origin_path: origin_path.to_string(),
                details: details.to_string(),
            }),
        }
    }

    /// Loads an asset from a path on the local filesystem, returning a
    /// vector of bytes of its contents
    pub fn load_bytes(origin_path: &str) -> Result<Vec<u8>> {
        match Path::new(origin_path).try_exists() {
            Ok(_) => match fs::read(origin_path) {
                Ok(contents) => Ok(contents),
                Err(details) => Err(AxoassetError::LocalAssetReadFailed {
                    origin_path: origin_path.to_string(),
                    details: details.to_string(),
                }),
            },
            Err(details) => Err(AxoassetError::LocalAssetNotFound {
                origin_path: origin_path.to_string(),
                details: details.to_string(),
            }),
        }
    }

    /// Writes an asset to a path on the local filesystem, determines the
    /// filename from the origin path
    pub fn write(&self, dest_dir: &str) -> Result<PathBuf> {
        let dest_path = self.dest_path(dest_dir)?;
        match fs::write(&dest_path, &self.contents) {
            Ok(_) => Ok(dest_path),
            Err(details) => Err(AxoassetError::LocalAssetWriteFailed {
                origin_path: self.origin_path.to_string(),
                dest_path: dest_path.display().to_string(),
                details: details.to_string(),
            }),
        }
    }

    /// Copies an asset from one location on the local filesystem to another
    pub fn copy(origin_path: &str, dest_dir: &str) -> Result<PathBuf> {
        LocalAsset::load(origin_path)?.write(dest_dir)
    }

    fn filename(&self) -> Result<PathBuf> {
        if let Some(filename) = Path::new(&self.origin_path).file_name() {
            Ok(filename.into())
        } else {
            Err(AxoassetError::LocalAssetMissingFilename {
                origin_path: self.origin_path.to_string(),
            })
        }
    }

    fn dest_path(&self, dest_dir: &str) -> Result<PathBuf> {
        let filename = self.filename()?;
        Ok(Path::new(dest_dir).join(filename))
    }
}
