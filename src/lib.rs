use regex::Regex;
use reqwest::Client;
use serde::Deserialize;

pub enum StoreType {
    AppStore(Parameter),
    Nintendo(Parameter),
}

type AppDetailError = Box<dyn std::error::Error>;

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
                    StoreType::AppStore(_) => Some(StoreType::AppStore(Parameter {
                        app_id,
                        country_code,
                    })),
                    StoreType::Nintendo(_) => Some(StoreType::Nintendo(Parameter {
                        app_id,
                        country_code,
                    })),
                };
            }
        }
        None
    }

    pub async fn fetch_app_detail(&self) -> Result<AppDetail, AppDetailError> {
        let client = Client::new();
        match self {
            StoreType::AppStore(param) => {
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
                    let detail = app_store_response.results.into_iter().next().unwrap();
                    Ok(AppDetail { price: detail.price })
                } else {
                    Err("No results found".into())
                }
            }
            StoreType::Nintendo(param) => {
                let Parameter {
                    app_id,
                    country_code,
                } = param;
                let url = format!(
                    "https://api.ec.nintendo.com/v1/price?country={}&ids={}&lang=en",
                    country_code.to_uppercase(), app_id
                );
                let response = client
                    .get(&url)
                    .send()
                    .await
                    .map_err(|_| "Request failed")?;
                let nintendo_response = response.json::<NintendoResponse>().await.map_err(|_| "Json parsing error")?;
                if nintendo_response.prices.is_empty() {
                    return Err("No prices found".into());
                }
                let price = nintendo_response.prices.first().unwrap().regular_price.raw_value.parse::<f64>()
                    .map_err(|_| "Failed to parse price")?;
                Ok(AppDetail { price })
            }
        }
    }
}

pub struct Parameter {
    pub app_id: String,
    pub country_code: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct AppStoreResponse {
    pub result_count: u32,
    pub results: Vec<AppStoreResponseDetail>,
}

#[derive(Deserialize, Debug)]
struct AppStoreResponseDetail {
    pub price: f64
}

#[derive(Deserialize, Debug)]
struct NintendoResponse {
    pub prices: Vec<NintendoPrice>,
}

#[derive(Deserialize, Debug)]
struct NintendoPrice {
    pub regular_price: NintendoPriceDetail,
}

#[derive(Deserialize, Debug)]
struct NintendoPriceDetail {
    pub raw_value: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AppDetail {
    pub price: f64
}
