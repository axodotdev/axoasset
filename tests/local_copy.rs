use std::collections::HashMap;
use std::path::Path;

use assert_fs::prelude::*;

#[tokio::test]
async fn it_copies_local_assets() {
    let origin = assert_fs::TempDir::new().unwrap();
    let dest = assert_fs::TempDir::new().unwrap();
    let dest_dir = Path::new(dest.to_str().unwrap());

    let mut files = HashMap::new();
    files.insert("README.md", "# axoasset");
    files.insert("styles.css", "@import");

    for (file, contents) in files {
        let asset = origin.child(file);
        let content = Path::new("./tests/assets").join(file);
        asset.write_file(&content).unwrap();

        let origin_path = asset.to_str().unwrap();
        axoasset::Asset::copy(origin_path, dest.to_str().unwrap())
            .await
            .unwrap();

        let copied_file = dest_dir.join(file);
        assert!(copied_file.exists());
        let loaded_asset = axoasset::Asset::load(copied_file.to_str().unwrap())
            .await
            .unwrap();
        if let axoasset::Asset::LocalAsset(asset) = loaded_asset {
            assert!(std::str::from_utf8(&asset.contents)
                .unwrap()
                .contains(contents));
        }
    }
}
