#![allow(irrefutable_let_patterns)]

use std::collections::HashMap;

use assert_fs::prelude::*;
use camino::Utf8Path;

#[tokio::test]
async fn it_copies_local_assets() {
    let origin = assert_fs::TempDir::new().unwrap();
    let dest = assert_fs::TempDir::new().unwrap();
    let dest_dir = Utf8Path::from_path(dest.path()).unwrap();

    let mut files = HashMap::new();
    files.insert("README.md", "# axoasset");
    files.insert("styles.css", "@import");

    for (file, contents) in files {
        let asset = origin.child(file);
        asset.write_str(contents).unwrap();

        axoasset::Asset::copy(asset.to_str().unwrap(), dest.to_str().unwrap())
            .await
            .unwrap();

        let copied_file = dest_dir.join(file);
        assert!(copied_file.exists());
        let loaded_asset = axoasset::Asset::load(copied_file.as_str()).await.unwrap();
        if let axoasset::Asset::LocalAsset(asset) = loaded_asset {
            assert!(std::str::from_utf8(&asset.contents)
                .unwrap()
                .contains(contents));
        }
    }
}

#[tokio::test]
async fn it_copies_named_local_assets() {
    let origin = assert_fs::TempDir::new().unwrap();
    let dest = assert_fs::TempDir::new().unwrap();
    let dest_dir = Utf8Path::from_path(dest.path()).unwrap();

    let mut files = HashMap::new();
    files.insert("README.md", "# axoasset");
    files.insert("styles.css", "@import");

    for (file, contents) in files {
        let asset = origin.child(file);
        asset.write_str(contents).unwrap();

        let origin_path = asset.to_str().unwrap();
        axoasset::LocalAsset::copy_named(origin_path, dest_dir.join(file)).unwrap();

        let copied_file = dest_dir.join(file);
        assert!(copied_file.exists());
        let loaded_asset = axoasset::LocalAsset::load(copied_file).unwrap();
        assert!(std::str::from_utf8(&loaded_asset.contents)
            .unwrap()
            .contains(contents));
    }
}

#[tokio::test]
async fn it_copies_dirs() {
    let origin = assert_fs::TempDir::new().unwrap().child("result");
    let dest = assert_fs::TempDir::new().unwrap();
    let origin_dir = Utf8Path::from_path(origin.path()).unwrap();
    let dest_dir = Utf8Path::from_path(dest.path()).unwrap();
    origin.create_dir_all().unwrap();

    // None means it's just a dir, used to make sure empty dirs get copied
    let mut files = HashMap::new();
    files.insert("blah/blargh/README3.md", Some("# axoasset3"));
    files.insert("blah/README2.md", Some("# axoasset2"));
    files.insert("blah/README.md", Some("# axoasset"));
    files.insert("styles.css", Some("@import"));
    files.insert("blah/blargh/empty_dir", None);
    files.insert("empty/dirs", None);
    files.insert("root_empty", None);

    for (file, contents) in &files {
        let asset = origin.child(file);
        if let Some(contents) = contents {
            std::fs::create_dir_all(asset.parent().unwrap()).unwrap();
            asset.write_str(contents).unwrap();
        } else {
            asset.create_dir_all().unwrap();
        }
    }

    axoasset::LocalAsset::copy_dir(origin_dir, dest_dir).unwrap();

    for (file, contents) in &files {
        let copied_file = dest_dir.join("result").join(file);

        assert!(copied_file.exists());
        if let Some(contents) = contents {
            let loaded_asset = axoasset::LocalAsset::load(copied_file).unwrap();
            assert!(std::str::from_utf8(&loaded_asset.contents)
                .unwrap()
                .contains(contents));
        }
    }
}

#[tokio::test]
async fn it_copies_named_dirs() {
    let origin = assert_fs::TempDir::new().unwrap();
    let dest = assert_fs::TempDir::new().unwrap();
    let origin_dir = Utf8Path::from_path(origin.path()).unwrap();
    let dest_dir = Utf8Path::from_path(dest.path()).unwrap().join("result");

    // None means it's just a dir, used to make sure empty dirs get copied
    let mut files = HashMap::new();
    files.insert("blah/blargh/README3.md", Some("# axoasset3"));
    files.insert("blah/README2.md", Some("# axoasset2"));
    files.insert("blah/README.md", Some("# axoasset"));
    files.insert("styles.css", Some("@import"));
    files.insert("blah/blargh/empty_dir", None);
    files.insert("empty/dirs", None);
    files.insert("root_empty", None);

    for (file, contents) in &files {
        let asset = origin.child(file);
        if let Some(contents) = contents {
            std::fs::create_dir_all(asset.parent().unwrap()).unwrap();
            asset.write_str(contents).unwrap();
        } else {
            asset.create_dir_all().unwrap();
        }
    }

    axoasset::LocalAsset::copy_dir_named(origin_dir, &dest_dir).unwrap();

    for (file, contents) in &files {
        let copied_file = dest_dir.join(file);

        assert!(copied_file.exists());
        if let Some(contents) = contents {
            let loaded_asset = axoasset::LocalAsset::load(copied_file).unwrap();
            assert!(std::str::from_utf8(&loaded_asset.contents)
                .unwrap()
                .contains(contents));
        }
    }
}
