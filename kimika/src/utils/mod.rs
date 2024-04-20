pub mod clap;
pub mod color;
pub mod select;
pub mod udp;

use indicatif::{ProgressBar, ProgressStyle};
use std::io::Read;

pub fn stdin_to_string() -> String {
    let mut string = String::new();

    std::io::stdin()
        .read_to_string(&mut string)
        .expect("read standard input failed");

    string
}

pub fn create_progress_bar(total_size: u64, filename: &String) -> ProgressBar {
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::with_template("{msg:.green} {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .unwrap()
        .progress_chars("#>-"));
    pb.set_message(filename.clone());
    pb
}
