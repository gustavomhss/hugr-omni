//! The life of a child: spawn, `wait`, `stop`, `processes`, the `Exit` and its `reason`, timeout and
//! cancellation, and `run()` (W07: `child`, `exit`; W09: `deadline`, `run`).
//!
//! SEAM (frozen in W00): the public types and methods re-exported by `api`, `Options`, `spawn_pipe`,
//! `spawn_pty` and `run`. Bodies and private fields belong to the owners above.

mod child;
mod deadline;
mod exit;
mod run;

use std::time::Duration;

use tokio_util::sync::CancellationToken;

pub use child::{Child, DroppedBytes, PipeChild, PtyChild};
pub(crate) use child::{spawn_pipe, spawn_pty};
pub use exit::{Exit, ProcessInfo, Reason, RunOutput};
pub(crate) use run::run;

/// Default `max_output_bytes` per stream (contract §6).
pub(crate) const DEFAULT_MAX_OUTPUT: usize = 16 << 20;

/// The options that are not about launching (those are in `spawn::Request`).
#[derive(Debug, Clone)]
pub(crate) struct Options {
    pub timeout: Option<Duration>,
    pub cancel: Option<CancellationToken>,
    pub text: bool,
    pub input: Option<Vec<u8>>,
    pub max_output_bytes: usize,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            timeout: None,
            cancel: None,
            text: true,
            input: None,
            max_output_bytes: DEFAULT_MAX_OUTPUT,
        }
    }
}
