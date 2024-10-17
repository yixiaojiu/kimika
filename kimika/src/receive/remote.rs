use super::ReceiveArgs;
use crate::request::remote as request_remote;
use crate::utils;
use crate::{config, utils::handle};
use crossterm::style::Stylize;
use std::path::PathBuf;
use tokio::io::{AsyncWriteExt, BufWriter};
use tokio::{fs, time};
use tokio_stream::StreamExt;

pub async fn remote_receive(
    args: &ReceiveArgs,
    config: &config::Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let address = if let Some(addr) = handle::handle_address(args.address.clone(), config) {
        addr
    } else {
        println!("{}", "No server address configured".red());
        return Ok(());
    };

    let request = request_remote::RequestClient::new(&address);

    let receiver_id = request.post_register(config.alias.clone()).await?.id;

    let mut metadatas = Vec::new();
    loop {
        let result = request.get_metadata(receiver_id.clone()).await;
        if result.is_ok() {
            metadatas = result?.metadatas;
        }
        if metadatas.len() > 0 {
            break;
        }
        time::sleep(time::Duration::from_secs(2)).await;
    }

    // TODO: select metadata

    request
        .post_select_metadata(&request_remote::PostSelectMetadataPayload {
            id: receiver_id.clone(),
            selected_tokens: metadatas.iter().map(|item| item.token.clone()).collect(),
        })
        .await?;

    for metadata in metadatas {
        let res = request
            .post_download(metadata.token.clone(), receiver_id.clone())
            .await?;

        if metadata.metadata_type == "file" {
            let mut pathbuf = PathBuf::from(config.receiver.save_folder.clone());
            let filename = metadata.file_name.unwrap();
            pathbuf.push(&filename);
            let mut rename_num = 1;
            loop {
                if !pathbuf.exists() {
                    break;
                }
                pathbuf.set_file_name(format!("{}({})", &filename, rename_num));
                rename_num += 1;
            }
            let total_size = metadata.size.unwrap();
            let progreebar = utils::handle::create_progress_bar(total_size, &filename);
            let mut buffer_writer = BufWriter::new(fs::File::create(pathbuf).await?);
            let mut downloaded_size = 0;
            let mut chunks = res.bytes_stream();
            while let Some(chunk) = chunks.next().await {
                let chunk = chunk?;
                buffer_writer.write(&chunk).await?;
                downloaded_size += chunk.len() as u64;
                progreebar.set_position(std::cmp::min(downloaded_size, total_size));
            }
            buffer_writer.flush().await?;
            progreebar.finish_with_message(filename);
        } else {
            println!("{}", String::from_utf8_lossy(&res.bytes().await?));
        }
    }

    Ok(())
}
