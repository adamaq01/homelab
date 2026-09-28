//! [`endpoint!`] declares one request/response endpoint: two structs, each with its own wire
//! code, and the handler that turns the request into the response.

mod initialize_resource_reader;
mod read_resource;

use serde::{Deserialize, Serialize};

pub(super) use initialize_resource_reader::InitializeResourceReaderRequest;
pub(super) use read_resource::ReadResourceRequest;
use super::ResourceReader;

pub(super) trait Message {
    const CODE: i64;
}

/// A request paired with the response it produces.
pub(super) trait Request: for<'de> Deserialize<'de> {
    type Response: Serialize + Message;

    fn handle(self, reader: &dyn ResourceReader) -> Self::Response;
}

/// ```ignore
/// endpoint! {
///     request SomeRequest = 0x01 { request_id: i64 }
///     response SomeResponse = 0x02 { request_id: i64 }
///     handle(self, reader) { SomeResponse { request_id: self.request_id } }
/// }
/// ```
macro_rules! endpoint {
    (
        request $req:ident = $req_code:literal {
            $($(#[$req_attr:meta])* $req_field:ident : $req_ty:ty),* $(,)?
        }
        response $resp:ident = $resp_code:literal {
            $($(#[$resp_attr:meta])* $resp_field:ident : $resp_ty:ty),* $(,)?
        }
        handle($self:ident, $reader:ident) $handle:block
    ) => {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub(in crate::protocol) struct $req {
            $($(#[$req_attr])* $req_field: $req_ty,)*
        }

        impl Message for $req {
            const CODE: i64 = $req_code;
        }

        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        pub(in crate::protocol) struct $resp {
            $($(#[$resp_attr])* $resp_field: $resp_ty,)*
        }

        impl Message for $resp {
            const CODE: i64 = $resp_code;
        }

        impl Request for $req {
            type Response = $resp;

            fn handle($self, $reader: &dyn ResourceReader) -> $resp $handle
        }
    };
}

use endpoint;
