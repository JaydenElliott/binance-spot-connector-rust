use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::anyhow;
use reqwest::{Client, Response};
use url::Url;

use crate::http::request::{self, Request};
use crate::http::Credentials;

const API_KEY_HEADER: &str = "X-MBX-APIKEY";

pub struct BinanceHttpClient {
    client: Client,
    base_url: Url,
    timestamp_delta: i64,
    credentials: Credentials,
}

impl BinanceHttpClient {
    pub fn new(base_url: &str, credentials: Credentials) -> anyhow::Result<Self> {
        Ok(Self {
            client: Client::new(),
            base_url: Url::parse(base_url)?,
            timestamp_delta: 0,
            credentials,
        })
    }

    pub async fn send<R: Into<Request>>(&mut self, request: R) -> anyhow::Result<Response> {
        // legacy
        let Request {
            method,
            path,
            params,
            sign,
            ..
        } = request.into();

        // todo: update rest of lib later
        let updated_method: reqwest::Method = match method {
            crate::http::Method::Get => reqwest::Method::GET,
            crate::http::Method::Post => reqwest::Method::POST,
            crate::http::Method::Put => reqwest::Method::PUT,
            crate::http::Method::Delete => reqwest::Method::DELETE,
        };

        let updated_url = {
            let mut url = self.base_url.to_owned();
            url.set_path(path.trim_start_matches('/'));
            url
        };
        let mut request_builder = self
            .client
            .request(updated_method, updated_url)
            .query(&params);

        if sign {
            //let mut headers = header::HeaderMap::new();
            //let name = header::HeaderName::from_str(API_KEY_HEADER)?;
            //let value = header::HeaderValue::from_str(&self.credentials.api_key)?;
            //headers.insert(name, value);

            request_builder =
                request_builder.header(API_KEY_HEADER, self.credentials.api_key.clone());
            let mut timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Clock may have gone backwards")
                .as_millis();
            timestamp -= self.timestamp_delta as u128;
            request_builder = request_builder.query(&[("timestamp", timestamp)]);

            // need to build to serialize query string
            let temp_builder = request_builder
                .try_clone()
                .ok_or_else(|| anyhow!("failed to clone request builder"))?;
            let temp_request = temp_builder.build()?;
            let query_string = temp_request
                .url()
                .query()
                .ok_or_else(|| anyhow!("no query string parameters on signed request"))?;
            let signature = crate::utils::sign(&query_string, &self.credentials.signature)?;
            request_builder = request_builder.query(&[("signature", signature)]);
        }

        let request = request_builder.build()?;
        self.client
            .execute(request)
            .await
            .map_err(|e| anyhow!("failed to send binance request: {:?}", e))
    }
}
