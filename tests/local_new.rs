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

#[test]
fn it_creates_parent_directories() {
    let dest = assert_fs::TempDir::new().unwrap();

    let dest_dir = Path::new(&dest.as_os_str())
        .join("subdir")
        .join("test.md")
        .display()
        .to_string();
    axoasset::LocalAsset::write_new_all("file content", "index.md", &dest_dir).unwrap();

    assert!(Path::new(&dest.as_os_str()).join("subdir").exists());
}

#[test]
fn it_creates_a_new_directory() {
    let dest = assert_fs::TempDir::new().unwrap();

    let dest_dir = Path::new(&dest.as_os_str())
        .join("subdir")
        .display()
        .to_string();
    axoasset::LocalAsset::create_dir(&dest_dir).unwrap();

    assert!(Path::new(&dest.as_os_str()).join("subdir").exists());
}
