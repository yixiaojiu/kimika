use crate::send::SendArgs;
use crate::utils::crossterm::try_read_from_pipeline;
use crate::utils::Host;
use crate::CONFIG;

use indicatif::{ProgressBar, ProgressStyle};
use inquire::Select;
use inquire::{Confirm, InquireError};

pub fn create_progress_bar(total_size: u64, filename: &String) -> ProgressBar {
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::with_template("{msg:10.green} {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .unwrap()
        .progress_chars("#>-"));
    pb.set_message(filename.clone());
    pb
}

pub fn handle_address(address: Option<String>) -> Option<Host> {
    if let Some(address_str) = address {
        Some(Host::new(address_str))
    } else {
        if CONFIG.server.is_empty() {
            return None;
        }

        if CONFIG.auto_select_first_server {
            return Some(Host::new(CONFIG.server[0].address.clone()));
        }

        let options: Vec<String> = CONFIG
            .server
            .iter()
            .map(|server| format!("{:12} {}", server.alias, server.address))
            .collect();
        let answer = Select::new("Please select a remote server", options).prompt();

        match answer {
            Ok(options) => {
                let address = options.split_whitespace().skip(1).next().unwrap();
                return Some(Host::new(address.to_string()));
            }
            Err(err) => {
                if match err {
                    InquireError::OperationCanceled => false,
                    InquireError::OperationInterrupted => false,
                    _ => true,
                } {
                    eprintln!("Error: {}", err);
                }

                return None;
            }
        }
    }
}

pub fn handle_message(args: &SendArgs) -> Option<Vec<String>> {
    if let Some(message) = &args.message {
        Some(message.clone())
    } else {
        try_read_from_pipeline()
    }
}

pub fn get_mac_address() -> Option<String> {
    if let Ok(Some(ma)) = mac_address::get_mac_address() {
        Some(ma.to_string())
    } else {
        None
    }
}

pub fn handle_confirm(alias: &str) -> Result<bool, InquireError> {
    if CONFIG.receiver.auto_confirm {
        return Ok(true);
    }
    Confirm::new(&format!("Do you want to receive from [ {} ]?", alias))
        .with_default(true)
        .with_help_message("The default is true")
        .prompt()
}
