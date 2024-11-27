use super::SendArgs;
use crate::request::{local as request_local, udp};
use crate::server::receiver;
use crate::server::sender;
use crate::utils::{handle, select, Content, ContentType};

use std::net::SocketAddr;
use tokio::sync::{mpsc, oneshot};
use uuid::Uuid;

pub async fn local_send(args: &SendArgs) -> Result<(), Box<dyn std::error::Error>> {
    let mut content_list = Vec::new();
    if let Some(message) = handle::handle_message(args) {
        for message_item in message {
            content_list.push(Content {
                id: Uuid::new_v4().to_string(),
                content_type: ContentType::Message,
                message: Some(message_item),
                path: None,
            });
        }
    }
    if let Some(path) = &args.path {
        for path_item in path {
            let pathbuf = path_item.clone();

            if !pathbuf.exists() {
                return Err("file not exists".into());
            }
            if pathbuf.is_dir() {
                return Err("send directory is not supported".into());
            }
            content_list.push(Content {
                id: Uuid::new_v4().to_string(),
                content_type: ContentType::File,
                message: None,
                path: Some(pathbuf),
            });
        }
    };

    let (close_boardcast_tx, close_boardcast_rx) = oneshot::channel::<()>();

    tokio::spawn(async move {
        udp::broadcast(close_boardcast_rx).await.unwrap();
    });

    let (receiver_tx, mut receiver_rx) = mpsc::channel(1);
    let (close_server_tx, close_server_rx) = oneshot::channel::<()>();
    tokio::spawn(async move {
        sender::start_server(receiver_tx, close_server_rx)
            .await
            .unwrap();
    });

    let (options_tx, mut options_rx) = mpsc::channel(1);
    tokio::spawn(async move {
        let mut options: Vec<select::SelectItem<String>> = Vec::new();

        loop {
            match receiver_rx.recv().await {
                Some(receiver) => {
                    let address = receiver.address.to_string();
                    if options.iter().any(|option| option.id == address) {
                        continue;
                    }
                    options.push(select::SelectItem::new(
                        address,
                        format!("{:12} {}", receiver.alias, receiver.address),
                    ));
                    options_tx.send(options.clone()).await.unwrap();
                    continue;
                }
                None => break,
            }
        }
    });

    let address: SocketAddr =
        if let Some(selected_item) = select::receiver_select(&mut options_rx).await? {
            selected_item.id.parse()?
        } else {
            return Ok(());
        };

    // close channel, boardcast and server
    drop(options_rx);
    if let Err(e) = close_boardcast_tx.send(()) {
        eprintln!("Error: {:?}", e);
    };
    if let Err(e) = close_server_tx.send(()) {
        eprintln!("Error: {:?}", e);
    };

    let request = request_local::RequestClient::new(&address);

    let metadata_list: Vec<receiver::PayloadMetadataItem> = content_list
        .iter()
        .map(|content| match content.content_type {
            ContentType::Message => receiver::PayloadMetadataItem {
                file_name: None,
                file_type: None,
                size: None,
                metadata_type: "message".to_string(),
                id: content.id.clone(),
            },
            ContentType::File => {
                let pathbuf = content.path.as_ref().unwrap();
                let metadata = std::fs::metadata(pathbuf).expect("get metadata failed");
                let filename = pathbuf
                    .file_name()
                    .expect("invalid file name")
                    .to_str()
                    .unwrap();
                receiver::PayloadMetadataItem {
                    id: content.id.clone(),
                    metadata_type: "file".to_string(),
                    file_name: Some(filename.to_string()),
                    file_type: Some("text".to_string()),
                    size: Some(metadata.len()),
                }
            }
        })
        .collect();

    let res = request.post_metadata(metadata_list).await?;

    for metadata in res.selected_metadata_list {
        let content_list = content_list.clone();
        let token = metadata.token.clone();
        let content = content_list
            .iter()
            .find(|content| content.id == metadata.id)
            .unwrap();

        request.post_upload(content, token).await?;
    }

    Ok(())
}
