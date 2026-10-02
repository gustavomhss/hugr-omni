//! The supervisor on Linux and macOS (W05): one thread, one event loop; `posix_spawn` + SETSID for pipe
//! roots, `fork` for PTY roots (via `pty_unix`), sessions as the kill unit, the process inventory,
//! pidfd/kqueue, reaping and pinning, `Stop`, host death (ADR-0005 R1, R3, R4, R5, R8, R10).
//!
//! SEAM (frozen in W00): `run`. Everything else belongs to W05.

use std::process::ExitCode;

use crate::Args;

/// Serves the host on fd 0 until the host is gone and every tree is gone.
pub(crate) fn run(args: &Args) -> ExitCode {
    let _ = args;
    eprintln!("hugr-omni-supervisor: not implemented yet (W05)");
    ExitCode::FAILURE
}
