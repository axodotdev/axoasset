use std::fs;
use std::path::{Path, PathBuf};

use crate::error::*;

#[derive(Debug)]
pub struct RemoteAsset {
    pub filename: String,
    pub origin_path: String,
    pub contents: Vec<u8>,
}

impl RemoteAsset {
    pub async fn load(origin_path: &str) -> Result<RemoteAsset> {
        match reqwest::get(origin_path).await {
            Ok(response) => {
                let filename = RemoteAsset::filename(origin_path, response.headers())?;
                Ok(RemoteAsset {
                    origin_path: origin_path.to_string(),
                    contents: response.bytes().await?.to_vec(),
                    filename,
                })
            }
            Err(details) => Err(AxoassetError::RemoteAssetRequestFailed {
                origin_path: origin_path.to_string(),
                details: details.to_string(),
            }),
        }
    }

    pub async fn copy(origin_path: &str, dist_dir: &str) -> Result<PathBuf> {
        match RemoteAsset::load(origin_path).await {
            Ok(a) => {
                let dist_path = Path::new(dist_dir).join(a.filename);
                fs::write(&dist_path, a.contents)?;
                Ok(dist_path)
            }
            Err(details) => Err(AxoassetError::RemoteAssetLoadFailed {
                origin_path: origin_path.to_string(),
                details: details.to_string(),
            }),
        }
    }

    pub async fn write(self, dist_dir: &str) -> Result<PathBuf> {
        let dist_path = Path::new(dist_dir).join(self.filename);
        match fs::write(&dist_path, self.contents) {
            Ok(_) => Ok(dist_path),
            Err(details) => Err(AxoassetError::RemoteAssetWriteFailed {
                origin_path: self.origin_path,
                dist_path: dist_path.display().to_string(),
                details: details.to_string(),
            }),
        }
    }

    fn mimetype(headers: &reqwest::header::HeaderMap, origin_path: &str) -> Result<mime::Mime> {
        match headers.get(reqwest::header::CONTENT_TYPE) {
            Some(content_type) => {
                let mtype: mime::Mime = content_type.to_str()?.parse()?;
                match mtype.type_() {
                    mime::IMAGE => Ok(mtype),
                    _ => Err(AxoassetError::RemoteAssetNonImageMimeType {
                        origin_path: origin_path.to_string(),
                    }),
                }
            }
            None => Err(AxoassetError::RemoteAssetMissingContentTypeHeader {
                origin_path: origin_path.to_string(),
            }),
        }
    }

    fn extension(mimetype: mime::Mime, origin_path: &str) -> Result<String> {
        if let Some(img_format) = image::ImageFormat::from_mime_type(&mimetype) {
            let extensions = img_format.extensions_str();
            if !extensions.is_empty() {
                Ok(extensions[0].to_string())
            } else {
                Err(
                    AxoassetError::RemoteAssetIndeterminateImageFormatExtension {
                        origin_path: origin_path.to_string(),
                    },
                )
            }
        } else {
            Err(AxoassetError::RemoteAssetMimeTypeNotSupported {
                origin_path: origin_path.to_string(),
                mimetype: mimetype.to_string(),
            })
        }
    }

    fn filename(origin_path: &str, headers: &reqwest::header::HeaderMap) -> Result<String> {
        let filestem = url::Url::parse(origin_path)?
            .path()
            .to_string()
            .replace('/', "_");
        let extension =
            RemoteAsset::extension(RemoteAsset::mimetype(headers, origin_path)?, origin_path)?;
        Ok(format!("{filestem}.{extension}"))
    }
}
