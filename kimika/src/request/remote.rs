use crate::send::utils::{Content, ContentType};
use crate::utils;
use reqwest::{Body, Client, Url};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::{
    fs,
    io::{self, AsyncReadExt},
    sync::mpsc,
};

#[derive(Deserialize)]
pub struct Receiver {
    pub id: String,
    pub alias: String,
}

#[derive(Serialize)]
pub struct Metadata {
    /// metadata unique id
    pub id: String,
    /// file or message
    pub metadata_type: String,
    pub file_name: Option<String>,
    pub file_type: Option<String>,
    pub size: Option<u64>,
}

/// server metadata structure
#[derive(Deserialize)]
pub struct MetadataItem {
    pub id: String,
    pub token: String,
    /// file or message
    pub metadata_type: String,
    pub file_name: Option<String>,
    pub file_type: Option<String>,
    pub size: Option<u64>,
}

/** ===================================== */

#[derive(Deserialize)]
pub struct GetReceiversResponse {
    pub receivers: Vec<Receiver>,
    pub message: String,
}

/** ===================================== */

#[derive(Serialize)]
pub struct PostMetadataPayload {
    pub receiver_id: String,
    pub alias: String,
    pub metadata: Vec<Metadata>,
}

#[derive(Deserialize)]
pub struct PostMetadataResponseMetadata {
    pub id: String,
    pub token: String,
}

#[derive(Deserialize)]
pub struct PostMetadataResponse {
    pub selected_metadata_list: Vec<PostMetadataResponseMetadata>,
    /// sender id
    pub id: String,
    pub message: String,
}

/** ===================================== */

#[derive(Serialize)]
pub struct PostUploadParams {
    pub token: String,
    /// sender id
    pub id: String,
    /// receiver id
    pub receiver: String,
}

/** ===================================== */

#[derive(Serialize)]
pub struct PostRegisterPayload {
    alias: String,
}

#[derive(Deserialize)]
pub struct PostRegisterResponse {
    pub id: String,
    pub message: String,
}

/** ===================================== */

#[derive(Serialize)]
pub struct GetMetadataParams {
    /// receiver id
    pub id: String,
}

#[derive(Deserialize)]
pub struct GetMetadataResponse {
    pub metadatas: Vec<MetadataItem>,
    pub message: String,
}

/** ===================================== */

#[derive(Serialize)]
pub struct PostSelectMetadataPayload {
    /// receiver id
    pub id: String,
    /// metadata unique token
    pub selected_tokens: Vec<String>,
}

#[derive(Deserialize)]
pub struct PostSelectMetadataResponse {
    pub message: String,
}

/** ===================================== */

pub struct RequestClient {
    url: Url,
}

impl RequestClient {
    pub fn new(address: &SocketAddr) -> Self {
        let url_string = format!("http://{}", address);
        let url = Url::parse(&url_string).expect("invalid address");
        Self { url }
    }

    pub async fn get_receivers(&self) -> Result<GetReceiversResponse, reqwest::Error> {
        let mut url = self.url.clone();
        url.set_path("/receivers");

        let result = Client::new().get(url).send().await?;
        Ok(result.json().await.unwrap())
    }

    pub async fn post_metadata(
        &self,
        payload: &PostMetadataPayload,
    ) -> Result<PostMetadataResponse, reqwest::Error> {
        let mut url = self.url.clone();
        url.set_path("/metadata");

        let result = Client::new().post(url).json(payload).send().await?;
        Ok(result.json().await.unwrap())
    }

    pub async fn post_upload(
        &self,
        content: &Content,
        payload: PostUploadParams,
    ) -> Result<(), reqwest::Error> {
        let mut url = self.url.clone();
        url.set_path("/upload");

        let request_builder = Client::new().post(url).query(&payload);

        match content.content_type {
            ContentType::File => {
                let path = content.path.as_ref().unwrap().clone();
                let filename = path
                    .file_name()
                    .expect("invalid file name")
                    .to_str()
                    .unwrap()
                    .to_string();
                let file = fs::File::open(path).await.expect("open file failed");
                let metadata = file.metadata().await.expect("get metadata failed");
                let total_size = metadata.len();
                let mut reader = io::BufReader::new(file);
                let (tx, rx) = mpsc::channel::<Result<Vec<u8>, reqwest::Error>>(5);

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
                    .send()
                    .await?;
            }
            ContentType::Message => {
                let message = content.message.as_ref().unwrap().clone();
                request_builder.body(message).send().await?;
            }
        }
        Ok(())
    }

    pub async fn post_register(
        &self,
        alias: String,
    ) -> Result<PostRegisterResponse, reqwest::Error> {
        let mut url = self.url.clone();
        url.set_path("/register");

        let result = Client::new()
            .post(url)
            .json(&PostRegisterPayload { alias })
            .send()
            .await?;
        Ok(result.json().await.unwrap())
    }

    pub async fn get_metadata(
        &self,
        receiver_id: String,
    ) -> Result<GetMetadataResponse, reqwest::Error> {
        let mut url = self.url.clone();
        url.set_path("/metadata");
        url.set_query(Some(
            serde_qs::to_string(&GetMetadataParams { id: receiver_id })
                .unwrap()
                .as_str(),
        ));
        let result = Client::new().get(url).send().await?;
        Ok(result.json().await.unwrap())
    }

    pub async fn post_select_metadata(
        &self,
        payload: &PostSelectMetadataPayload,
    ) -> Result<PostSelectMetadataResponse, reqwest::Error> {
        let mut url = self.url.clone();
        url.set_path("/metadata/select");
        let result = Client::new().post(url).json(payload).send().await?;
        Ok(result.json().await.unwrap())
    }
}
