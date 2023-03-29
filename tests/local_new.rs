#![allow(irrefutable_let_patterns)]

use std::collections::HashMap;
use std::path::Path;

#[tokio::test]
async fn it_creates_new_assets() {
    let dest = assert_fs::TempDir::new().unwrap();

    let mut files = HashMap::new();
    files.insert("README.md", "# axoasset");
    files.insert("styles.css", "@import");

    for (file, contents) in files {
        let origin_path = Path::new("./tests/assets").join(file).display().to_string();
        let dest_dir = Path::new(&dest.as_os_str())
            .join(file)
            .display()
            .to_string();
        axoasset::Asset::new(&origin_path, contents.into())
            .unwrap()
            .write(dest.to_str().unwrap())
            .await
            .unwrap();

        let loaded_asset = axoasset::Asset::load(&dest_dir).await.unwrap();

        if let axoasset::Asset::LocalAsset(asset) = loaded_asset {
            assert!(std::str::from_utf8(&asset.contents)
                .unwrap()
                .contains(contents));
        }
    }
}
