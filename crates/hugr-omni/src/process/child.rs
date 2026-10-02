//! `Child`, `PipeChild`, `PtyChild` (contract §1, §5, §9, §10). W07.

use std::ops::Deref;
use std::time::Duration;

use super::{Exit, Options, ProcessInfo};
use crate::error::{Error, ErrorCode};
use crate::io::{Lines, Output};
use crate::spawn::{PtySize, Request};

fn todo() -> Error {
    Error::new(ErrorCode::Io, "not implemented yet (W07: process/child)")
}

/// Bytes dropped so far because nobody was reading (contract §4). PTY output counts as stdout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DroppedBytes {
    /// stdout (and PTY output).
    pub stdout: u64,
    /// stderr.
    pub stderr: u64,
}

/// A running process and its tree. What pipe and PTY children share.
///
/// Dropping it force-kills the tree at once without blocking; call `stop()` for a graceful end.
#[derive(Debug)]
pub struct Child {
    pid: u32,
}

impl Child {
    /// The root process id.
    pub fn pid(&self) -> u32 {
        self.pid
    }

    /// Claims the single output consumer as chunks (contract §4). A second claim, or a claim after the
    /// consumer was dropped, fails with `InvalidArgument`. Dropping the `Output` detaches for good.
    pub fn output(&self) -> Result<Output, Error> {
        Err(todo())
    }

    /// Claims the single output consumer as lines (contract §4).
    pub fn lines(&self) -> Result<Lines, Error> {
        Err(todo())
    }

    /// Bytes dropped so far (contract §4).
    pub fn dropped_bytes(&self) -> DroppedBytes {
        DroppedBytes::default()
    }

    /// Writes to stdin or types into the terminal; resolves when the OS accepted the bytes (contract §9).
    pub async fn write(&self, data: impl AsRef<[u8]>) -> Result<(), Error> {
        let _ = data;
        Err(todo())
    }

    /// Resolves when the root process exits; every call returns the same `Exit` (contract §5).
    pub async fn wait(&self) -> Result<Exit, Error> {
        Err(todo())
    }

    /// Ends the whole tree with one deadline: graceful first, forced after `grace` (default: the command's).
    /// Resolves once the tree is gone; calling it again returns the same `Exit` (contract §5).
    pub async fn stop(&self, grace: Option<Duration>) -> Result<Exit, Error> {
        let _ = grace;
        Err(todo())
    }

    /// The live processes `stop()` would end right now; `[]` once the tree is gone (contract §5).
    pub async fn processes(&self) -> Result<Vec<ProcessInfo>, Error> {
        Err(todo())
    }
}

/// A child with pipes (`Command::spawn`).
#[derive(Debug)]
pub struct PipeChild {
    child: Child,
}

impl PipeChild {
    /// Waits for queued writes, then closes stdin; idempotent (contract §9).
    pub async fn close_stdin(&self) -> Result<(), Error> {
        Err(todo())
    }
}

impl Deref for PipeChild {
    type Target = Child;
    fn deref(&self) -> &Child {
        &self.child
    }
}

/// A child inside a terminal (`Command::spawn_pty`).
#[derive(Debug)]
pub struct PtyChild {
    child: Child,
}

impl PtyChild {
    /// Resizes the terminal; `Closed` after exit, `InvalidArgument` for a zero size (contract §10).
    pub fn resize(&self, size: PtySize) -> Result<(), Error> {
        let _ = size;
        Err(todo())
    }
}

impl Deref for PtyChild {
    type Target = Child;
    fn deref(&self) -> &Child {
        &self.child
    }
}

/// Starts a pipe child (`InvalidArgument` if `req.mode` is a terminal).
pub(crate) fn spawn_pipe(req: &Request, opts: &Options) -> Result<PipeChild, Error> {
    let _ = (req, opts);
    Err(todo())
}

/// Starts a terminal child.
pub(crate) fn spawn_pty(req: &Request, opts: &Options) -> Result<PtyChild, Error> {
    let _ = (req, opts);
    Err(todo())
}
