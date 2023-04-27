use std::fs;
use std::path::{Path, PathBuf};

use camino::{Utf8Path, Utf8PathBuf};

use crate::compression::{tar_dir, zip_dir, CompressionImpl};
use crate::error::*;

/// A local asset contains a path on the local filesystem and its contents
#[derive(Debug)]
pub struct LocalAsset {
    /// The computed filename from origin_path
    pub filename: String,
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
    pub fn new(origin_path: &str, contents: Vec<u8>) -> Result<Self> {
        Ok(LocalAsset {
            filename: LocalAsset::filename(origin_path)?,
            origin_path: origin_path.to_string(),
            contents,
        })
    }

    /// Loads an asset from a path on the local filesystem, returning a
    /// LocalAsset struct
    pub fn load(origin_path: &str) -> Result<LocalAsset> {
        match Path::new(origin_path).try_exists() {
            Ok(_) => match fs::read(origin_path) {
                Ok(contents) => Ok(LocalAsset {
                    filename: LocalAsset::filename(origin_path)?,
                    origin_path: origin_path.to_string(),
                    contents,
                }),
                Err(details) => Err(AxoassetError::LocalAssetReadFailed {
                    origin_path: origin_path.to_string(),
                    details,
                }),
            },
            Err(details) => Err(AxoassetError::LocalAssetNotFound {
                origin_path: origin_path.to_string(),
                details,
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
                    details,
                }),
            },
            Err(details) => Err(AxoassetError::LocalAssetNotFound {
                origin_path: origin_path.to_string(),
                details,
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
                    details,
                }),
            },
            Err(details) => Err(AxoassetError::LocalAssetNotFound {
                origin_path: origin_path.to_string(),
                details,
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
                details,
            }),
        }
    }

    /// Writes an asset to a path on the local filesystem, determines the
    /// filename from the origin path
    pub fn write_new(contents: &str, filename: &str, dest_dir: &str) -> Result<PathBuf> {
        let dest_path = Path::new(dest_dir).join(filename);
        match fs::write(&dest_path, contents) {
            Ok(_) => Ok(dest_path),
            Err(details) => Err(AxoassetError::LocalAssetWriteNewFailed {
                dest_path: dest_path.display().to_string(),
                details,
            }),
        }
    }

    /// Writes an asset and all of its parent directories on the local filesystem.
    pub fn write_new_all(contents: &str, filename: &str, dest_dir: &str) -> Result<PathBuf> {
        let dest_path = Path::new(dest_dir).join(filename);
        match fs::create_dir_all(dest_dir) {
            Ok(_) => (),
            Err(details) => {
                return Err(AxoassetError::LocalAssetWriteNewFailed {
                    dest_path: dest_path.display().to_string(),
                    details,
                })
            }
        }
        LocalAsset::write_new(contents, filename, dest_dir)
    }

    /// Creates a new directory
    pub fn create_dir(dest: &str) -> Result<PathBuf> {
        let dest_path = PathBuf::from(dest);
        match fs::create_dir(&dest_path) {
            Ok(_) => Ok(dest_path),
            Err(details) => Err(AxoassetError::LocalAssetDirCreationFailed {
                dest_path: dest_path.display().to_string(),
                details,
            }),
        }
    }

    /// Creates a new directory, including all parent directories
    pub fn create_dir_all(dest: &str) -> Result<PathBuf> {
        let dest_path = PathBuf::from(dest);
        match fs::create_dir_all(&dest_path) {
            Ok(_) => Ok(dest_path),
            Err(details) => Err(AxoassetError::LocalAssetDirCreationFailed {
                dest_path: dest_path.display().to_string(),
                details,
            }),
        }
    }

    /// Removes a file
    pub fn remove_file(dest: &str) -> Result<()> {
        let dest_path = PathBuf::from(dest);
        if let Err(details) = fs::remove_file(&dest_path) {
            return Err(AxoassetError::LocalAssetRemoveFailed {
                dest_path: dest_path.display().to_string(),
                details,
            });
        }

        Ok(())
    }

    /// Removes a directory
    pub fn remove_dir(dest: &str) -> Result<()> {
        let dest_path = PathBuf::from(dest);
        if dest_path.is_dir() {
            if let Err(details) = fs::remove_dir(&dest_path) {
                return Err(AxoassetError::LocalAssetRemoveFailed {
                    dest_path: dest_path.display().to_string(),
                    details,
                });
            }
        }

        Ok(())
    }

    /// Removes a directory and all of its contents
    pub fn remove_dir_all(dest: &str) -> Result<()> {
        let dest_path = PathBuf::from(dest);
        if dest_path.is_dir() {
            if let Err(details) = fs::remove_dir_all(&dest_path) {
                return Err(AxoassetError::LocalAssetRemoveFailed {
                    dest_path: dest_path.display().to_string(),
                    details,
                });
            }
        }

        Ok(())
    }

    /// Copies an asset from one location on the local filesystem to another
    pub fn copy(origin_path: &str, dest_dir: &str) -> Result<PathBuf> {
        LocalAsset::load(origin_path)?.write(dest_dir)
    }

    /// Get the current working directory
    pub fn current_dir() -> Result<Utf8PathBuf> {
        let cur_dir =
            std::env::current_dir().map_err(|details| AxoassetError::CurrentDir { details })?;
        let cur_dir = Utf8PathBuf::from_path_buf(cur_dir)
            .map_err(|details| AxoassetError::Utf8Path { path: details })?;
        Ok(cur_dir)
    }

    /// Find a desired file in the provided dir or an ancestor of it.
    ///
    /// On success returns the path to the found file.
    pub fn search_ancestors<'a>(
        start_dir: impl Into<&'a Utf8Path>,
        desired_filename: &str,
    ) -> Result<Utf8PathBuf> {
        let start_dir = start_dir.into();
        // We want a proper absolute path so we can compare paths to workspace roots easily.
        //
        // Also if someone starts the path with ./ we should trim that to avoid weirdness.
        // Maybe we should be using proper `canonicalize` but then we'd need to canonicalize
        // every path we get from random APIs to be consistent and that's a whole mess of its own!
        let start_dir = if let Ok(clean_dir) = start_dir.strip_prefix("./") {
            clean_dir.to_owned()
        } else {
            start_dir.to_owned()
        };
        let start_dir = if start_dir.is_relative() {
            let current_dir = LocalAsset::current_dir()?;
            current_dir.join(start_dir)
        } else {
            start_dir
        };
        for dir_path in start_dir.ancestors() {
            let file_path = dir_path.join(desired_filename);
            if file_path.is_file() {
                return Ok(file_path);
            }
        }
        Err(AxoassetError::SearchFailed {
            start_dir,
            desired_filename: desired_filename.to_owned(),
        })
    }

    /// Computes filename from provided origin path
    pub fn filename(origin_path: &str) -> Result<String> {
        if let Some(filename) = Path::new(origin_path).file_name() {
            if let Some(filename) = filename.to_str() {
                Ok(filename.to_string())
            } else {
                Err(AxoassetError::LocalAssetMissingFilename {
                    origin_path: origin_path.to_string(),
                })
            }
        } else {
            Err(AxoassetError::LocalAssetMissingFilename {
                origin_path: origin_path.to_string(),
            })
        }
    }

    /// Creates a new .tar.gz file from a provided directory
    pub fn tar_gz_dir(origin_dir: &str, dest_dir: &str) -> Result<()> {
        tar_dir(
            Utf8Path::new(origin_dir),
            Utf8Path::new(dest_dir),
            &CompressionImpl::Gzip,
        )
    }

    /// Creates a new .tar.xz file from a provided directory
    pub fn tar_xz_dir(origin_dir: &str, dest_dir: &str) -> Result<()> {
        tar_dir(
            Utf8Path::new(origin_dir),
            Utf8Path::new(dest_dir),
            &CompressionImpl::Xzip,
        )
    }

    /// Creates a new .tar.zstd file from a provided directory
    pub fn tar_zstd_dir(origin_dir: &str, dest_dir: &str) -> Result<()> {
        tar_dir(
            Utf8Path::new(origin_dir),
            Utf8Path::new(dest_dir),
            &CompressionImpl::Zstd,
        )
    }

    /// Creates a new .zip file from a provided directory
    pub fn zip_dir(origin_dir: &str, dest_dir: &str) -> Result<()> {
        zip_dir(Utf8Path::new(origin_dir), Utf8Path::new(dest_dir))
    }

    fn dest_path(&self, dest_dir: &str) -> Result<PathBuf> {
        Ok(Path::new(dest_dir).join(&self.filename))
    }
}
