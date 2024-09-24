use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Deserialize)]
pub struct Receiver {
    pub id: String,
    pub alias: String,
}

fn into_url(address: &SocketAddr) -> Url {
    let url_string = format!("http://{}", address);
    Url::parse(&url_string).expect("invalid address")
}

#[derive(Deserialize)]
pub struct GetReceiversRes {
    pub receivers: Vec<Receiver>,
    message: String,
}

pub async fn get_receivers(address: &SocketAddr) -> Result<GetReceiversRes, reqwest::Error> {
    let result = Client::new().get(into_url(address)).send().await?;
    Ok(result.json().await.unwrap())
}
