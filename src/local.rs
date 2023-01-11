use std::fs;
use std::path::{Path, PathBuf};

use crate::error::*;

#[derive(Debug)]
pub struct LocalAsset {
    pub origin_path: String,
    pub contents: Vec<u8>,
}

impl LocalAsset {
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
