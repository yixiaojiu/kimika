use kimika_grpc::local::local_server::Local;
use kimika_grpc::local::{EmptyRequest, EmptyResponse, FileRequest, MessageRequest};
use kimika_shared::type_utils::TonicRes;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio_stream::StreamExt;
use tonic::{Request, Response};

pub struct LocalService {
    tx: tokio::sync::mpsc::Sender<()>,
    save_folder: PathBuf,
}

impl LocalService {
    pub fn new(tx: tokio::sync::mpsc::Sender<()>, save_folder: &PathBuf) -> Self {
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
        let filename = file_metadata.get("filename").unwrap().to_str().unwrap();

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
        while let Some(request) = stream.next().await {
            let request = request?;
            let data = request.data;
            file.write(&data).await?;
        }
        Ok(Response::new(EmptyResponse {}))
    }
}