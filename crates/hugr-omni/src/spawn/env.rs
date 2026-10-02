//! The child's final environment (C-ENV-01). W03.

use std::ffi::OsString;

use super::Request;

/// Inherited (unless `inherit_env` is false) + overrides in call order; keys case-insensitive on Windows.
pub(super) fn build(req: &Request) -> Vec<(OsString, OsString)> {
    let _ = req;
    Vec::new()
}
