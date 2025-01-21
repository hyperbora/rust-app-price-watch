use rust_app_price_watch::{fetch_app_details, Parameters};
use tokio;

#[tokio::test]
async fn test_fetch_app_details() {
    let params = Parameters {
        app_id: "6502453075",
        country_code: "us",
    };

    let details = fetch_app_details(params).await;
    assert!(details.is_ok(), "API Error");
}
