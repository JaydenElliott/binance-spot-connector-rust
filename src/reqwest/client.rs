use reqwest::{Client, IntoUrl, Request, RequestBuilder, Url};

use crate::http::Credentials;

#[derive(Clone)]
pub struct BinanceHttpClient {
    //client: Client<T, Body>,
    client: Client,
    base_url: Url,
    timestamp_delta: i64,
    credentials: Option<Credentials>,
}

impl BinanceHttpClient {
    pub fn new<T: IntoUrl>(base_url: T) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.into_url().unwrap(),
            timestamp_delta: 0,
            credentials: None,
        }
    }

    pub async fn send(&self, request: Request) -> () {
        //let req = RequestBuilder::
        let url = self.base_url.join(&request.url().to_string()).unwrap();
        let request_builder = self.client.request(request.method().to_owned(), url);

        request_builder.query(request)
    }
}
