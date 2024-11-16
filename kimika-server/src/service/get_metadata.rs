use super::Server;
use crate::utils::hyper_utils;
use crate::utils::types;

use bytes::Bytes;
use hyper::Response;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct Params {
    /// receiver id
    id: String,
}

#[derive(Clone, Serialize)]
pub struct MetadataItem {
    pub id: String,
    pub token: String,
    /// file or message
    pub metadata_type: String,
    pub file_name: Option<String>,
    pub file_type: Option<String>,
    pub size: Option<u64>,
}

#[derive(Serialize)]
struct ResponseBody {
    metadatas: Vec<MetadataItem>,
    /// sender alias
    alias: Option<String>,
}

impl Server {
    pub async fn get_metadata(self, req: types::RequestType) -> types::ResponseType {
        // TODO none hander
        let query = req.uri().query().unwrap();
        let params: Params = serde_qs::from_str(query)?;

        let metadata_guard = self.metadata.lock().await;
        let metadata_entry = metadata_guard.get(&params.id);

        if let Some(metadata) = metadata_entry {
            let metadatas = metadata
                .metadata_list
                .iter()
                .map(|v| MetadataItem {
                    id: v.id.clone(),
                    token: v.token.clone(),
                    metadata_type: v.metadata_type.clone(),
                    file_name: v.file_name.clone(),
                    file_type: v.file_type.clone(),
                    size: v.size.clone(),
                })
                .collect();
            let body = hyper_utils::full(Bytes::from(
                serde_json::to_string(&ResponseBody {
                    metadatas,
                    alias: Some(metadata.sender.alias.clone()),
                })
                .unwrap(),
            ));
            let res = Response::new(body);
            Ok(res)
        } else {
            let body = hyper_utils::full(Bytes::from(
                serde_json::to_string(&ResponseBody {
                    metadatas: vec![],
                    alias: None,
                })
                .unwrap(),
            ));
            let mut res = Response::new(body);
            *res.status_mut() = hyper::StatusCode::BAD_REQUEST;
            Ok(res)
        }
    }
}
