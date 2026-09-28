//! Pkl's external resource reader protocol: `[code, body]` MessagePack messages over stdio, with
//! no extra framing. Only the codes a resource reader needs are implemented here, not module
//! readers or the `pkl server` embedding direction. See
//! <https://pkl-lang.org/main/current/bindings-specification/message-passing-api.html>.

mod envelope;
mod endpoint;

use std::io::{Read, Write};

use anyhow::Result;
use envelope::Envelope;
use endpoint::{InitializeResourceReaderRequest, Message, ReadResourceRequest, Request};
use rmpv::Value;

/// The URI scheme(s) a reader process serves.
pub trait ResourceReader {
    /// `scheme` is without the trailing `:` (e.g. `"talos-factory"`).
    fn handles_scheme(&self, scheme: &str) -> bool;

    /// Resolves one resource URI to its contents, or an error message to surface back in Pkl.
    fn read(&self, uri: &str) -> Result<Vec<u8>, String>;
}

/// Sent once by `pkl`, with no response, when it's done with this process.
const CLOSE_EXTERNAL_PROCESS: i64 = 0x32;

/// Runs the protocol loop against `input`/`output`, dispatching to `reader`, until `pkl` closes
/// the process or the stream ends.
pub fn serve(reader: &dyn ResourceReader, mut input: impl Read, mut output: impl Write) -> Result<()> {
    while let Some(envelope) = Envelope::<Value>::read(&mut input)? {
        match envelope.code {
            code if code == InitializeResourceReaderRequest::CODE => {
                respond::<InitializeResourceReaderRequest>(&mut output, envelope, reader)?
            }
            code if code == ReadResourceRequest::CODE => {
                respond::<ReadResourceRequest>(&mut output, envelope, reader)?
            }
            CLOSE_EXTERNAL_PROCESS => break,
            _ => {}
        }
    }
    Ok(())
}

fn respond<Req: Request>(output: &mut impl Write, envelope: Envelope<Value>, reader: &dyn ResourceReader) -> Result<()> {
    let request: Req = envelope.decode()?;
    let response = request.handle(reader);
    Envelope::new(<Req::Response as Message>::CODE, response).write(output)
}
