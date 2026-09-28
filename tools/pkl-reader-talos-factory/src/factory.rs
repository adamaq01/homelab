//! [`protocol::ResourceReader`] for the `talos-factory+http:`/`talos-factory+https:` schemes.
//! See the crate-level docs and `talos/common/Schematic.pkl` for what calls into this and why.

use base64::Engine;
use serde::Deserialize;

use crate::protocol::ResourceReader;

pub struct TalosFactoryReader;

impl ResourceReader for TalosFactoryReader {
    fn handles_scheme(&self, scheme: &str) -> bool {
        matches!(scheme, "talos-factory+http" | "talos-factory+https")
    }

    fn read(&self, uri: &str) -> Result<Vec<u8>, String> {
        resolve_schematic_id(uri).map(String::into_bytes)
    }
}

#[derive(Deserialize)]
struct FactoryResponse {
    id: String,
}

/// `uri`'s scheme, host and path name the Factory endpoint to submit to (e.g.
/// `talos-factory+https://factory.talos.dev/schematics`); its `request` query parameter is the
/// base64-encoded, JSON-rendered schematic to submit. Returns the schematic ID Factory assigns.
fn resolve_schematic_id(uri: &str) -> Result<String, String> {
    let parsed = url::Url::parse(uri).map_err(|error| format!("invalid resource URI: {error}"))?;
    let factory_url = factory_url(&parsed)?;
    let schematic_json = decode_request(&parsed)?;

    let response = ureq::post(&factory_url)
        .header("Content-Type", "application/json")
        .send(&schematic_json)
        .map_err(|error| format!("Talos Image Factory request failed: {error}"))?;

    response
        .into_body()
        .read_json::<FactoryResponse>()
        .map(|body| body.id)
        .map_err(|error| format!("failed to parse Talos Image Factory response: {error}"))
}

fn factory_url(uri: &url::Url) -> Result<String, String> {
    let scheme = match uri.scheme() {
        "talos-factory+http" => "http",
        "talos-factory+https" => "https",
        other => return Err(format!("unsupported resource scheme: {other}")),
    };
    let host = uri
        .host_str()
        .ok_or_else(|| "resource URI has no host".to_string())?;
    Ok(format!("{scheme}://{host}{}", uri.path()))
}

fn decode_request(uri: &url::Url) -> Result<Vec<u8>, String> {
    let encoded_request = uri
        .query_pairs()
        .find(|(key, _)| key == "request")
        .map(|(_, value)| value.into_owned())
        .ok_or_else(|| "missing `request` query parameter".to_string())?;

    base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(encoded_request)
        .map_err(|error| format!("failed to base64-decode request: {error}"))
}
