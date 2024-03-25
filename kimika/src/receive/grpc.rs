use std::path::Path;

use crate::utils::utils_type::TonicRes;
use kimika_grpc::local::local_server::Local;
use kimika_grpc::local::{EmptyRequest, EmptyResponse, FileRequest, MessageRequest};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio_stream::StreamExt;
use tonic::{Request, Response};

#[derive(Debug)]
pub struct LocalService {
    tx: tokio::sync::mpsc::Sender<()>,
    save_folder: Option<String>,
}

impl LocalService {
    pub fn new(tx: tokio::sync::mpsc::Sender<()>, save_folder: String) -> Self {
        let path = if save_folder == "" {
            None
        } else {
            Some(save_folder)
        };
        LocalService {
            tx,
            save_folder: path,
        }
    }

    async fn shutdown(&self) {
        self.tx.send(()).await.unwrap();
    }
}

#[tonic::async_trait]
impl Local for LocalService {
    async fn send_message(&self, request: Request<MessageRequest>) -> TonicRes<EmptyResponse> {
        let message_request = request.into_inner();
        print!("{}", message_request.message);
        Ok(Response::new(EmptyResponse {}))
    }

    async fn close(&self, _: Request<EmptyRequest>) -> TonicRes<EmptyResponse> {
        self.shutdown().await;
        Ok(Response::new(EmptyResponse {}))
    }

    async fn send_file(
        &self,
        request: Request<tonic::Streaming<FileRequest>>,
    ) -> TonicRes<EmptyResponse> {
        let file_metadata = request.metadata();
        let filename = file_metadata.get("filename").unwrap().to_str().unwrap();

        let mut path = if let Some(save_folder) = &self.save_folder {
            Path::new(save_folder).join(filename)
        } else {
            Path::new(filename).to_path_buf()
        };

        let mut rename_num = 1;
        loop {
            if !path.exists() {
                break;
            }
            path.set_file_name(format!("{}({})", filename, rename_num));
            rename_num += 1;
        }

        let mut file = File::create(path).await?;

        let mut stream = request.into_inner();
        while let Some(request) = stream.next().await {
            let request = request?;
            let data = request.data;
            file.write(&data[..]).await?;
        }
        Ok(Response::new(EmptyResponse {}))
    }
}
