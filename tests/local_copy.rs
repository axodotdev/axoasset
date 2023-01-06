use std::path::Path;

use assert_fs::prelude::*;

#[tokio::test]
async fn it_copies_local_assets() {
    let origin = assert_fs::TempDir::new().unwrap();
    let dest = assert_fs::TempDir::new().unwrap();
    let dest_dir = Path::new(dest.to_str().unwrap());

    let files = vec!["README.md", "logo.png", "styles.css"];

    for file in files {
        let asset = origin.child(file);
        let content = Path::new("./tests/assets").join(file);
        asset.write_file(&content).unwrap();

        let origin_path = asset.to_str().unwrap();
        axoasset::copy(origin_path, dest.to_str().unwrap())
            .await
            .unwrap();

        let copied_file = dest_dir.join(file);
        assert!(copied_file.exists());
    }
}
