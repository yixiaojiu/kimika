use super::remote::Content;
use kimika_grpc::remote;
use kimika_grpc::remote::remote_client::RemoteClient;
use std::net::SocketAddr;
use tokio::io::AsyncReadExt;
use tokio::{fs, sync::mpsc};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{
    self,
    transport::{Channel, Uri},
    Streaming,
};

pub async fn create_client(
    address: SocketAddr,
) -> Result<RemoteClient<Channel>, Box<dyn std::error::Error>> {
    let url = format!("http://{}", address).parse::<Uri>()?;
    Ok(RemoteClient::connect(url).await?)
}

pub async fn register_content(
    client: &mut RemoteClient<Channel>,
    content: &Content,
) -> Result<remote::RegisterContentResponse, Box<dyn std::error::Error>> {
    let request = if content.path.is_some() {
        remote::RegisterContentRequest {
            content_type: remote::Type::File as i32,
            alias: "sender".to_string(),
            size: content.size,
            name: content.name.clone(),
        }
    } else {
        remote::RegisterContentRequest {
            content_type: remote::Type::Message as i32,
            alias: "sender".to_string(),
            size: None,
            name: None,
        }
    };

    Ok(client.register_content(request).await?.into_inner())
}

pub async fn get_receivers(
    client: &mut RemoteClient<Channel>,
) -> Result<Streaming<remote::GetReceiversResponse>, Box<dyn std::error::Error>> {
    Ok(client
        .get_receivers(remote::EmptyRequest {})
        .await?
        .into_inner())
}

pub async fn choose_receiver(
    client: &mut RemoteClient<Channel>,
    receiver_id: String,
    sender_id: String,
) -> Result<Streaming<remote::ChooseReceiverResponse>, Box<dyn std::error::Error>> {
    Ok(client
        .choose_receiver(remote::ChooseReceiverRequest {
            receiver_id: receiver_id,
            sender_id: sender_id,
        })
        .await?
        .into_inner())
}

pub async fn send(
    client: &mut RemoteClient<Channel>,
    content_id: String,
    content: &Content,
) -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = mpsc::channel(10);

    if content.path.is_some() {
        let mut file = fs::File::open(content.path.as_ref().unwrap()).await?;
        tokio::spawn(async move {
            let mut buf = [0; 1024 * 1024];
            let index: u64 = 0;
            loop {
                let n = file.read(&mut buf).await.unwrap();
                if n == 0 {
                    break;
                }
                let req = remote::TransferContent {
                    data: buf[..n].to_vec(),
                    range: vec![index, index + n as u64],
                };
                tx.send(req).await.unwrap();
                println!("Sent {} MB", n / (1024 * 1024));
            }
        });
    } else {
        let data = content.message.clone().unwrap().as_bytes().to_vec();
        let len = data.len();
        tx.send(remote::TransferContent {
            data,
            range: vec![0, len as u64],
        })
        .await?;
    }

    let mut request = tonic::Request::new(ReceiverStream::new(rx));
    request.metadata_mut().insert("id", content_id.parse()?);

    client.send(request).await?.into_inner();

    Ok(())
}
