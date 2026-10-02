//! The host's side of the supervisor (ADR-0005, `docs/protocol.md`): lazy start once per host process,
//! restart by generation after supervisor death, creator-pid check, non-blocking bounded channel,
//! pipe creation and descriptor transfer. W04.
//!
//! SEAM (frozen in W00): `spawn`, `Spawned`, `HostStdio`, `Pipe`, `Tree` and its methods, `Stopped`.
//! Bodies and private items belong to W04. Every failure of the supervisor or the channel is `Io`;
//! after one, every pending call of that generation fails and the next `spawn` starts a new supervisor.

use std::time::Duration;

pub(crate) use omni_proto::{Exit as RootExit, ProcEntry};

use crate::error::{Error, ErrorCode};
use crate::spawn::Spec;

/// A host-side pipe or terminal end.
#[cfg(unix)]
pub(crate) type Pipe = std::os::fd::OwnedFd;
/// A host-side pipe or terminal end.
#[cfg(windows)]
pub(crate) type Pipe = std::os::windows::io::OwnedHandle;

/// The host's ends of the child's stdio.
#[derive(Debug)]
pub(crate) enum HostStdio {
    /// `stdin` only with `Stdin::Pipe`; `stderr` is `None` with `merge_stderr`.
    Pipes {
        stdin: Option<Pipe>,
        stdout: Pipe,
        stderr: Option<Pipe>,
    },
    /// Terminal output and input (on Unix both are the master, duplicated).
    Pty { output: Pipe, input: Pipe },
}

/// A started tree.
#[derive(Debug)]
pub(crate) struct Spawned {
    pub tree: Tree,
    pub pid: u32,
    pub stdio: HostStdio,
}

/// The confirmation of `Tree::stop`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Stopped {
    /// Something was still alive at the deadline and was force-killed.
    pub forced: bool,
    pub elapsed: Duration,
}

/// Starts `spec` through the supervisor (starting the supervisor first if needed). Blocks the calling
/// thread for one round trip (like `std::process::Command::spawn`), bounded: a stalled supervisor yields
/// `Io`, never a hang. Startup failures map to `NotFound` / `NotExecutable` / `InvalidCwd` / `Io`.
/// A Unix PTY root is held before exec until `Tree::go`.
pub(crate) fn spawn(spec: &Spec) -> Result<Spawned, Error> {
    let _ = spec;
    Err(todo())
}

/// One tree of one supervisor generation. Dropping it sends `Release` without blocking; the supervisor
/// keeps its cleanup duty (the tree is still stopped on host death).
#[derive(Debug)]
pub(crate) struct Tree {}

impl Tree {
    /// Unix PTY roots: lets the held root exec, once the host's reader runs. A no-op elsewhere.
    pub(crate) async fn go(&self) -> Result<(), Error> {
        Err(todo())
    }

    /// Resolves at root exit; every call returns the same value.
    pub(crate) async fn exited(&self) -> Result<RootExit, Error> {
        Err(todo())
    }

    /// One deadline for the whole tree: graceful now, forced at `grace`; resolves once the tree is gone
    /// (at once if it already is). Overlapping calls share the earliest deadline (ADR-0005 R2).
    pub(crate) async fn stop(&self, grace: Duration) -> Result<Stopped, Error> {
        let _ = grace;
        Err(todo())
    }

    /// `stop` without waiting for the confirmation (for `Drop`); never blocks.
    pub(crate) fn stop_detached(&self, grace: Duration) {
        let _ = grace;
    }

    /// Resizes the terminal; `Closed` after the root exited.
    pub(crate) fn resize(&self, cols: u16, rows: u16) -> Result<(), Error> {
        let _ = (cols, rows);
        Err(todo())
    }

    /// The live processes `stop` would end now; `[]` once gone; `Io` if the inventory is incomplete.
    pub(crate) async fn processes(&self) -> Result<Vec<ProcEntry>, Error> {
        Err(todo())
    }
}

fn todo() -> Error {
    Error::new(ErrorCode::Io, "not implemented yet (W04: client)")
}
