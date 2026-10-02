//! For the language bindings only (hidden from the docs): the contract §3 rules for numbers that only TS and Python
//! can express wrongly (NaN, infinities, negatives, fractions), with the same error texts as every other check.
//! Rust callers never need it: `Duration`, `usize` and `u16` cannot hold those values. W03.
//!
//! SEAM (frozen in W00): `millis`, `byte_limit`, `pty_side`. Bodies belong to W03.

use std::time::Duration;

use crate::error::{Error, ErrorCode};

fn todo() -> Error {
    Error::new(ErrorCode::Io, "not implemented yet (W03: binding)")
}

/// `timeoutMs` / `graceMs`: finite, from 0 to 4294967295; a fraction rounds up to the next millisecond.
pub fn millis(field: &str, value: f64) -> Result<Duration, Error> {
    let _ = (field, value);
    Err(todo())
}

/// `maxOutputBytes`: an integer from 0.
pub fn byte_limit(field: &str, value: f64) -> Result<usize, Error> {
    let _ = (field, value);
    Err(todo())
}

/// `pty.cols` / `pty.rows`: an integer from 1 to 32767.
pub fn pty_side(field: &str, value: f64) -> Result<u16, Error> {
    let _ = (field, value);
    Err(todo())
}
