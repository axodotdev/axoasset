use thiserror::Error;

pub type Result<T> = std::result::Result<T, AxoassetError>;

/// The set of errors that can occur when axoasset is used
#[derive(Debug, Error)]
pub enum AxoassetError {
    /// This error is a transparent error forwarded from the reqwest library.
    /// Long-term the goal is to eliminate this error variant in favor of more
    /// specific error variants.
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    /// This error is a transparent error forwarded from the URL library. This
    /// error indicates that the provided URL did not properly parse and may
    /// either be invalid or an unsupported format.
    #[error(transparent)]
    UrlParse(#[from] url::ParseError),

    /// This error is a transparent error forwarded from the reqwest library.
    /// This error indicates that the received headers were not able to be
    /// parsed into a string, which means they may be corrupted in some way.
    #[error(transparent)]
    ReqwestHeaderParse(#[from] reqwest::header::ToStrError),

    /// This error is a transparent error forwarded from the mime library.
    /// This error indicates that the given mime type was not able to be
    /// parsed into a string, which means it may be corrupted in some way.
    #[error(transparent)]
    MimeParseParse(#[from] mime::FromStrError),

    /// This error indicates that axoasset was asked to create a new remote
    /// asset, likely by being given an path that starts with http or https.
    /// Axoasset can only create new assets on the file system.
    #[error("failed to create asset at {origin_path}, because {origin_path} is a remote address. Axoasset cannot create remote assets; Did you mean to create a local asset? You can do so by passing a local path.")]
    CannotCreateRemoteAsset {
        /// The origin path of the asset, used as an identifier
        origin_path: String,
    },

    /// This error indicates that axoasset failed to fetch a remote asset.
    #[error("failed to fetch asset at {origin_path}: Encountered an error when requesting a remote asset. Make sure the url you prodived is accurate. Details:\r{details}")]
    RemoteAssetRequestFailed {
        /// The origin path of the asset, used as an identifier
        origin_path: String,
        /// Details of the error
        details: String,
    },

    /// This error indicates that axoasset failed to load a remote asset.
    #[error("failed to fetch asset at {origin_path}: Encountered an error when requesting a remote asset. Make sure the url you prodived is accurate. Details:\r{details}")]
    RemoteAssetLoadFailed {
        /// The origin path of the asset, used as an identifier
        origin_path: String,
        /// Details of the error
        details: String,
    },

    /// This error indicates that axoasset was given a url that used a protocol
    /// other than http or https, such as file://. Axoasset currently only
    /// supports http and https.
    #[error("remote asset url, {origin_path}, did not use http or https: Please use an http or https url or a local path.")]
    RemoteAssetPathSchemeNotSupported {
        /// The origin path of the asset, used as an identifier
        origin_path: String,
    },

    /// This error indicates that the mime type of the requested remote asset
    /// was not an image.  
    #[error("when fetching asset at {origin_path}, the server's response mime type did not indicate an image: Please make sure the asset url is correct and that the server is properly configured")]
    RemoteAssetNonImageMimeType {
        /// The origin path of the asset, used as an identifier
        origin_path: String,
    },

    /// This error indicates that axoasset failed to copy a remote asset.
    #[error("failed to copy asset from {origin_path} to {dest_path}: Encountered an error copying server response body to filesystem. Make sure your server is configured correctly and your destination path has the correct permissions. Details:\r{details}")]
    RemoteAssetCopyFailed {
        /// The origin path of the asset, used as an identifier
        origin_path: String,
        /// The path where the asset was being copied to
        dest_path: String,
        /// Details of the error
        details: String,
    },

    /// This error indicates that the mime type of the requested remote asset
    /// was of a type that axoasset does not support.
    #[error("when fetching asset at {origin_path}, the server responded with a mime type that was non supported: Please make sure the asset url is correct and that the server is properly configured")]
    RemoteAssetMimeTypeNotSupported {
        /// The origin path of the asset, used as an identifier
        origin_path: String,
        /// The mimetype from the server response
        mimetype: String,
    },

    /// This error indicates that the requested remote asset was an image, but
    /// axoasset could not determine what file extenstion to use for the
    /// received format.
    #[error("when fetching asset at {origin_path}, we could not determine an appropriate file extension based on the server response: Please make sure the asset url is correct and that the server is properly configured")]
    RemoteAssetIndeterminateImageFormatExtension {
        /// The origin path of the asset, used as an identifier
        origin_path: String,
    },

    /// This error indicates that the server response for the remote asset request
    /// did not include a content-type header. Axoasset needs the content-type
    /// header to determine what type of file the asset contains.
    #[error("when fetching asset at {origin_path}, the server's response did not contain a content type header: Please make sure the asset url is correct and that the server is properly configured")]
    RemoteAssetMissingContentTypeHeader {
        /// The origin path of the asset, used as an identifier
        origin_path: String,
    },

    /// This error indicates that the provided path was determined to be for a
    /// remote asset but could not be parsed into a valid URL.
    #[error("could not parse asset url, {origin_path}: Please use an http or https url or a local path. Details:\r{details}")]
    RemoteAssetPathParseError {
        /// The origin path of the asset, used as an identifier
        origin_path: String,
        /// Details of the error
        details: String,
    },

    /// This error indicates that axoasset failed to write a remote asset to the
    /// local filesystem.
    #[error("failed to write asset at {origin_path} to {dest_path}: Could not find asset at provided path. Make sure your path is correct and your server is configured correctly. Details:\r{details}")]
    RemoteAssetWriteFailed {
        /// The origin path of the asset, used as an identifier
        origin_path: String,
        /// The path where the asset was being written to
        dest_path: String,
        /// Details of the error
        details: String,
    },

    /// This error indicates that axoasset failed to fetch a local asset at the
    /// provided path.
    #[error("failed to fetch asset at {origin_path}: Could not find asset at provided path. Make sure your path is relative to your oranda config or project manifest file. Details:\r{details}")]
    LocalAssetNotFound {
        /// The origin path of the asset, used as an identifier
        origin_path: String,
        /// Details of the error
        details: String,
    },

    /// This error inidcates that axoasset failed to copy a local asset.
    #[error("failed to copy asset from {origin_path} to {dest_path}: Make sure your path is relative to your oranda config or project manifest file. Details:\r{details}")]
    LocalAssetCopyFailed {
        /// The origin path of the asset, used as an identifier
        origin_path: String,
        /// The path where the asset was being copied to
        dest_path: String,
        /// Details of the error
        details: String,
    },

    /// This error indicates that axoasset failed to read a local asset at the
    /// provided path.
    #[error("failed to read asset from {origin_path}: An error occured while reading the asset at provided path. Make sure your path is relative to your oranda config or project manifest file. Details:\r{details}")]
    LocalAssetReadFailed {
        /// The origin path of the asset, used as an identifier
        origin_path: String,
        /// Details of the error
        details: String,
    },

    /// This error indicates that axoasset failed to write a local asset.
    #[error("failed to write asset from {origin_path} to {dest_path}: Make sure your path is relative to your oranda config or project manifest file. Details:\r{details}")]
    LocalAssetWriteFailed {
        /// The origin path of the asset, used as an identifier
        origin_path: String,
        /// The path where the asset was being written to
        dest_path: String,
        /// Details of the error
        details: String,
    },

    /// This error indicates that axoasset could not determine the filename for
    /// a local asset.
    #[error("could not determine file name for asset at {origin_path}: Make sure your path is relative to your oranda config or project manifest file.")]
    LocalAssetMissingFilename {
        /// The origin path of the asset, used as an identifier
        origin_path: String,
    },
}
