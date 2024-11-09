use super::SendArgs;
use crate::request::remote as request_remote;
use crate::utils::{handle, select, Content, ContentType};
use crate::CONFIG;

use crossterm::style::Stylize;
use std::{path::PathBuf, sync::Arc};
use tokio::{sync::mpsc, time};
use uuid::Uuid;

pub async fn remote_send(args: &SendArgs) -> Result<(), Box<dyn std::error::Error>> {
    let mut content_list = Vec::new();
    if let Some(message) = handle::handle_message(args) {
        content_list.push(Content {
            id: Uuid::new_v4().to_string(),
            content_type: ContentType::Message,
            message: Some(message),
            path: None,
        });
    }
    if let Some(path) = &args.path {
        let pathbuf = PathBuf::from(path);

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
    };

    let address = if let Some(addr) = handle::handle_address(args.address.clone()) {
        addr
    } else {
        println!("{}", "No server address configured".red());
        return Ok(());
    };

    let request = Arc::new(request_remote::RequestClient::new(&address));

    let (tx, mut rx) = mpsc::channel::<Vec<select::SelectItem<String>>>(1);
    let request_clone = Arc::clone(&request);
    tokio::spawn(async move {
        loop {
            if tx.is_closed() {
                break;
            }
            let res = request_clone.get_receivers().await.unwrap();
            let receiver_iter = res.receivers.iter().map(|receiver| select::SelectItem {
                id: receiver.id.clone(),
                label: receiver.alias.clone(),
            });
            let result = tx.send(receiver_iter.collect()).await;
            if result.is_err() {
                break;
            }
            time::sleep(time::Duration::from_secs(2)).await;
        }
    });

    let selected_receiver_id = if let Some(selected_item) = select::receiver_select(&mut rx)
        .await
        .expect("select receiver failed")
    {
        selected_item.id
    } else {
        return Ok(());
    };
    // close channel
    drop(rx);

    let metadata_list: Vec<request_remote::Metadata> = content_list
        .iter()
        .map(|content| match content.content_type {
            ContentType::Message => request_remote::Metadata {
                id: content.id.clone(),
                metadata_type: "message".to_string(),
                file_name: None,
                file_type: None,
                size: None,
            },
            ContentType::File => {
                let pathbuf = content.path.as_ref().unwrap();
                let metadata = std::fs::metadata(pathbuf).expect("get metadata failed");
                let filename = pathbuf
                    .file_name()
                    .expect("invalid file name")
                    .to_str()
                    .unwrap();
                request_remote::Metadata {
                    id: content.id.clone(),
                    metadata_type: "file".to_string(),
                    file_name: Some(filename.to_string()),
                    file_type: Some("text".to_string()),
                    size: Some(metadata.len()),
                }
            }
        })
        .collect();

    let res = request
        .post_metadata(&request_remote::PostMetadataPayload {
            receiver_id: selected_receiver_id.clone(),
            alias: CONFIG.alias.clone(),
            metadata: metadata_list,
        })
        .await
        .expect("POST metadata failed");

    for metadata in res.selected_metadata_list {
        let content_list = content_list.clone();
        let sender_id = res.id.clone();
        let receiver_id = selected_receiver_id.clone();
        let token = metadata.token.clone();

        let content = content_list
            .iter()
            .find(|content| content.id == metadata.id)
            .unwrap();
        request
            .post_upload(
                content,
                request_remote::PostUploadParams {
                    id: sender_id,
                    receiver: receiver_id,
                    token,
                },
            )
            .await
            .expect("post upload failed");
    }

    Ok(())
}
