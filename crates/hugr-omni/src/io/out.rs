//! The consumer side of a child's output (contract §4). W10.

use crate::error::{Error, ErrorCode};
use crate::types::{Chunk, Line};

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
