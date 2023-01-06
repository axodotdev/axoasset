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
        let loaded_asset = axoasset::load(origin_path).await.unwrap();

        if let axoasset::Asset::LocalAsset(asset) = loaded_asset {
            assert!(std::str::from_utf8(&asset.contents)
                .unwrap()
                .contains(contents));
        }
    }
}
