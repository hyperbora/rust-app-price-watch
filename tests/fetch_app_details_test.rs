use reqwest::Client;
use rust_app_price_watch::StoreType;
use rust_app_price_watch::{fetch_app_detail, Parameter};
use tokio;

#[tokio::test]
async fn test_fetch_app_detail() {
    let url = "https://apps.apple.com/us/app/balatro/id6502453075";
    let params = Parameter::from_url(url).expect("Failed to parse URL");

    let client = Client::new();
    let detail = fetch_app_detail(&client, params).await;
    assert!(detail.is_ok(), "{}", detail.unwrap_err().to_string());
}

#[test]
fn test_parameter_from_url() {
    let url = "https://apps.apple.com/us/app/balatro/id6502453075";
    let params = Parameter::from_url(url).expect("Failed to parse URL");
    assert_eq!(params.country_code, "us");
    assert_eq!(params.app_id, "6502453075");
}

#[test]
fn test_parameter_from_invalid_url() {
    let url = "https://invalid.url.com/app/balatro/id6502453075";
    let result = Parameter::from_url(url);
    assert!(result.is_err());
}

#[test]
fn test_store_type_from_url() {
    // AppStore 케이스
    let url = "https://apps.apple.com/kr/app/balatro/id6502453075";
    let store = StoreType::from_url(url);
    match store {
        Some(StoreType::AppStore(param)) => {
            assert_eq!(param.country_code, "kr");
            assert_eq!(param.app_id, "6502453075");
        }
        _ => panic!("AppStore URL을 올바르게 파싱하지 못함"),
    }

    // Nintendo 케이스
    let url = "https://store.nintendo.co.kr/70010000096811";
    let store = StoreType::from_url(url);
    match store {
        Some(StoreType::Nintendo(param)) => {
            assert_eq!(param.country_code, "kr");
            assert_eq!(param.app_id, "70010000096811");
        }
        _ => panic!("Nintendo URL을 올바르게 파싱하지 못함"),
    }

    // 잘못된 URL
    let url = "https://invalid.url.com/app/balatro/id6502453075";
    let store = StoreType::from_url(url);
    assert!(store.is_none());
}
