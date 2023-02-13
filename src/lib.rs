use std::path::PathBuf;

pub(crate) mod error;
pub(crate) mod local;
pub(crate) mod remote;

pub use error::AxoassetError;
use error::Result;
pub use local::LocalAsset;
pub use remote::RemoteAsset;

pub enum Asset {
    LocalAsset(LocalAsset),
    RemoteAsset(RemoteAsset),
}

impl Asset {
    pub fn new(origin_path: &str, contents: Vec<u8>) -> Result<Asset> {
        if is_remote(origin_path)? {
            Err(AxoassetError::CannotCreateRemoteAsset {
                origin_path: origin_path.to_string(),
            })
        } else {
            Ok(Asset::LocalAsset(LocalAsset::new(origin_path, contents)))
        }
    }

    pub async fn load(origin_path: &str) -> Result<Asset> {
        if is_remote(origin_path)? {
            Ok(Asset::RemoteAsset(RemoteAsset::load(origin_path).await?))
        } else {
            Ok(Asset::LocalAsset(LocalAsset::load(origin_path)?))
        }
    }

    pub async fn load_string(origin_path: &str) -> Result<String> {
        if is_remote(origin_path)? {
            Ok(RemoteAsset::load_string(origin_path).await?)
        } else {
            Ok(LocalAsset::load_string(origin_path)?)
        }
    }

    pub async fn load_bytes(origin_path: &str) -> Result<Vec<u8>> {
        if is_remote(origin_path)? {
            Ok(RemoteAsset::load_bytes(origin_path).await?)
        } else {
            Ok(LocalAsset::load_bytes(origin_path)?)
        }
    }

    pub async fn copy(origin_path: &str, dest_dir: &str) -> Result<PathBuf> {
        if is_remote(origin_path)? {
            RemoteAsset::copy(origin_path, dest_dir).await
        } else {
            LocalAsset::copy(origin_path, dest_dir)
        }
    }

    pub async fn write(self, dest_dir: &str) -> Result<PathBuf> {
        match self {
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
                details: details.to_string(),
            }),
        }
    } else {
        Ok(false)
    }
}

fn is_http(url: url::Url) -> bool {
    url.scheme() == "https" || url.scheme() == "http"
}
