//! Output pumps with bounded buffering, UTF-8 decoding, lines, stdin, and `run()` collection (W10).
//!
//! SEAM (frozen in W00 and, for the internal part, at W10's dispatch): `Output` and `Lines` (re-exported by
//! `api`), `Pipe`, `Source`, `Pumps` and its methods, `Collected`, `Stdin` and its methods. Bodies and every
//! private item belong to W10. `process` (W07/W09) and `pty` (W12) are the callers.
//!
//! Rules every caller can rely on (contract §4, §6, §9):
//! - the pumps start draining at once and never stop until every source reached end of file or `end()` was
//!   called, so a child never blocks on output;
//! - one consumer at a time: `output()` or `lines()` claims it; a second claim, or a claim after the consumer
//!   was dropped, is `InvalidArgument`; dropping it detaches for good;
//! - while a consumer is attached up to 16 MiB per stream wait for it; otherwise up to 1 MiB per stream is
//!   kept; anything beyond is dropped, counted in `dropped()`, and reported in order (`lost_before`);
//! - nothing the child wrote before end of file is lost.

mod out;

use std::time::Duration;

pub use out::{Lines, Output};

use crate::error::{Error, ErrorCode};
use crate::types::{Data, DroppedBytes, Stream};

/// A host-side pipe or terminal end (the same type `client` hands out).
#[cfg(unix)]
pub(crate) type Pipe = std::os::fd::OwnedFd;
/// A host-side pipe or terminal end (the same type `client` hands out).
#[cfg(windows)]
pub(crate) type Pipe = std::os::windows::io::OwnedHandle;

/// Where a child's output comes from.
#[derive(Debug)]
pub(crate) enum Source {
    /// stdout, and stderr unless it was merged into stdout.
    Pipes { stdout: Pipe, stderr: Option<Pipe> },
    /// Everything the terminal shows (`Stream::Pty`).
    Pty { output: Pipe },
}

fn todo() -> Error {
    Error::new(ErrorCode::Io, "not implemented yet (W10: io)")
}

/// The output side of one child.
#[derive(Debug)]
pub(crate) struct Pumps {}

impl Pumps {
    /// Starts draining `source` at once on `rt` (the library's own runtime). `text` selects `Data::Text`
    /// (UTF-8 decoded across chunks, restarting at every gap) or `Data::Bytes`.
    pub(crate) fn start(rt: &tokio::runtime::Handle, source: Source, text: bool) -> Result<Pumps, Error> {
        let _ = (rt, source, text);
        Err(todo())
    }

    /// Claims the single consumer, as chunks.
    pub(crate) fn output(&self) -> Result<Output, Error> {
        Err(todo())
    }

    /// Claims the single consumer, as lines (always text).
    pub(crate) fn lines(&self) -> Result<Lines, Error> {
        Err(todo())
    }

    /// Bytes dropped so far, per stream (terminal output counts as stdout).
    pub(crate) fn dropped(&self) -> DroppedBytes {
        DroppedBytes::default()
    }

    /// Resolves once every source reached end of file (every holder of the pipes closed them), or once
    /// `end()` was called. Never requires a consumer.
    pub(crate) async fn ended(&self) {}

    /// Stops reading (the tree is gone, contract §5): a consumer gets what is already buffered, then the end.
    /// Idempotent.
    pub(crate) fn end(&self) {}

    /// For `run()`: claims the consumer and keeps every byte, up to `max` per stream. Returns once the output
    /// ended, or as soon as a stream went over `max` (then `over_limit` names it and each stream holds its
    /// first `max` bytes; the caller stops the tree). `within` bounds the wait after `end()`.
    pub(crate) async fn collect(&self, max: usize, within: Duration) -> Result<Collected, Error> {
        let _ = (max, within);
        Err(todo())
    }
}

/// What `run()` collected.
#[derive(Debug)]
pub(crate) struct Collected {
    pub stdout: Data,
    /// Empty for a terminal and with `merge_stderr`.
    pub stderr: Data,
    /// The stream that went over the limit, if any.
    pub over_limit: Option<Stream>,
}

/// The input side of one child: a pipe to its stdin, or the terminal's input.
#[derive(Debug)]
pub(crate) struct Stdin {}

impl Stdin {
    /// Takes ownership of the host's end of the child's input.
    pub(crate) fn start(rt: &tokio::runtime::Handle, sink: Pipe) -> Result<Stdin, Error> {
        let _ = (rt, sink);
        Err(todo())
    }

    /// Resolves when the OS accepted all of `data` (backpressure). Writes apply in call order, never
    /// interleaved. `Closed` after `close()`, after `abandon()`, or when the child closed its end.
    pub(crate) async fn write(&self, data: &[u8]) -> Result<(), Error> {
        let _ = data;
        Err(todo())
    }

    /// Waits for queued writes, then closes; idempotent.
    pub(crate) async fn close(&self) -> Result<(), Error> {
        Err(todo())
    }

    /// The child exited or the tree was stopped: queued and later writes settle with `Closed`. Idempotent.
    pub(crate) fn abandon(&self) {}
}
