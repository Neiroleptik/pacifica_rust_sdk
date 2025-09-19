use reqwest::{Client, header::HeaderMap};
use serde::{
    Serialize,
    de::DeserializeOwned,
};
use tracing::debug;

use crate::common::errors::ExchangeError;

pub struct RestClient {
    client: Client,
    base_url: String,
}

type Result<T> = std::result::Result<T, ExchangeError>;

impl RestClient {
    pub fn new(base_url: &'static str) -> Self {
        Self {
            client: Client::builder().connection_verbose(true).build().unwrap(),
            base_url: base_url.to_string(),
        }
    }

    pub async fn get<T, P>(
        &self,
        endpoint: Option<&str>,
        params: Option<&P>,
        headers: Option<&HeaderMap>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        P: Serialize,
    {
        let endpoint = endpoint.unwrap_or("");
        let mut req = self.client.get(format!("{}{}", self.base_url, endpoint));

        if let Some(p) = params {
            if cfg!(debug_assertions) {
                match serde_json::to_string(p) {
                    Ok(json) => debug!("GET {} params: {}", endpoint, json),
                    Err(_) => debug!("GET {} params: <failed to serialize>", endpoint),
                }
            }
            req = req.query(p);
        }

        if let Some(h) = headers {
            req = req.headers(h.clone());
        }

        let resp = req.send().await?;
        debug!("GET {} response: {:?}", endpoint, &resp);
        let resp_text = resp.text().await?;
        let response: T = serde_json::from_str(&resp_text)?;
        Ok(response)
    }

    pub async fn post<T, P>(
        &self,
        endpoint: Option<&str>,
        body: Option<&P>,
        headers: Option<&HeaderMap>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        P: Serialize,
    {
        let endpoint = endpoint.unwrap_or("");
        let mut req = self.client.post(format!("{}{}", self.base_url, endpoint));

        if let Some(b) = body {
            if cfg!(debug_assertions) {
                match serde_json::to_string(b) {
                    Ok(json) => debug!("POST {} body: {}", endpoint, json),
                    Err(_) => debug!("POST {} body: <failed to serialize>", endpoint),
                }
            }

            req = req.json(b);
        }

        if let Some(h) = headers {
            req = req.headers(h.clone());
        }

        let resp = req.send().await?;
        let resp_text = resp.text().await?;
        debug!("POST {} response: {:?}", endpoint, &resp_text);
        let response: T = serde_json::from_str(&resp_text)?;
        Ok(response)
    }
}
