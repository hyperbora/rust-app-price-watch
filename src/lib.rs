use regex::Regex;
use reqwest::Client;
use serde::Deserialize;

pub enum StoreType {
    AppStore(Parameter),
    Nintendo(Parameter),
}

impl StoreType {
    fn pattern(&self) -> &'static str {
        match self {
            StoreType::AppStore(_) => {
                r"https://apps\.apple\.com/(?P<country_code>\w{2})/app/.+/id(?P<app_id>\d+)"
            }
            StoreType::Nintendo(_) => {
                r"https://store\.nintendo\.co\.(?P<country_code>\w{2})/(?P<app_id>\d+)"
            }
        }
    }

    pub fn from_url(url: &str) -> Option<Self> {
        let types = [
            StoreType::AppStore(Parameter {
                app_id: String::new(),
                country_code: String::new(),
            }),
            StoreType::Nintendo(Parameter {
                app_id: String::new(),
                country_code: String::new(),
            }),
        ];
        for store in types.iter() {
            let re = Regex::new(store.pattern()).ok()?;
            if let Some(caps) = re.captures(url) {
                let country_code = caps.name("country_code")?.as_str().to_string();
                let app_id = caps.name("app_id")?.as_str().to_string();
                return match store {
                    StoreType::AppStore(_) => Some(StoreType::AppStore(Parameter { app_id, country_code })),
                    StoreType::Nintendo(_) => Some(StoreType::Nintendo(Parameter { app_id, country_code })),
                };
            }
        }
        None
    }
}

pub struct Parameter {
    pub app_id: String,
    pub country_code: String,
}

impl Parameter {
    pub fn from_url(url: &str) -> Result<Self, String> {
        let re = Regex::new(
            r"https://apps\\.apple\\.com/(?P<country_code>\\w{2})/app/.+/id(?P<app_id>\\d+)",
        )
        .map_err(|_| "Invalid regex".to_string())?;
        let caps = re
            .captures(url)
            .ok_or("URL does not match the expected format".to_string())?;

        let country_code = caps
            .name("country_code")
            .ok_or("Country code not found".to_string())?
            .as_str()
            .to_string();
        let app_id = caps
            .name("app_id")
            .ok_or("App ID not found".to_string())?
            .as_str()
            .to_string();

        Ok(Parameter {
            app_id,
            country_code,
        })
    }
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
    param: Parameter,
) -> Result<AppDetail, Box<dyn std::error::Error>> {
    let Parameter {
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
