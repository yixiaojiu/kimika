use super::SendArgs;
use crate::{config, utils::handle, utils::select};
use crossterm::style::Stylize;
use std::{fs, path::PathBuf};

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

    Ok(())
}
