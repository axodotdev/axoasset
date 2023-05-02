#![deny(missing_docs)]

//! # axoasset
//! > 📮 load, write, and copy remote and local assets
//!
//! this library is a utility focused on managing both local (filesystem) assets
//! and remote (via http/https) assets. the bulk of the logic is not terribly
//! interesting or uniquely engineered; the purpose this library is primarily
//! to unify and co-locate the logic to make debugging simpler and error handling
//! more consistent and comprehensive.

use std::path::PathBuf;

#[cfg(any(
    feature = "compression",
    feature = "compression-zip",
    feature = "compression-tar"
))]
pub(crate) mod compression;
pub(crate) mod error;
pub(crate) mod local;
#[cfg(feature = "remote")]
pub(crate) mod remote;
pub(crate) mod source;
pub(crate) mod spanned;

pub use error::AxoassetError;
use error::Result;
pub use local::LocalAsset;
#[cfg(feature = "remote")]
pub use remote::RemoteAsset;
pub use source::SourceFile;
pub use spanned::Spanned;

/// An asset can either be a local asset, which is designated by a path on the
/// local file system, or a remote asset, which is designated by an http or
/// https url.
#[derive(Debug)]
pub enum Asset {
    /// An asset is a local asset if it is located on the local filesystem
    LocalAsset(LocalAsset),
    /// An asset is a remote asset if it is located at a http or https URL
    #[cfg(feature = "remote")]
    RemoteAsset(RemoteAsset),
}

impl Asset {
    /// Creates a new local asset. Does not write to filesystem. Will fail if
    /// passed a URL.
    pub fn new(origin_path: &str, contents: Vec<u8>) -> Result<Asset> {
        if is_remote(origin_path)? {
            Err(AxoassetError::CannotCreateRemoteAsset {
                origin_path: origin_path.to_string(),
            })
        } else {
            Ok(Asset::LocalAsset(LocalAsset::new(origin_path, contents)?))
        }
    }

    /// Loads an asset, either locally or remotely, returning an Asset enum
    /// variant containing the contents as bytes.
    pub async fn load(origin_path: &str) -> Result<Asset> {
        #[cfg(feature = "remote")]
        if is_remote(origin_path)? {
            return Ok(Asset::RemoteAsset(RemoteAsset::load(origin_path).await?));
        }
        Ok(Asset::LocalAsset(LocalAsset::load(origin_path)?))
    }

    /// Loads an asset, returning its contents as a String.
    pub async fn load_string(origin_path: &str) -> Result<String> {
        #[cfg(feature = "remote")]
        if is_remote(origin_path)? {
            return RemoteAsset::load_string(origin_path).await;
        }
        LocalAsset::load_string(origin_path)
    }

    /// Loads an asset, returning its contents as a vector of bytes.
    pub async fn load_bytes(origin_path: &str) -> Result<Vec<u8>> {
        #[cfg(feature = "remote")]
        if is_remote(origin_path)? {
            return RemoteAsset::load_bytes(origin_path).await;
        }
        LocalAsset::load_bytes(origin_path)
    }

    /// Copies an asset, returning the path to the copy destination on the
    /// local filesystem.
    pub async fn copy(origin_path: &str, dest_dir: &str) -> Result<PathBuf> {
        #[cfg(feature = "remote")]
        if is_remote(origin_path)? {
            return RemoteAsset::copy(origin_path, dest_dir).await;
        }
        LocalAsset::copy(origin_path, dest_dir)
    }

    /// Writes an asset, returning the path to the write destination on the
    /// local filesystem.
    pub async fn write(self, dest_dir: &str) -> Result<PathBuf> {
        match self {
            #[cfg(feature = "remote")]
            Asset::RemoteAsset(a) => a.write(dest_dir).await,
            Asset::LocalAsset(a) => a.write(dest_dir),
        }
    }
}

fn is_remote(origin_path: &str) -> Result<bool> {
    if origin_path.starts_with("http") {
        match origin_path.parse() {
            Ok(url) => {
                if is_http(url) {
                    Ok(true)
                } else {
                    Err(AxoassetError::RemoteAssetPathSchemeNotSupported {
                        origin_path: origin_path.to_string(),
                    })
                }
            }
            Err(details) => Err(AxoassetError::RemoteAssetPathParseError {
                origin_path: origin_path.to_string(),
                details,
            }),
        }
    } else {
        Ok(false)
    }
}

fn is_http(url: url::Url) -> bool {
    url.scheme() == "https" || url.scheme() == "http"
}
