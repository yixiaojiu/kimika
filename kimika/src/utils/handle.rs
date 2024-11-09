use crate::send::SendArgs;
use crate::utils::crossterm::try_read_from_pipeline;
use crate::CONFIG;

use crossterm::style::Stylize;
use indicatif::{ProgressBar, ProgressStyle};
use std::net::SocketAddr;

pub fn create_progress_bar(total_size: u64, filename: &String) -> ProgressBar {
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::with_template("{msg:.green} {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .unwrap()
        .progress_chars("#>-"));
    pb.set_message(filename.clone());
    pb
}

pub fn handle_address(address: Option<String>) -> Option<SocketAddr> {
    if let Some(address_str) = address {
        // search through alias
        Some(address_str.parse::<SocketAddr>().expect("invalid address"))
    } else {
        if CONFIG.server.is_empty() {
            println!("{}", "No server address configured".red());
            None
        } else {
            Some(
                CONFIG.server[0]
                    .address
                    .parse::<SocketAddr>()
                    .expect("invalid address"),
            )
        }
    }
}

pub fn handle_message(args: &SendArgs) -> Option<String> {
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
