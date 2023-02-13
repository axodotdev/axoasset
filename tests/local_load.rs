use std::collections::HashMap;
use std::path::Path;

use assert_fs::prelude::*;

#[tokio::test]
async fn it_loads_local_assets() {
    let origin = assert_fs::TempDir::new().unwrap();

    let mut files = HashMap::new();
    files.insert("README.md", "# axoasset");
    files.insert("styles.css", "@import");

    for (file, contents) in files {
        let asset = origin.child(file);
        let content = Path::new("./tests/assets").join(file);
        asset.write_file(&content).unwrap();

        let origin_path = asset.to_str().unwrap();
        let loaded_asset = axoasset::Asset::load(origin_path).await.unwrap();

        if let axoasset::Asset::LocalAsset(asset) = loaded_asset {
            assert!(std::str::from_utf8(&asset.contents)
                .unwrap()
                .contains(contents));
        }
    }
}

#[tokio::test]
async fn it_loads_local_assets_as_bytes() {
    let origin = assert_fs::TempDir::new().unwrap();

    let mut files = HashMap::new();
    files.insert("README.md", "# axoasset");
    files.insert("styles.css", "@import");

    for (file, contents) in files {
        let asset = origin.child(file);
        let content = Path::new("./tests/assets").join(file);
        asset.write_file(&content).unwrap();

        let origin_path = asset.to_str().unwrap();
        let loaded_bytes = axoasset::Asset::load_bytes(origin_path).await.unwrap();

        assert!(std::str::from_utf8(&loaded_bytes)
            .unwrap()
            .contains(contents));
    }
}

#[tokio::test]
async fn it_loads_local_assets_as_strings() {
    let origin = assert_fs::TempDir::new().unwrap();

    let mut files = HashMap::new();
    files.insert("README.md", "# axoasset");
    files.insert("styles.css", "@import");

    for (file, contents) in files {
        let asset = origin.child(file);
        let content = Path::new("./tests/assets").join(file);
        asset.write_file(&content).unwrap();

        let origin_path = asset.to_str().unwrap();
        let loaded_string = axoasset::Asset::load_string(origin_path).await.unwrap();

        assert!(loaded_string.contains(contents))
    }
}
