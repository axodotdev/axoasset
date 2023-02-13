use std::collections::HashMap;
use std::fs;

use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn it_writes_remote_assets() {
    let mock_server = MockServer::start().await;

    let dest = assert_fs::TempDir::new().unwrap();

    let mut assets = HashMap::new();
    assets.insert("/README.md", "# axoasset");
    assets.insert("/README", "# axoasset");
    assets.insert("/styles.css", "@import");
    assets.insert("/styles", "@import");

    for (route, contents) in assets {
        let response = if route.contains("README") {
            let readme_bytes = fs::read("./tests/assets/README.md").unwrap();
            ResponseTemplate::new(200)
                .set_body_bytes(readme_bytes)
                .insert_header("Content-Type", "text/plain+md")
        } else {
            let styles_bytes = fs::read("./tests/assets/styles.css").unwrap();
            ResponseTemplate::new(200)
                .set_body_bytes(styles_bytes)
                .insert_header("Content-Type", "text/css")
        };

        Mock::given(method("GET"))
            .and(path(route))
            .respond_with(response)
            .mount(&mock_server)
            .await;

        let mut origin_path = format!("http://{}", mock_server.address());
        origin_path.push_str(route);
        let loaded_asset = axoasset::Asset::load(&origin_path).await.unwrap();

        if let axoasset::Asset::RemoteAsset(asset) = loaded_asset {
            let dest = asset.write(dest.to_str().unwrap()).await.unwrap();
            assert!(dest.exists());
            fs::read_to_string(dest).unwrap().contains(contents);
        }
    }
}
