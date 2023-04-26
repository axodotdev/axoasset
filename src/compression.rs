//! Compression-related methods, all used in `axoasset::Local`

use crate::error::*;
use camino::Utf8Path;
use flate2::{write::ZlibEncoder, Compression, GzBuilder};
use std::{
    fs::{self, DirEntry},
    io::BufReader,
};
use xz2::write::XzEncoder;
use zip::ZipWriter;

/// Internal tar-file compression algorithms
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum CompressionImpl {
    /// .gz
    Gzip,
    /// .xz
    Xzip,
    /// .zstd
    Zstd,
}

pub(crate) fn tar_dir(
    src_path: &Utf8Path,
    dest_path: &Utf8Path,
    compression: &CompressionImpl,
) -> Result<()> {
    // Set up the archive/compression
    // The contents of the zip (e.g. a tar)
    let dir_name = src_path.file_name().unwrap();
    let zip_contents_name = format!("{dir_name}.tar");
    let final_zip_file = match fs::File::create(dest_path) {
        Ok(file) => file,
        Err(details) => {
            return Err(AxoassetError::LocalAssetWriteNewFailed {
                dest_path: dest_path.to_string(),
                details,
            })
        }
    };

    match compression {
        CompressionImpl::Gzip => {
            // Wrap our file in compression
            let zip_output = GzBuilder::new()
                .filename(zip_contents_name)
                .write(final_zip_file, Compression::default());

            // Write the tar to the compression stream
            let mut tar = tar::Builder::new(zip_output);

            // Add the whole dir to the tar
            if let Err(details) = tar.append_dir_all(dir_name, src_path) {
                return Err(AxoassetError::LocalAssetArchive {
                    reason: format!("failed to copy directory into tar: {src_path} => {dir_name}",),
                    details,
                });
            }
            // Finish up the tarring
            let zip_output = match tar.into_inner() {
                Ok(out) => out,
                Err(details) => {
                    return Err(AxoassetError::LocalAssetArchive {
                        reason: format!("failed to write tar: {dest_path}"),
                        details,
                    })
                }
            };
            // Finish up the compression
            let _zip_file = match zip_output.finish() {
                Ok(file) => file,
                Err(details) => {
                    return Err(AxoassetError::LocalAssetArchive {
                        reason: format!("failed to write archive: {dest_path}"),
                        details,
                    })
                }
            };
            // Drop the file to close it
        }
        CompressionImpl::Xzip => {
            let zip_output = XzEncoder::new(final_zip_file, 9);
            // Write the tar to the compression stream
            let mut tar = tar::Builder::new(zip_output);

            // Add the whole dir to the tar
            if let Err(details) = tar.append_dir_all(dir_name, src_path) {
                return Err(AxoassetError::LocalAssetArchive {
                    reason: format!("failed to copy directory into tar: {src_path} => {dir_name}",),
                    details,
                });
            }
            // Finish up the tarring
            let zip_output = match tar.into_inner() {
                Ok(out) => out,
                Err(details) => {
                    return Err(AxoassetError::LocalAssetArchive {
                        reason: format!("failed to write tar: {dest_path}"),
                        details,
                    })
                }
            };
            // Finish up the compression
            let _zip_file = match zip_output.finish() {
                Ok(file) => file,
                Err(details) => {
                    return Err(AxoassetError::LocalAssetArchive {
                        reason: format!("failed to write archive: {dest_path}"),
                        details,
                    })
                }
            };
            // Drop the file to close it
        }
        CompressionImpl::Zstd => {
            // Wrap our file in compression
            let zip_output = ZlibEncoder::new(final_zip_file, Compression::default());

            // Write the tar to the compression stream
            let mut tar = tar::Builder::new(zip_output);

            // Add the whole dir to the tar
            if let Err(details) = tar.append_dir_all(dir_name, src_path) {
                return Err(AxoassetError::LocalAssetArchive {
                    reason: format!("failed to copy directory into tar: {src_path} => {dir_name}",),
                    details,
                });
            }
            // Finish up the tarring
            let zip_output = match tar.into_inner() {
                Ok(out) => out,
                Err(details) => {
                    return Err(AxoassetError::LocalAssetArchive {
                        reason: format!("failed to write tar: {dest_path}"),
                        details,
                    })
                }
            };
            // Finish up the compression
            let _zip_file = match zip_output.finish() {
                Ok(file) => file,
                Err(details) => {
                    return Err(AxoassetError::LocalAssetArchive {
                        reason: format!("failed to write archive: {dest_path}"),
                        details,
                    })
                }
            };
            // Drop the file to close it
        }
    }

    Ok(())
}

pub(crate) fn zip_dir(src_path: &Utf8Path, dest_path: &Utf8Path) -> Result<()> {
    // Set up the archive/compression
    let final_zip_file = match fs::File::create(dest_path) {
        Ok(file) => file,
        Err(details) => {
            return Err(AxoassetError::LocalAssetWriteNewFailed {
                dest_path: dest_path.to_string(),
                details,
            })
        }
    };

    // Wrap our file in compression
    let mut zip = ZipWriter::new(final_zip_file);

    let dir = match std::fs::read_dir(src_path) {
        Ok(dir) => dir,
        Err(details) => {
            return Err(AxoassetError::LocalAssetReadFailed {
                origin_path: src_path.to_string(),
                details,
            })
        }
    };

    for entry in dir {
        if let Err(details) = copy_into_zip(entry, &mut zip) {
            return Err(AxoassetError::LocalAssetArchive {
                reason: format!("failed to create file in zip: {dest_path}"),
                details,
            });
        }
    }

    // Finish up the compression
    let _zip_file = match zip.finish() {
        Ok(file) => file,
        Err(details) => {
            return Err(AxoassetError::LocalAssetArchive {
                reason: format!("failed to write archive: {dest_path}"),
                details: details.into(),
            })
        }
    };
    // Drop the file to close it
    Ok(())
}

/// Copies a file into a provided `ZipWriter`. Mostly factored out so that we can bunch up
/// a bunch of `std::io::Error`s without having to individually handle them.
fn copy_into_zip(
    entry: std::result::Result<DirEntry, std::io::Error>,
    zip: &mut ZipWriter<fs::File>,
) -> std::result::Result<(), std::io::Error> {
    let entry = entry?;
    if entry.file_type()?.is_file() {
        let options =
            zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);
        let file = fs::File::open(entry.path())?;
        let mut buf = BufReader::new(file);
        let file_name = entry.file_name();
        // FIXME: ...don't do this lossy conversion?
        let utf8_file_name = file_name.to_string_lossy();
        zip.start_file(utf8_file_name.clone(), options)?;
        std::io::copy(&mut buf, zip)?;
    } else {
        todo!("implement zip subdirs! (or was this a symlink?)");
    }
    Ok(())
}
