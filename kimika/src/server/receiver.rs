use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone, Debug)]
pub struct MetadataItem {
    pub id: String,
    pub token: String,
    /// file or message
    pub metadata_type: String,
    pub file_name: Option<String>,
    pub file_type: Option<String>,
    pub size: Option<u64>,
}

pub struct Metadata {
    pub metadata_list: Vec<MetadataItem>,
}

pub struct ReceiverServer {
    metadata: Arc<Mutex<dashmap::DashMap<String, Metadata>>>,
}

impl Clone for ReceiverServer {
    fn clone(&self) -> Self {
        Self {
            metadata: Arc::clone(&self.metadata),
        }
    }
}
