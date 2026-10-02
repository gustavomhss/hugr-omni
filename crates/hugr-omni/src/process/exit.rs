//! `Exit`, `Reason`, `RunOutput`, `ProcessInfo` (contract §1, §5, §7). W07.

use crate::io::Data;

/// Why the root process ended (contract §7). The cause is committed when the library acts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Reason {
    /// It exited on its own.
    Exit,
    /// A signal ended it, not sent by this library (Unix only).
    Signal,
    /// `stop()` or dropping the child ended it.
    Killed,
    /// The timeout ended it.
    Timeout,
    /// Cancellation ended it.
    Aborted,
}

/// How the root process ended.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Exit {
    /// The exit code; `None` only when a Unix process was ended by a signal. Windows codes are reported as
    /// non-negative integers (e.g. `0xC000013A`).
    pub code: Option<u32>,
    /// The Unix signal name (e.g. `"SIGTERM"`); always `None` on Windows.
    pub signal: Option<String>,
    /// Why it ended.
    pub reason: Reason,
}

impl Exit {
    /// `reason` is `Exit` and the code is 0.
    pub fn success(&self) -> bool {
        self.reason == Reason::Exit && self.code == Some(0)
    }
}

/// The result of `run()`: describes the whole run, not only the root (contract §6).
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct RunOutput {
    /// How the run ended.
    pub exit: Exit,
    /// Complete stdout (PTY output lands here).
    pub stdout: Data,
    /// Complete stderr; empty for a PTY and with `merge_stderr`.
    pub stderr: Data,
}

/// One live process of a child's tree (contract §5, `processes()`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct ProcessInfo {
    /// Process id.
    pub pid: u32,
    /// The parent's pid when the parent is in the same list; `None` otherwise (e.g. the root).
    pub parent_pid: Option<u32>,
    /// The executable's file name without directory (`node`, `node.exe`); `None` when the OS does not tell.
    pub name: Option<String>,
}
