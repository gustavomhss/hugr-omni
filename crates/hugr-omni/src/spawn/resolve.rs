//! Program resolution against the child's final PATH (+ PATHEXT on Windows) (C-SPAWN-01, C-SPAWN-03). W03.

use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

use crate::error::{Error, ErrorCode};

/// Absolute path of the program to execute. Never executes anything.
pub(super) fn program(program: &OsStr, env: &[(OsString, OsString)], cwd: &Path) -> Result<PathBuf, Error> {
    let _ = (program, env, cwd);
    Err(Error::new(ErrorCode::Io, "not implemented yet (W03: spawn/resolve)"))
}
