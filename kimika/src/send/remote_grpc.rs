use super::remote::Content;
use crate::utils;
use kimika_grpc::remote::{self, remote_client::RemoteClient};
use std::{cmp::min, net::SocketAddr};
use tokio::{fs, io::AsyncReadExt, sync::mpsc};
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
    let (tx, rx) = mpsc::channel(5);

    if content.path.is_some() {
        let mut file = fs::File::open(content.path.as_ref().unwrap()).await?;
        let filename = content.name.as_ref().unwrap().clone();
        let total_size = content.size.unwrap();
        let progreebar = utils::handle::create_progress_bar(total_size, &filename);
        tokio::spawn(async move {
            let mut buf = vec![0u8; 2 * 1024 * 1024];
            let mut uploaded_size: u64 = 0;
            loop {
                let n = file.read(&mut buf).await.unwrap();
                if n == 0 {
                    break;
                }
                let left = uploaded_size;
                uploaded_size += n as u64;
                let req = remote::TransferContent {
                    data: buf[..n].to_vec(),
                    range: vec![left, uploaded_size],
                };
                tx.send(req).await.unwrap();
                progreebar.set_position(min(uploaded_size, total_size));
            }
            progreebar.finish_with_message(filename);
        });
    } else {
        let data = content.message.clone().unwrap().as_bytes().to_vec();
        let len = data.len();
        tx.send(remote::TransferContent {
            data,
            range: vec![0, len as u64],
        })
        .await?;
        drop(tx);
    }

    let mut request = tonic::Request::new(ReceiverStream::new(rx));
    request.metadata_mut().insert("id", content_id.parse()?);

    client.send(request).await?.into_inner();

    Ok(())
}
