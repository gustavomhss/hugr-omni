//! Frame codec (`docs/protocol.md`). W04: a pure function pair, tested on its own.

use crate::{Msg, ProtoError};

/// Appends one frame for `msg` to `out`.
pub fn encode(msg: &Msg, out: &mut Vec<u8>) -> Result<(), ProtoError> {
    let _ = (msg, out);
    Err(ProtoError("codec not implemented yet (W04)".into()))
}

/// Decodes one frame from the front of `buf`: `Ok(None)` = need more bytes; `Ok(Some((msg, used)))`.
pub fn decode(buf: &[u8]) -> Result<Option<(Msg, usize)>, ProtoError> {
    let _ = buf;
    Err(ProtoError("codec not implemented yet (W04)".into()))
}
