//! The consumer side of a child's output (contract §4). W10.

use std::fmt;

use crate::error::{Error, ErrorCode};

/// Which stream a chunk or line came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Stream {
    /// Standard output (and stderr with `merge_stderr`).
    Stdout,
    /// Standard error.
    Stderr,
    /// Everything a terminal shows.
    Pty,
}

/// Output data: text with `text(true)` (the default), raw bytes with `text(false)`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Data {
    /// UTF-8 text, decoded across chunks; invalid bytes become U+FFFD.
    Text(String),
    /// Raw bytes, lossless.
    Bytes(Vec<u8>),
}

impl Data {
    /// The data as bytes (text as its UTF-8 encoding).
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Data::Text(s) => s.as_bytes(),
            Data::Bytes(b) => b,
        }
    }
}

impl fmt::Display for Data {
    /// Text as is; bytes decoded lossily.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Data::Text(s) => f.write_str(s),
            Data::Bytes(b) => f.write_str(&String::from_utf8_lossy(b)),
        }
    }
}

/// A piece of output.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Chunk {
    /// Where it came from.
    pub stream: Stream,
    /// The data (empty only on the final item that reports a gap before the end).
    pub data: Data,
    /// Bytes dropped right before this item (contract §4).
    pub lost_before: Option<u64>,
}

/// A line of output, without its `\n` (and a trailing `\r`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Line {
    /// Where it came from.
    pub stream: Stream,
    /// The text.
    pub text: String,
    /// Bytes dropped right before this line (contract §4).
    pub lost_before: Option<u64>,
    /// `true` on every piece but the last of a line longer than 1 MiB.
    pub continues: bool,
}

fn todo() -> Error {
    Error::new(ErrorCode::Io, "not implemented yet (W10: io)")
}

/// The single output consumer, as chunks. Dropping it detaches for good.
#[derive(Debug)]
pub struct Output {}

impl Output {
    /// The next chunk; `None` when the output ended.
    pub async fn next(&mut self) -> Option<Result<Chunk, Error>> {
        Some(Err(todo()))
    }
}

/// The single output consumer, as lines. Dropping it detaches for good.
#[derive(Debug)]
pub struct Lines {}

impl Lines {
    /// The next line; `None` when the output ended.
    pub async fn next(&mut self) -> Option<Result<Line, Error>> {
        Some(Err(todo()))
    }
}
