//! The supervisor on Windows (W06): `CreateProcessW` + `JOB_LIST` (own Job, KILL_ON_JOB_CLOSE, no
//! breakaway), std-equivalent argument quoting, `.cmd`/`.bat` through `cmd.exe` or refused, CTRL_BREAK
//! from its own console, Job inventory, host death (ADR-0005 R1, R5, R7).
//!
//! SEAM (frozen in W00): `run`. Everything else belongs to W06.

use std::process::ExitCode;

use crate::Args;

/// Serves the host on the named pipe until the host is gone and every tree is gone.
pub(crate) fn run(args: &Args) -> ExitCode {
    let _ = args;
    eprintln!("hugr-omni-supervisor: not implemented yet (W06)");
    ExitCode::FAILURE
}
