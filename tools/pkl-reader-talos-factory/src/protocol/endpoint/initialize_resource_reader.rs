use serde::{Deserialize, Serialize};

use super::endpoint;
use super::{Message, Request};
use crate::protocol::ResourceReader;

endpoint! {
    request InitializeResourceReaderRequest = 0x30 {
        request_id: i64,
        scheme: String,
    }
    response InitializeResourceReaderResponse = 0x31 {
        request_id: i64,
        #[serde(skip_serializing_if = "Option::is_none")]
        spec: Option<ResourceReaderSpec>,
    }
    handle(self, reader) {
        let spec = reader.handles_scheme(&self.scheme).then(|| ResourceReaderSpec {
            scheme: self.scheme.clone(),
            has_hierarchical_uris: false,
            is_globbable: false,
        });

        InitializeResourceReaderResponse { request_id: self.request_id, spec }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ResourceReaderSpec {
    scheme: String,
    has_hierarchical_uris: bool,
    is_globbable: bool,
}
