use reqwest::Client;
use serde::Deserialize;
pub struct Parameters<'a> {
    pub app_id: &'a str,
    pub country_code: &'a str,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AppStoreResponse {
    pub result_count: u32,
    pub results: Vec<AppDetail>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AppDetail {
    pub track_view_url: String,
    pub price: Option<f64>,
    pub formatted_price: Option<String>,
}

pub async fn fetch_app_detail(
    client: &Client,
    param: Parameters<'_>,
) -> Result<AppDetail, Box<dyn std::error::Error>> {
    let Parameters {
        app_id,
        country_code,
    } = param;
    let url = format!(
        "https://itunes.apple.com/lookup?id={}&country={}",
        app_id, country_code
    );
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|_| "Request failed")?;
    let app_store_response = response
        .json::<AppStoreResponse>()
        .await
        .map_err(|_| "Json parsing error")?;
    if app_store_response.result_count == 1 {
        Ok(app_store_response.results.into_iter().next().unwrap())
    } else {
        Err("No results found".into())
    }
}
