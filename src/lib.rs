use reqwest::{Error, Response};
use serde_json::Value;

pub struct Parameters {
    pub app_id: String,
    pub country_code: String,
}

pub async fn appstore_api(param: Parameters) -> Result<Value, Error> {
    let Parameters {
        app_id,
        country_code,
    } = param;
    let response: Response = reqwest::get(format!(
        "https://itunes.apple.com/search?term={}&country={}&entity=software",
        app_id, country_code
    ))
    .await?;

    let json = response.json().await?;
    Ok(json)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_appstore_api() {
        let params = Parameters {
            app_id: "id6502453075".to_string(),
            country_code: "kr".to_string(),
        };

        let result = appstore_api(params).await;
        assert!(result.is_ok());
    }
}
