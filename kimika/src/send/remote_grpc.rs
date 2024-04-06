use super::remote::Content;
use kimika_grpc::remote;
use kimika_grpc::remote::remote_client::RemoteClient;
use std::net::SocketAddr;
use tonic::transport::{Channel, Uri};

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
