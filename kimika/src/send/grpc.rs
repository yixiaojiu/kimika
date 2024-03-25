use crate::utils::color::{print_color, Color};
use kimika_grpc::local::{local_client::LocalClient, FileRequest, MessageRequest};
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::sync::mpsc;
use tonic::transport::Channel;

#[allow(unused_variables)]
pub async fn send_file(
    client: &mut LocalClient<Channel>,
    path: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(&path);
    println!("Sending file: {}", path.display());
    let mut file = File::open(path).await.expect("open file failed");
    let file_name = path.file_name().expect("").to_str().unwrap();
    let (tx, rx) = mpsc::channel(20);

    tokio::spawn(async move {
        let mut buf = [0; 1024 * 1024];
        loop {
            let n = file.read(&mut buf).await.unwrap();
            if n == 0 {
                break;
            }
            let req = FileRequest {
                data: buf[..n].to_vec(),
            };
            tx.send(req).await.unwrap();
            println!("Sent {} MB", n / (1024 * 1024));
        }
    });

    let mut request = tonic::Request::new(tokio_stream::wrappers::ReceiverStream::new(rx));
    request
        .metadata_mut()
        .insert("filename", file_name.parse()?);

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
    print_color("Message sent successfully", Color::Green)
}
