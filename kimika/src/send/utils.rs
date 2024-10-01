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
