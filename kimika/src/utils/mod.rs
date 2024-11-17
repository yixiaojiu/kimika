pub mod clap;
pub mod crossterm;
pub mod handle;
pub mod select;

use reqwest::Url;
use std::path::PathBuf;

#[derive(Clone)]
pub enum ContentType {
    File,
    Message,
}

#[derive(Clone)]
pub struct Content {
    pub content_type: ContentType,
    pub id: String,
    pub message: Option<String>,
    pub path: Option<PathBuf>,
}

pub struct Host {
    inner: String,
}

impl Host {
    pub fn new(host: String) -> Self {
        Self { inner: host }
    }

    pub fn url(&self, ssl: bool) -> Url {
        Url::parse(&format!(
            "{}://{}",
            if ssl { "https" } else { "http" },
            self.inner
        ))
        .unwrap()
    }
}
