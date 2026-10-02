//! Input validation before any syscall (C-ERR-02, C-SPAWN-03). W03.

use std::path::PathBuf;

use super::Request;
use crate::error::{Error, ErrorCode};

/// Rejects invalid input (NUL bytes, bad PTY size, ...) with `InvalidArgument` naming the field.
pub(super) fn check(req: &Request) -> Result<(), Error> {
    let _ = req;
    Err(Error::new(ErrorCode::Io, "not implemented yet (W03: spawn/validate)"))
}

/// The absolute working directory (`InvalidCwd` if it is missing or not a directory).
pub(super) fn cwd(req: &Request) -> Result<PathBuf, Error> {
    let _ = req;
    Err(Error::new(ErrorCode::Io, "not implemented yet (W03: spawn/validate)"))
}
