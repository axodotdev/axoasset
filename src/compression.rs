//! Compression-related methods, all used in `axoasset::Local`

use camino::Utf8Path;

/// Internal tar-file compression algorithms
#[cfg(feature = "compression-tar")]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum CompressionImpl {
    /// .gz
    Gzip,
    /// .xz
    Xzip,
    /// .zstd
    Zstd,
}

#[cfg(feature = "compression-tar")]
pub(crate) fn tar_dir(
    src_path: &Utf8Path,
    dest_path: &Utf8Path,
    with_root: Option<&Utf8Path>,
    compression: &CompressionImpl,
) -> crate::error::Result<()> {
    use crate::error::*;
    use flate2::{Compression, GzBuilder};
    use std::fs;
    use xz2::write::XzEncoder;
    use zstd::stream::Encoder as ZstdEncoder;

    // Set up the archive/compression
    // dir_name here is a prefix directory/path that the src dir's contents will be stored
    // under when being tarred. Having it be empty means the contents
    // will be placed in the root of the tarball.
    let dir_name = with_root.unwrap_or_else(|| Utf8Path::new(""));
    let zip_contents_name = format!("{}.tar", dest_path.file_name().unwrap());
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
            let zip_output = ZstdEncoder::new(final_zip_file, 0).map_err(|details| {
                AxoassetError::LocalAssetArchive {
                    reason: format!("failed to create zstd encoder"),
                    details,
                }
            })?;

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

#[cfg(feature = "compression-zip")]
pub(crate) fn zip_dir(
    src_path: &Utf8Path,
    dest_path: &Utf8Path,
    with_root: Option<&Utf8Path>,
) -> zip::result::ZipResult<()> {
    use std::{
        fs::File,
        io::{Read, Write},
    };
    use zip::{write::FileOptions, CompressionMethod};

    let file = File::create(dest_path)?;

    // The `zip` crate lacks the conveniences of the `tar` crate so we need to manually
    // walk through all the subdirs of `src_path` and copy each entry. walkdir streamlines
    // that process for us.
    let walkdir = crate::dirs::walk_dir(src_path);
    let it = walkdir.into_iter();

    let mut zip = zip::ZipWriter::new(file);
    let options = FileOptions::default().compression_method(CompressionMethod::STORE);

    // If there's a root prefix, add entries for all of its components
    if let Some(root) = with_root {
        for path in root.ancestors() {
            if !path.as_str().is_empty() {
                zip.add_directory(path.as_str(), options)?;
            }
        }
    }

    let mut buffer = Vec::new();
    for entry in it.filter_map(|e| e.ok()) {
        let name = &entry.rel_path;
        let path = &entry.full_path;
        // Optionally apply the root prefix
        let name = if let Some(root) = with_root {
            root.join(name)
        } else {
            name.to_owned()
        };

        // Write file or directory explicitly
        // Some unzip tools unzip files with directory paths correctly, some do not!
        if path.is_file() {
            zip.start_file(name, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
            buffer.clear();
        } else if !name.as_str().is_empty() {
            // Only if not root! Avoids path spec / warning
            // and mapname conversion failed error on unzip
            zip.add_directory(name, options)?;
        }
    }
    zip.finish()?;
    Ok(())
}
