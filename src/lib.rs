use std::path::PathBuf;

pub mod error;
pub(crate) mod local;
pub(crate) mod remote;

use error::*;

pub enum Asset {
    LocalAsset(local::LocalAsset),
    RemoteAsset(remote::RemoteAsset),
}

pub async fn load(origin_path: &str) -> Result<Asset> {
    if is_remote(origin_path)? {
        Ok(Asset::RemoteAsset(
            remote::RemoteAsset::load(origin_path).await?,
        ))
    } else {
        Ok(Asset::LocalAsset(local::LocalAsset::load(origin_path)?))
    }
}

pub async fn copy(origin_path: &str, dist_dir: &str) -> Result<PathBuf> {
    if is_remote(origin_path)? {
        remote::RemoteAsset::copy(origin_path, dist_dir).await
    } else {
        local::LocalAsset::copy(origin_path, dist_dir)
    }
}

pub async fn write(asset: Asset, dist_dir: &str) -> Result<PathBuf> {
    match asset {
        Asset::RemoteAsset(a) => a.write(dist_dir).await,
        Asset::LocalAsset(a) => a.write(dist_dir),
    }
}

fn is_remote(origin_path: &str) -> Result<bool> {
    if origin_path.starts_with("http") {
        println!("remote asset detected");
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
