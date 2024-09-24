use super::SendArgs;
use crate::utils::request::get_receivers;
use crate::{config, utils::handle, utils::select};
use crossterm::style::Stylize;
use std::{fs, path::PathBuf};
use tokio::{sync::mpsc, time};

pub struct Content {
    pub message: Option<String>,
    pub path: Option<PathBuf>,
    pub name: Option<String>,
    pub size: Option<u64>,
}

pub async fn remote_send(
    args: &SendArgs,
    config: &config::Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let message = handle::handle_message(args);

    let content = if let Some(path) = &args.path {
        let pathbuf = PathBuf::from(path);
        if !pathbuf.exists() {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "file not exists",
            ))?
        }
        let metadata = fs::metadata(&pathbuf).expect("get metadata failed");
        let filename = pathbuf
            .file_name()
            .expect("invalid file name")
            .to_str()
            .unwrap();
        Content {
            message: None,
            path: Some(pathbuf.clone()),
            name: Some(filename.to_string()),
            size: Some(metadata.len()),
        }
    } else {
        Content {
            message,
            path: None,
            name: None,
            size: None,
        }
    };

    let address = if let Some(addr) = handle::handle_address(args.address.clone(), config) {
        addr
    } else {
        println!("{}", "No server address configured".red());
        return Ok(());
    };

    let (tx, mut rx) = mpsc::channel::<Vec<select::SelectItem<String>>>(1);

    #[warn(while_true)]
    loop {
        let res = get_receivers(&address).await.expect("");
        let receiver_iter = res.receivers.iter().map(|receiver| select::SelectItem {
            id: receiver.id.clone(),
            label: receiver.alias.clone(),
        });
        let result = tx.send(receiver_iter.collect()).await;
        if result.is_err() {
            break;
        }
        time::sleep(time::Duration::from_secs(1)).await;
    }

    let selected_receiver_id = if let Some(id) = select::receiver_select(&mut rx)
        .await
        .expect("select receiver failed")
    {
        id
    } else {
        return Ok(());
    };

    Ok(())
}
