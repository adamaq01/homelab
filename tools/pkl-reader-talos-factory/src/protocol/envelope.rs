//! The `[code, body]` message envelope used by every protocol message.

use std::io::{ErrorKind, Read, Write};

use anyhow::{Context, Result};
use rmpv::Value;
use serde::{Deserialize, Serialize};

/// Incoming messages are read as `Envelope<Value>` (untyped until dispatched on `code`) then
/// converted via [`decode`](Envelope::decode). Outgoing messages are built directly as
/// `Envelope<T>` for the response type at hand and sent via [`write`](Envelope::write).
pub(super) struct Envelope<T = Value> {
    pub(super) code: i64,
    body: T,
}

impl<T> Envelope<T> {
    pub(super) fn new(code: i64, body: T) -> Self {
        Envelope { code, body }
    }
}

impl Envelope<Value> {
    /// Reads one message, or `None` on a clean stream close (`pkl` exiting without sending
    /// `CloseExternalProcess` first).
    pub(super) fn read(input: &mut impl Read) -> Result<Option<Self>> {
        let value = match rmpv::decode::read_value(input) {
            Ok(value) => value,
            Err(error) if error.kind() == ErrorKind::UnexpectedEof => return Ok(None),
            Err(error) => return Err(error).context("failed to decode a message from pkl"),
        };

        let mut items = value.as_array().context("message was not an array")?.to_vec();
        let body = if items.len() > 1 { items.swap_remove(1) } else { Value::Nil };
        let code = items.first().and_then(Value::as_i64).context("message had no numeric code")?;

        Ok(Some(Envelope { code, body }))
    }

    pub(super) fn decode<T: for<'de> Deserialize<'de>>(self) -> Result<T> {
        rmpv::ext::from_value(self.body).context("failed to decode a message body from pkl")
    }
}

impl<T: Serialize> Envelope<T> {
    pub(super) fn write(&self, output: &mut impl Write) -> Result<()> {
        let mut serializer = rmp_serde::Serializer::new(&mut *output).with_struct_map();
        (self.code, &self.body).serialize(&mut serializer).context("failed to encode a message to pkl")?;
        output.flush().context("failed to flush a message to pkl")
    }
}
