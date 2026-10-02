//! ConPTY in the supervisor (W12w): flags 0, PTY-side ends closed after creation, explicit null standard
//! handles; `ClosePseudoConsole` on a worker thread with an independent deadline (ADR-0003, ADR-0005).
//! `CreateProcessW` with the `PSEUDOCONSOLE` attribute is the event loop's (`windows`, W06).
//!
//! SEAM (frozen in W00): `ConPty`, `ConPtyEnds`, `create` and the `ConPty` methods. Bodies belong to W12w.

use std::io;
use std::os::windows::io::OwnedHandle;
use std::time::Instant;

use windows_sys::Win32::System::Console::HPCON;

/// A pseudoconsole owned by the supervisor.
#[derive(Debug)]
pub(crate) struct ConPty {}

/// The host's ends, to be transferred with `DUPLICATE_CLOSE_SOURCE`.
#[derive(Debug)]
pub(crate) struct ConPtyEnds {
    /// What the terminal shows (host reads).
    pub output: OwnedHandle,
    /// What is typed (host writes).
    pub input: OwnedHandle,
}

/// Creates a pseudoconsole of `cols` x `rows`.
pub(crate) fn create(cols: u16, rows: u16) -> io::Result<(ConPty, ConPtyEnds)> {
    let _ = (cols, rows);
    Err(io::Error::other("not implemented yet (W12w)"))
}

impl ConPty {
    /// For the `PROC_THREAD_ATTRIBUTE_PSEUDOCONSOLE` attribute.
    pub(crate) fn handle(&self) -> HPCON {
        0
    }

    /// `ResizePseudoConsole`.
    pub(crate) fn resize(&self, cols: u16, rows: u16) -> io::Result<()> {
        let _ = (cols, rows);
        Err(io::Error::other("not implemented yet (W12w)"))
    }

    /// Closes on a worker thread; the caller enforces `deadline` independently (the close can block).
    pub(crate) fn close(self, deadline: Instant) {
        let _ = deadline;
    }
}
