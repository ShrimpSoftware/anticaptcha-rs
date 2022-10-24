use crate::errors::Error;
use crate::models::{BalanceResponse, ErrorResponse};

use serde::de::DeserializeOwned;
use std::collections::HashMap;

pub struct Anticaptcha {
    pub client: reqwest::Client,
    pub key: String,
}

impl Anticaptcha {
    pub fn new(key: String) -> Self {
        let client = reqwest::Client::new();
        Anticaptcha {
            client: client,
            key: key,
        }
    }

    pub async fn balance(&self) -> Result<BalanceResponse, Error> {
        let data = HashMap::from([("clientKey", self.key.as_str())]);
        let response: BalanceResponse =
            request(&self.client, String::from("/getBalance"), &data).await?;
        Ok(response)
    }
}

pub async fn request<T: DeserializeOwned>(
    client: &reqwest::Client,
    path: String,
    data: &HashMap<&str, &str>,
) -> Result<T, Error> {
    let response = client
        .post(format!("https://api.anti-captcha.com{}", path))
        .json(data)
        .send()
        .await?
        .text()
        .await?;

    if response.contains("errorCode") {
        let result: ErrorResponse = serde_json::from_str(&response)?;
        return Err(Error::ApiError(result));
    }

    let result: T = serde_json::from_str(&response)?;
    Ok(result)
}
