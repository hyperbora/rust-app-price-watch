use reqwest::Client;
use rust_app_price_watch::{fetch_app_detail, Parameters};
use tokio;

#[tokio::test]
async fn test_fetch_app_detail() {
    let params = Parameters {
        app_id: "6502453075",
        country_code: "us",
    };

    let client = Client::new();
    let detail = fetch_app_detail(&client, params).await;
    assert!(detail.is_ok(), "API Error");
}
