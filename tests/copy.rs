use std::fs;
use std::path::PathBuf;

use assert_fs::prelude::*;

use axoasset;

#[test]
fn it_copies_local_assets() {
    let origin = assert_fs::TempDir::new().unwrap();
    let dest = assert_fs::TempDir::new().unwrap();

    let text_asset = origin.child("hello.txt");
    text_asset.touch().unwrap();

    let img_asset = origin.child("profile.jpg");
    img_asset.touch().unwrap();

    let assets = vec![text_asset.to_str(), img_asset.to_str()];
    
    for asset in assets {
        let asset = asset.unwrap();
        axoasset::copy(asset, "local assets", dest.to_str().unwrap());
    }

    let contents: Vec<PathBuf> = fs::read_dir(dest).unwrap().map(|e| e.unwrap().path()).collect();
    assert!(contents.contains(&text_asset.to_path_buf()));
    assert!(contents.contains(&img_asset.to_path_buf()));
}
