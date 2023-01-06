use thiserror::Error;

pub type Result<T> = std::result::Result<T, AxoassetError>;

#[derive(Debug, Error)]
pub enum AxoassetError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    UrlParse(#[from] url::ParseError),

    #[error(transparent)]
    ReqwestHeaderParse(#[from] reqwest::header::ToStrError),

    #[error(transparent)]
    MimeParseParse(#[from] mime::FromStrError),

    #[error("failed to fetch asset at {origin_path}: Encountered an error when requesting a remote asset. Make sure the url you prodived is accurate. Details:\r{details}")]
    RemoteAssetRequestFailed {
        origin_path: String,
        details: String,
    },

    #[error("failed to fetch asset at {origin_path}: Encountered an error when requesting a remote asset. Make sure the url you prodived is accurate. Details:\r{details}")]
    RemoteAssetLoadFailed {
        origin_path: String,
        details: String,
    },

    #[error("remote asset url, {origin_path}, did not use http or https: Please use an http or https url or a local path.")]
    RemoteAssetPathSchemeNotSupported { origin_path: String },

    #[error("when fetching asset at {origin_path}, the server's response mime type did not indicate an image: Please make sure the asset url is correct and that the server is properly configured")]
    RemoteAssetNonImageMimeType { origin_path: String },

    #[error("failed to copy asset from {origin_path} to {dist_path}: Encountered an error copying server response body to filesystem. Make sure your server is configured correctly and your destination path has the correct permissions. Details:\r{details}")]
    RemoteAssetCopyFailed {
        origin_path: String,
        dist_path: String,
        details: String,
    },

    #[error("when fetching asset at {origin_path}, the server responded with a mime type that was non supported: Please make sure the asset url is correct and that the server is properly configured")]
    RemoteAssetMimeTypeNotSupported {
        origin_path: String,
        mimetype: String,
    },

    #[error("when fetching asset at {origin_path}, we could not determine an appropriate file extension based on the server response: Please make sure the asset url is correct and that the server is properly configured")]
    RemoteAssetIndeterminateImageFormatExtension { origin_path: String },

    #[error("when fetching asset at {origin_path}, the server's response did not contain a content type header: Please make sure the asset url is correct and that the server is properly configured")]
    RemoteAssetMissingContentTypeHeader { origin_path: String },

    #[error("could not parse asset url, {origin_path}: Please use an http or https url or a local path. Details:\r{details}")]
    RemoteAssetPathParseError {
        origin_path: String,
        details: String,
    },

    #[error("failed to write asset at {origin_path} to {dist_path}: Could not find asset at provided path. Make sure your path is correct and your server is configured correctly. Details:\r{details}")]
    RemoteAssetWriteFailed {
        origin_path: String,
        dist_path: String,
        details: String,
    },

    #[error("failed to fetch asset at {origin_path}: Could not find asset at provided path. Make sure your path is relative to your oranda config or project manifest file. Details:\r{details}")]
    LocalAssetNotFound {
        origin_path: String,
        details: String,
    },

    #[error("failed to copy asset from {origin_path} to {dist_path}: Could not find asset at provided path. Make sure your path is relative to your oranda config or project manifest file. Details:\r{details}")]
    LocalAssetCopyFailed {
        origin_path: String,
        dist_path: String,
        details: String,
    },

    #[error("failed to copy asset from {origin_path} to {dist_path}: Could not find asset at provided path. Make sure your path is relative to your oranda config or project manifest file. Details:\r{details}")]
    LocalAssetWriteFailed {
        origin_path: String,
        dist_path: String,
        details: String,
    },

    #[error("could not determine file name for asset at {origin_path}: Make sure your path is relative to your oranda config or project manifest file.")]
    LocalAssetMissingFilename { origin_path: String },
}
