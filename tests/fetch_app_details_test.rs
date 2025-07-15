use rust_app_price_watch::StoreType;

#[test]
fn test_store_type_from_url() {
    let url = "https://apps.apple.com/kr/app/balatro/id6502453075";
    let store = StoreType::from_url(url);
    match store {
        Some(StoreType::AppStore(param)) => {
            assert_eq!(param.country_code, "kr");
            assert_eq!(param.app_id, "6502453075");
        }
        _ => panic!("AppStore URL을 올바르게 파싱하지 못함"),
    }

    let url = "https://store.nintendo.co.kr/70010000096811";
    let store = StoreType::from_url(url);
    match store {
        Some(StoreType::Nintendo(param)) => {
            assert_eq!(param.country_code, "kr");
            assert_eq!(param.app_id, "70010000096811");
        }
        _ => panic!("Nintendo URL을 올바르게 파싱하지 못함"),
    }

    let url = "https://invalid.url.com/app/balatro/id6502453075";
    let store = StoreType::from_url(url);
    assert!(store.is_none());
}

#[tokio::test]
async fn test_fetch_app_detail() {
    let url = "https://apps.apple.com/kr/app/balatro/id6502453075";
    let store = StoreType::from_url(url).unwrap();
    let detail = store.fetch_app_detail().await;
    assert!(detail.is_ok());
    let app_detail = detail.unwrap();
    assert!(app_detail.price >= 0.0);
}