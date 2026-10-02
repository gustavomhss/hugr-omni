//! Terminals on Unix, supervisor side (W12): the PTY pair and the child-side step; the fork, the hold
//! until `Go` and exec are the event loop's (`unix`, W05) (ADR-0003, ADR-0005 R9).
//!
//! SEAM (frozen in W00): `PtyPair`, `open`, `resize`, `make_controlling`. Bodies belong to W12.

use std::io;
use std::os::fd::{BorrowedFd, OwnedFd, RawFd};

/// A new terminal: the master (sent to the host, a dup kept for `Resize`) and the slave (for the child).
#[derive(Debug)]
pub(crate) struct PtyPair {
    pub master: OwnedFd,
    pub slave: OwnedFd,
}

/// `/dev/ptmx` with `O_CLOEXEC`, grant + unlock, slave via `ptsname_r` (Linux) / `TIOCPTYGNAME` (macOS),
/// never `ptsname`; size set before any child exists.
pub(crate) fn open(cols: u16, rows: u16) -> io::Result<PtyPair> {
    let _ = (cols, rows);
    Err(io::Error::other("not implemented yet (W12)"))
}

/// `TIOCSWINSZ` on the master.
pub(crate) fn resize(master: BorrowedFd<'_>, cols: u16, rows: u16) -> io::Result<()> {
    let _ = (master, cols, rows);
    Err(io::Error::other("not implemented yet (W12)"))
}

/// In the forked child, after `setsid`: makes `slave` the controlling terminal (`TIOCSCTTY`) and its
/// stdio. Returns the errno on failure.
///
/// # Safety
/// Only between `fork` and `exec` in the child; async-signal-safe calls only.
pub(crate) unsafe fn make_controlling(slave: RawFd) -> Result<(), i32> {
    let _ = slave;
    Err(0)
}
