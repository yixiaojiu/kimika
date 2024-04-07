use kimika_grpc::remote::{self, remote_client::RemoteClient};
use std::{net::SocketAddr, path::PathBuf};
use tokio::fs;
use tokio::io::AsyncWriteExt;
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

pub async fn register_receiver(
    client: &mut RemoteClient<Channel>,
    alias: &String,
) -> Result<remote::RegisterReceiverResponse, Box<dyn std::error::Error>> {
    Ok(client
        .register_receiver(remote::RegisterReceiverRequest {
            alias: alias.clone(),
        })
        .await?
        .into_inner())
}

pub async fn get_content(
    client: &mut RemoteClient<Channel>,
    receiver_id: &String,
) -> Result<Streaming<remote::GetContentResponse>, Box<dyn std::error::Error>> {
    Ok(client
        .get_content(remote::GetContentRequest {
            receiver_id: receiver_id.clone(),
        })
        .await?
        .into_inner())
}

pub async fn receive(
    client: &mut RemoteClient<Channel>,
    receiver_id: &String,
    save_folder: &PathBuf,
    content: remote::get_content_response::Content,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut receive_res = client
        .receive(remote::ReceiveRequest {
            receiver_id: receiver_id.clone(),
            content_id: content.content_id.clone(),
        })
        .await?
        .into_inner();

    if content.content_type == remote::Type::Message as i32 {
        while let Some(res) = receive_res.message().await? {
            print!("{}", String::from_utf8_lossy(&res.data));
        }
    } else {
        let mut pathbuf = save_folder.clone();
        let filename = content.name.unwrap().clone();
        pathbuf.push(&filename);
        let mut rename_num = 1;
        loop {
            if !pathbuf.exists() {
                break;
            }
            pathbuf.set_file_name(format!("{}({})", filename, rename_num));
            rename_num += 1;
        }
        let mut file = fs::File::create(pathbuf).await.expect("create file failed");
        while let Some(res) = receive_res.message().await? {
            file.write(&res.data).await.expect("write file failed");
            println!("receiver {} bytes, range: {:?}", res.data.len(), res.range);
        }
    }

    Ok(())
}
