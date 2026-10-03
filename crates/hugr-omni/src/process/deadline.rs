//! Timeout and cancellation: one explicit state machine, every waiter resolved exactly once. W09.
//!
//! SEAM (frozen by the lead before W09): `refuse_if_cancelled` and `arm`, called by `spawn_pipe` and `spawn_pty`.
//! Bodies and every private item belong to W09.

use std::sync::Arc;
use std::time::Duration;

use tokio::runtime::Handle;
use tokio_util::sync::CancellationToken;

use super::Options;
use super::child::Inner;
use crate::error::Error;

/// `ABORTED` when `opts.cancel` is already cancelled, so nothing runs (contract §8). Called after validation and
/// before the spawn.
pub(super) fn refuse_if_cancelled(opts: &Options) -> Result<(), Error> {
    let _ = opts;
    Ok(())
}

/// Arms the timeout (measured from now, the spawn) and the cancellation of a started child, on the library's
/// runtime `rt`. Whichever fires first stops the whole tree with its cause (`Timeout` or `Aborted`) and the
/// command's grace, through `Inner::stop_with`; neither changes the `Exit` of a root that already exited (§7).
/// Never blocks; the armed work ends once the tree is gone.
pub(super) fn arm(rt: &Handle, inner: &Arc<Inner>, timeout: Option<Duration>, cancel: Option<CancellationToken>) {
    let _ = (rt, inner, timeout, cancel);
}
