use serde::Deserialize;
pub struct Parameters<'a> {
    pub app_id: &'a str,
    pub country_code: &'a str,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AppStoreResponse {
    pub result_count: u32,
    pub results: Vec<AppDetails>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AppDetails {
    pub track_view_url: String,
    pub price: f64,
    pub formatted_price: String,
}

pub async fn fetch_app_details(
    param: Parameters<'_>,
) -> Result<AppStoreResponse, Box<dyn std::error::Error>> {
    let Parameters {
        app_id,
        country_code,
    } = param;
    let response = reqwest::get(format!(
        "https://itunes.apple.com/lookup?id={}&country={}",
        app_id, country_code
    ))
    .await
    .map_err(|_| "Request failed")?
    .text()
    .await
    .map_err(|_| "Failed to get text")?;

    let app_store_response: AppStoreResponse =
        serde_json::from_str(&response).expect("Json Parsing Error");
    Ok(app_store_response)
}
