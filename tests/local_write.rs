#![allow(irrefutable_let_patterns)]

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use assert_fs::prelude::*;
use image::ImageFormat;

#[test]
fn it_writes_a_new_file_from_string() {
    let dest = assert_fs::TempDir::new().unwrap();
    let dest_dir = Path::new(dest.to_str().unwrap());

    let filename = "contents.txt";
    let contents = "CONTENTS";
    axoasset::LocalAsset::write_new(contents, filename, &dest_dir.display().to_string()).unwrap();
    let written_file = dest_dir.join(filename);
    assert!(written_file.exists());

    let loaded_contents =
        axoasset::LocalAsset::load_string(&written_file.display().to_string()).unwrap();
    assert!(loaded_contents.contains(contents));
}

#[tokio::test]
async fn it_writes_local_assets() {
    let origin = assert_fs::TempDir::new().unwrap();
    let dest = assert_fs::TempDir::new().unwrap();
    let dest_dir = Path::new(dest.to_str().unwrap());

    let mut files = HashMap::new();
    files.insert("README.md", "# axoasset");
    files.insert("styles.css", "@import");
    files.insert("logo.png", "");

    for (file, contents) in files {
        let asset = origin.child(file);
        let content = Path::new("./tests/assets").join(file);
        asset.write_file(&content).unwrap();

        let origin_path = asset.to_str().unwrap();
        let loaded_asset = axoasset::Asset::load(origin_path).await.unwrap();

        if let axoasset::Asset::LocalAsset(asset) = loaded_asset {
            asset.write(dest.to_str().unwrap()).unwrap();
            let written_file = dest_dir.join(file);
            assert!(written_file.exists());
            if asset.origin_path.contains("png") {
                let format = ImageFormat::from_path(written_file).unwrap();
                assert_eq!(format, ImageFormat::Png);
            } else {
                fs::read_to_string(written_file).unwrap().contains(contents);
            }
        }
    }
}
