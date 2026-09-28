use serde::{Deserialize, Serialize};

use super::endpoint;
use super::{Message, Request};
use crate::protocol::ResourceReader;

endpoint! {
    request ReadResourceRequest = 0x26 {
        request_id: i64,
        evaluator_id: i64,
        uri: String,
    }
    response ReadResourceResponse = 0x27 {
        request_id: i64,
        evaluator_id: i64,
        #[serde(skip_serializing_if = "Option::is_none")]
        contents: Option<serde_bytes::ByteBuf>,
        #[serde(skip_serializing_if = "Option::is_none")]
        error: Option<String>,
    }
    handle(self, reader) {
        let (contents, error) = match reader.read(&self.uri) {
            Ok(bytes) => (Some(serde_bytes::ByteBuf::from(bytes)), None),
            Err(message) => (None, Some(message)),
        };

        ReadResourceResponse {
            request_id: self.request_id,
            evaluator_id: self.evaluator_id,
            contents,
            error,
        }
    }
}
