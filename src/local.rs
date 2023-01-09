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

    pub fn write(&self, dist_dir: &str) -> Result<PathBuf> {
        let dist_path = self.dist_path(dist_dir)?;
        match fs::write(&dist_path, &self.contents) {
            Ok(_) => Ok(dist_path),
            Err(details) => Err(AxoassetError::LocalAssetWriteFailed {
                origin_path: self.origin_path.to_string(),
                dist_path: dist_path.display().to_string(),
                details: details.to_string(),
            }),
        }
    }

    pub fn copy(origin_path: &str, dist_dir: &str) -> Result<PathBuf> {
        LocalAsset::load(origin_path)?.write(dist_dir)
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

    fn dist_path(&self, dist_dir: &str) -> Result<PathBuf> {
        let filename = self.filename()?;
        Ok(Path::new(dist_dir).join(filename))
    }
}
