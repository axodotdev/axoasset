use std::collections::HashMap;
use std::fs;

use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn it_loads_remote_assets() {
    let mock_server = MockServer::start().await;

    let mut assets = HashMap::new();
    assets.insert("/README.md", "# axoasset");
    assets.insert("/README", "# axoasset");
    assets.insert("/styles.css", "@import");
    assets.insert("/styles", "@import");

    let readme_string = fs::read_to_string("./tests/assets/README.md")
        .expect("failed to read ./tests/assets/README.md");
    let styles_string = fs::read_to_string("./tests/assets/styles.css")
        .expect("failed to read ./tests/assets/styles.css");

    for (route, contents) in assets {
        let resp_string = if route.contains("README") {
            &readme_string
        } else {
            &styles_string
        };

        Mock::given(method("GET"))
            .and(path(route))
            .respond_with(ResponseTemplate::new(200).set_body_string(resp_string))
            .mount(&mock_server)
            .await;

        let mut origin_path = mock_server.address().to_string();
        origin_path.push_str(route);
        let loaded_asset = axoasset::load(&origin_path).await.unwrap();

        if let axoasset::Asset::RemoteAsset(asset) = loaded_asset {
            assert!(std::str::from_utf8(&asset.contents)
                .unwrap()
                .contains(contents));
        }
    }
}
