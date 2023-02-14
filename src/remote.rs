use std::fs;
use std::path::{Path, PathBuf};

use crate::error::*;

/// A remote asset is an asset that is fetched over the network.
#[derive(Debug)]
pub struct RemoteAsset {
    /// A string containing a valid filename and extension. The filename is
    /// determined by the origin path and the content-type headers from the
    /// server response.
    pub filename: String,
    /// A string containing a http or https URL pointing to the asset. This does
    /// not need to be `https://origin.com/myfile.ext` as filename is determined by
    /// content-type headers in the server response.
    pub origin_path: String,
    /// The contents of the asset as a vector of bytes
    pub contents: Vec<u8>,
}

impl RemoteAsset {
    /// Loads an asset from a URL and returns a RemoteAsset struct.
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

    /// Loads an asset from a URL and returns a String of the asset's contents.
    pub async fn load_string(origin_path: &str) -> Result<String> {
        match reqwest::get(origin_path).await {
            Ok(response) => Ok(response.text().await?),
            Err(details) => Err(AxoassetError::RemoteAssetRequestFailed {
                origin_path: origin_path.to_string(),
                details: details.to_string(),
            }),
        }
    }

    /// Loads an asset from a URL and returns a vector of bytes of the asset's contents.
    pub async fn load_bytes(origin_path: &str) -> Result<Vec<u8>> {
        match reqwest::get(origin_path).await {
            Ok(response) => Ok(response.bytes().await?.to_vec()),
            Err(details) => Err(AxoassetError::RemoteAssetRequestFailed {
                origin_path: origin_path.to_string(),
                details: details.to_string(),
            }),
        }
    }

    /// Copies an asset to the local filesystem.
    pub async fn copy(origin_path: &str, dest_dir: &str) -> Result<PathBuf> {
        match RemoteAsset::load(origin_path).await {
            Ok(a) => {
                let dest_path = Path::new(dest_dir).join(a.filename);
                match fs::write(&dest_path, a.contents) {
                    Ok(_) => Ok(dest_path),
                    Err(details) => Err(AxoassetError::RemoteAssetWriteFailed {
                        origin_path: origin_path.to_string(),
                        dest_path: dest_path.display().to_string(),
                        details: details.to_string(),
                    }),
                }
            }
            Err(details) => Err(AxoassetError::RemoteAssetLoadFailed {
                origin_path: origin_path.to_string(),
                details: details.to_string(),
            }),
        }
    }

    /// Writes an asset to the local filesystem
    pub async fn write(self, dest_dir: &str) -> Result<PathBuf> {
        let dest_path = Path::new(dest_dir).join(self.filename);
        match fs::write(&dest_path, self.contents) {
            Ok(_) => Ok(dest_path),
            Err(details) => Err(AxoassetError::RemoteAssetWriteFailed {
                origin_path: self.origin_path,
                dest_path: dest_path.display().to_string(),
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
                    mime::TEXT => Ok(mtype),
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
        match mimetype.type_() {
            mime::IMAGE => RemoteAsset::image_extension(mimetype, origin_path),
            mime::TEXT => RemoteAsset::text_extension(mimetype, origin_path),
            _ => Err(AxoassetError::RemoteAssetMimeTypeNotSupported {
                origin_path: origin_path.to_string(),
                mimetype: mimetype.to_string(),
            }),
        }
    }

    fn text_extension(mimetype: mime::Mime, origin_path: &str) -> Result<String> {
        if let Some(extension) = mimetype.suffix() {
            Ok(extension.to_string())
        } else {
            match mimetype.subtype() {
                mime::PLAIN => Ok("txt".to_string()),
                mime::CSS => Ok("css".to_string()),
                _ => Err(AxoassetError::RemoteAssetMimeTypeNotSupported {
                    origin_path: origin_path.to_string(),
                    mimetype: mimetype.to_string(),
                }),
            }
        }
    }

    fn image_extension(mimetype: mime::Mime, origin_path: &str) -> Result<String> {
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

    // TODO: https://github.com/axodotdev/axoasset/issues/6
    // TODO: https://github.com/axodotdev/axoasset/issues/9
    // Currently, this function will take an asset's origin path, and attempt
    // to identify if the final segment of the URL is a filename.
    //
    // If it does not find a filename it will drop the host from the origin
    // url, slugify the set of the path, and then add an extension based on the
    // Mime type in the associated response headers.
    //
    // A large portion of the origin path is preserved in the filename to help
    // avoid name conflicts, but this is a half measure at best and leaves a
    // lot of room for improvment.
    fn filename(origin_path: &str, headers: &reqwest::header::HeaderMap) -> Result<String> {
        let mut filestem = url::Url::parse(origin_path)?
            .path()
            .to_string()
            .replace('/', "_");
        filestem.remove(0);
        if filestem.contains('.') {
            Ok(filestem)
        } else {
            let extension =
                RemoteAsset::extension(RemoteAsset::mimetype(headers, origin_path)?, origin_path)?;
            Ok(format!("{filestem}.{extension}"))
        }
    }
}
