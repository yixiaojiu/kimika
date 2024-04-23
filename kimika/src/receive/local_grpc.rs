use crate::utils;
use kimika_grpc::local::local_server::Local;
use kimika_grpc::local::{EmptyRequest, EmptyResponse, FileRequest, MessageRequest};
use kimika_shared::type_utils::TonicRes;
use std::{cmp::min, path};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio_stream::StreamExt;
use tonic::{Request, Response};

pub struct LocalService {
    tx: tokio::sync::mpsc::Sender<()>,
    save_folder: path::PathBuf,
}

impl LocalService {
    pub fn new(tx: tokio::sync::mpsc::Sender<()>, save_folder: &path::PathBuf) -> Self {
        LocalService {
            tx,
            save_folder: save_folder.clone(),
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
        let filename = file_metadata
            .get("filename")
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let total_size: u64 = file_metadata
            .get("size")
            .unwrap()
            .to_str()
            .unwrap()
            .parse()
            .unwrap();

        let mut path = self.save_folder.clone();
        path.push(&filename);

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
        let progreebar = utils::handle::create_progress_bar(total_size, &filename);
        let mut downloaded_size: u64 = 0;
        while let Some(request) = stream.next().await {
            let request = request?;
            let data = request.data;
            file.write(&data).await?;
            downloaded_size += data.len() as u64;
            progreebar.set_position(min(downloaded_size, total_size));
        }
        progreebar.finish_with_message(filename);
        Ok(Response::new(EmptyResponse {}))
    }
}
