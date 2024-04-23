use crate::utils;
use kimika_grpc::local::{local_client::LocalClient, FileRequest, MessageRequest};
use std::time::Duration;
use std::{cmp::min, path};
use tokio::io::AsyncReadExt;
use tokio::{fs, sync::mpsc, time};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Channel, Request};

#[allow(unused_variables)]
pub async fn send_file(
    client: &mut LocalClient<Channel>,
    path: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let pathbuf = path::PathBuf::from(path);
    let mut file = fs::File::open(&pathbuf).await.expect("open file failed");
    let filename = pathbuf.file_name().expect("").to_str().unwrap().to_string();
    let total_size = fs::metadata(&pathbuf).await?.len();
    let (tx, rx) = mpsc::channel(10);

    let progreebar = utils::handle::create_progress_bar(total_size, &filename);
    let filename_clone = filename.clone();
    tokio::spawn(async move {
        let mut buf = [0; 1024 * 1024];
        let mut uploaded_size: u64 = 0;
        loop {
            let n = file.read(&mut buf).await.unwrap();
            if n == 0 {
                break;
            }
            let left = uploaded_size;
            uploaded_size += n as u64;
            let req = FileRequest {
                data: buf[..n].to_vec(),
            };
            tx.send(req).await.unwrap();
            if cfg!(debug_assertions) {
                time::sleep(Duration::from_millis(100)).await;
            }
            progreebar.set_position(min(uploaded_size, total_size));
        }
        progreebar.finish_with_message(filename_clone);
    });

    let mut request = Request::new(ReceiverStream::new(rx));
    request.metadata_mut().insert("filename", filename.parse()?);
    request.metadata_mut().insert("size", total_size.into());

    client
        .send_file(request)
        .await
        .expect("File sending failed");
    Ok(())
}

pub async fn send_message(client: &mut LocalClient<Channel>, message: String) {
    client
        .send_message(MessageRequest { message })
        .await
        .expect("Message sending failed");
}
