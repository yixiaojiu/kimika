use crate::server::sender;
use reqwest::{Client, Url};
use std::net::SocketAddr;

pub struct RequestClient {
    url: Url,
}

impl RequestClient {
    pub fn new(address: &SocketAddr) -> Self {
        let url_string = format!("http://{}", address);
        let url = Url::parse(&url_string).expect("invalid address");
        Self { url }
    }
}

impl RequestClient {
    pub async fn register(&self, alias: String, port: u16) -> Result<String, reqwest::Error> {
        let mut url = self.url.clone();
        url.set_path("/register");

        let result = Client::new()
            .post(url)
            .json(&sender::Payload { alias, port })
            .send()
            .await?;

        let bytes = result.bytes().await?;

        Ok(String::from_utf8_lossy(&bytes).to_string())
    }
}
