use crate::server::{receiver, sender};
use crate::{utils, CONFIG};

use reqwest::{Body, Client, Url};
use std::net::SocketAddr;
use tokio::{
    fs,
    io::{self, AsyncReadExt},
    sync::mpsc,
};

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

    pub async fn post_metadata(
        &self,
        metadata_list: Vec<receiver::PayloadMetadataItem>,
    ) -> Result<receiver::PostMetadataResponse, Box<dyn std::error::Error>> {
        let mut url = self.url.clone();
        url.set_path("/metadata");

        let result = Client::new()
            .post(url)
            .json(&receiver::PostRegisterPayload {
                alias: CONFIG.alias.clone(),
                metadata_list,
            })
            .send()
            .await?;
        let bytes = result.bytes().await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    pub async fn post_upload(
        &self,
        content: &utils::Content,
        token: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut url = self.url.clone();
        url.set_path("/upload");

        let request_builder = Client::new()
            .post(url)
            .query(&receiver::PostUploadParams { token });

        match content.content_type {
            utils::ContentType::File => {
                let path = content.path.as_ref().unwrap().clone();
                let filename = path
                    .file_name()
                    .expect("invalid file name")
                    .to_str()
                    .unwrap()
                    .to_string();
                let file = fs::File::open(path).await?;
                let metadata = file.metadata().await?;
                let total_size = metadata.len();
                let mut reader = io::BufReader::with_capacity(1024 * 1024, file);
                let (tx, rx) = mpsc::channel::<Result<Vec<u8>, reqwest::Error>>(3);

                let progreebar = utils::handle::create_progress_bar(total_size, &filename);

                tokio::spawn(async move {
                    let mut buffer = vec![0u8; 512 * 1024];
                    let mut uploaded_size: u64 = 0;

                    loop {
                        let size = reader.read(&mut buffer).await.unwrap();
                        if size == 0 {
                            tx.closed().await;
                            break;
                        }
                        uploaded_size += size as u64;

                        if tx.send(Ok(buffer[0..size].to_vec())).await.is_err() {
                            tx.closed().await;
                            break;
                        }
                        progreebar.set_position(std::cmp::min(uploaded_size, total_size));
                    }
                    progreebar.finish_with_message(filename);
                });

                request_builder
                    .body(Body::wrap_stream(
                        tokio_stream::wrappers::ReceiverStream::new(rx),
                    ))
                    .header("Content-Length", total_size)
                    .send()
                    .await?;
            }
            utils::ContentType::Message => {
                let message = content.message.as_ref().unwrap().clone();
                request_builder.body(message).send().await?;
            }
        }
        Ok(())
    }
}
